//! # Desktop - Desktop Environment
//!
//! Gerencia o ambiente de desktop do Shell.
//!
//! ## v0.2.0 Melhorias
//!
//! - Menu iniciar lista apps de /apps/
//! - Clique no menu lança aplicativos
//! - Taskbar mostra janelas abertas
//! - Relógio mostra uptime real

use crate::app::{discover_apps, launch_app, AppInfo};
use crate::ui::taskbar::TaskbarAction;
use crate::ui::{Taskbar, Wallpaper};
use alloc::string::ToString;
use redpowder::ipc::Port;
use redpowder::println;
use redpowder::syscall::SysResult;
use redpowder::window::{
    opcodes, RegisterTaskbarRequest, Window, WindowLifecycleEvent, COMPOSITOR_PORT,
};

// ============================================================================
// CONSTANTES
// ============================================================================

const LISTENER_PORT_NAME: &str = "shell.taskbar";

// ============================================================================
// DESKTOP
// ============================================================================

pub struct Desktop {
    /// Janela principal do desktop
    window: Window,
    /// Porta para eventos de lifecycle
    listener_port: Port,
    /// Janela do menu iniciar
    menu_window: Option<Window>,
    /// Componente de wallpaper
    wallpaper: Wallpaper,
    /// Componente de taskbar
    taskbar: Taskbar,
    /// Apps descobertos
    available_apps: Vec<AppInfo>,
    /// Flag de dirty
    dirty: bool,
    /// Contador de frames para atualização do relógio
    frame_counter: u32,
}

impl Desktop {
    /// Cria e inicializa o desktop.
    pub fn new() -> SysResult<Self> {
        println!("[Shell] Iniciando v0.2.0...");

        // Obter resolução
        let fb_info = redpowder::graphics::get_framebuffer_info()?;
        let screen_width = fb_info.width;
        let screen_height = fb_info.height;

        println!("[Shell] Resolução: {}x{}", screen_width, screen_height);

        // Criar janela fullscreen
        let window = Window::create_with_flags(0, 0, screen_width, screen_height, 0x08, "Shell")?;
        println!("[Shell] Desktop window criada");

        // Criar componentes
        let mut taskbar = Taskbar::new(screen_width, screen_height);
        let wallpaper = Wallpaper::with_bounds(0, 0, screen_width, screen_height);

        // Descobrir apps disponíveis
        println!("[Shell] Descobrindo apps...");
        let available_apps = discover_apps();
        println!("[Shell] {} apps encontrados", available_apps.len());
        for app in &available_apps {
            println!("[Shell]   - {} ({})", app.name, app.path);
        }

        // Passar apps para taskbar
        taskbar.set_available_apps(available_apps.clone());

        // Criar porta listener
        let listener_port = Port::create(LISTENER_PORT_NAME, 4096)?;
        println!("[Shell] Listener port criada");

        // Registrar como taskbar no compositor
        Self::register_with_compositor();

        Ok(Self {
            window,
            listener_port,
            menu_window: None,
            wallpaper,
            taskbar,
            available_apps,
            dirty: true,
            frame_counter: 0,
        })
    }

    /// Registra o shell como taskbar no compositor
    fn register_with_compositor() {
        match Port::connect(COMPOSITOR_PORT) {
            Ok(compositor) => {
                let mut port_name_buf = [0u8; 32];
                let bytes = LISTENER_PORT_NAME.as_bytes();
                let len = bytes.len().min(32);
                port_name_buf[..len].copy_from_slice(&bytes[..len]);

                let req = RegisterTaskbarRequest {
                    op: opcodes::REGISTER_TASKBAR,
                    listener_port: port_name_buf,
                };

                let req_bytes = unsafe {
                    core::slice::from_raw_parts(
                        &req as *const _ as *const u8,
                        core::mem::size_of::<RegisterTaskbarRequest>(),
                    )
                };

                let _ = compositor.send(req_bytes, 0);
                println!("[Shell] Registrado como taskbar");
            }
            Err(e) => println!("[Shell] Erro ao registrar: {:?}", e),
        }
    }

    /// Executa o loop principal
    pub fn run(&mut self) -> ! {
        println!("[Shell] Desktop Environment iniciado!");

        // Renderização inicial
        self.redraw();

        println!("[Shell] Pronto!");
        println!("[Shell] Entrando no loop principal...");

        let mut last_heartbeat = 0u64;
        let mut loop_count = 0u64;

        loop {
            loop_count += 1;

            // Log na primeira iteração
            if loop_count == 1 {
                println!("[Shell] Loop: primeira iteração");
            }

            // Heartbeat a cada ~60 iterações (~1 segundo)
            if loop_count % 60 == 0 {
                let uptime_secs = redpowder::time::clock().unwrap_or(0) / 1000;
                if uptime_secs != last_heartbeat {
                    last_heartbeat = uptime_secs;
                    println!(
                        "[Shell] Heartbeat: uptime={}s, loops={}",
                        uptime_secs, loop_count
                    );

                    // Forçar atualização do relógio
                    self.taskbar.update_uptime();
                    self.dirty = true;
                }
            }

            // Log antes de process_events na primeira iteração
            if loop_count == 1 {
                println!("[Shell] Loop: chamando process_events()");
            }

            // Processar eventos
            self.process_events();

            if loop_count == 1 {
                println!("[Shell] Loop: process_events() retornou");
            }

            // Incrementar frame counter
            self.frame_counter += 1;

            // Atualizar relógio a cada ~30 frames (~500ms)
            if self.frame_counter >= 30 {
                self.frame_counter = 0;
                self.taskbar.update_uptime();
                self.dirty = true; // Forçar redraw para atualizar relógio
            }

            // Redesenhar se necessário
            if self.dirty {
                if loop_count == 1 {
                    println!("[Shell] Loop: chamando redraw()");
                }
                self.redraw();
                self.dirty = false;
                if loop_count == 1 {
                    println!("[Shell] Loop: redraw() retornou");
                }
            }

            if loop_count == 1 {
                println!("[Shell] Loop: chamando sleep(16)");
            }

            // Throttle - sleep 16ms (~60fps)
            let _ = redpowder::time::sleep(16);

            if loop_count == 1 {
                println!("[Shell] Loop: sleep() retornou, fim da primeira iteração");
            }
        }
    }

    /// Processa eventos pendentes
    fn process_events(&mut self) {
        let mut toggle_menu = false;
        let mut close_menu = false;
        let mut launch_app_index: Option<usize> = None;

        // DEBUG: Log antes de poll_events
        if self.frame_counter == 0 {
            println!("[Shell] process_events: antes de poll_events()");
        }

        // 1. Eventos da janela principal
        let mut event_count = 0;
        for event in self.window.poll_events() {
            event_count += 1;
            if event_count == 1 && self.frame_counter == 0 {
                println!("[Shell] process_events: primeiro evento recebido");
            }
            match event {
                redpowder::event::Event::Input(input) => {
                    println!("[Shell] Input event: type={}", input.event_type);
                    if input.event_type == redpowder::event::event_type::MOUSE_DOWN {
                        let x = input.param1 as i32;
                        let y = (input.param2 >> 16) as i32;
                        println!("[Shell] Mouse click at ({}, {})", x, y);

                        match self.taskbar.handle_click(x, y) {
                            TaskbarAction::ToggleStartMenu => {
                                println!("[Shell] Action: ToggleStartMenu");
                                toggle_menu = true;
                            }
                            TaskbarAction::ToggleWindow(id) => {
                                println!("[Shell] Action: ToggleWindow({})", id);
                                self.toggle_window(id);
                            }
                            TaskbarAction::LaunchApp(idx) => {
                                println!("[Shell] Action: LaunchApp({})", idx);
                                launch_app_index = Some(idx);
                            }
                            TaskbarAction::None => {
                                // Clique fora? Fechar menu
                                if self.taskbar.start_menu_open && y < self.taskbar.y as i32 {
                                    close_menu = true;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // DEBUG: Log após poll_events
        if self.frame_counter == 0 {
            println!(
                "[Shell] process_events: após poll_events(), {} eventos",
                event_count
            );
        }

        // 2. Eventos da porta de lifecycle
        let mut msg_buf = [0u8; 256];
        loop {
            match self.listener_port.recv(&mut msg_buf, 0) {
                Ok(n) if n > 0 => {
                    if n >= core::mem::size_of::<WindowLifecycleEvent>() {
                        let evt = unsafe { &*(msg_buf.as_ptr() as *const WindowLifecycleEvent) };
                        if evt.op == opcodes::EVENT_WINDOW_LIFECYCLE {
                            self.handle_lifecycle_event(evt);
                        }
                    }
                }
                _ => break, // n == 0 ou Err -> sair do loop
            }
        }

        // 3. Eventos do menu (se aberto)
        if let Some(ref mut menu_win) = self.menu_window {
            for event in menu_win.poll_events() {
                match event {
                    redpowder::event::Event::Input(input) => {
                        if input.event_type == redpowder::event::event_type::MOUSE_DOWN {
                            let x = input.param1 as i32;
                            let y = (input.param2 >> 16) as i32;

                            match self.taskbar.handle_menu_click(x, y, menu_win.height) {
                                TaskbarAction::LaunchApp(idx) => {
                                    launch_app_index = Some(idx);
                                    close_menu = true;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Aplicar ações
        if toggle_menu {
            self.dirty = true;
            self.update_menu_state();
        } else if close_menu {
            self.taskbar.start_menu_open = false;
            self.update_menu_state();
            self.dirty = true;
        }

        // Lançar app se solicitado
        if let Some(idx) = launch_app_index {
            self.launch_app_by_index(idx);
        }
    }

    /// Trata eventos de lifecycle de janela
    fn handle_lifecycle_event(&mut self, evt: &WindowLifecycleEvent) {
        let title_len = evt
            .title
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(evt.title.len());
        let title = core::str::from_utf8(&evt.title[..title_len])
            .unwrap_or("?")
            .to_string();

        match evt.event_type {
            redpowder::window::lifecycle_events::CREATED => {
                println!("[Shell] Janela criada: {} (ID {})", title, evt.window_id);
                self.taskbar.add_window(evt.window_id, title);
            }
            redpowder::window::lifecycle_events::DESTROYED => {
                println!("[Shell] Janela destruída: ID {}", evt.window_id);
                self.taskbar.remove_window(evt.window_id);
            }
            redpowder::window::lifecycle_events::MINIMIZED => {
                self.taskbar.set_window_minimized(evt.window_id, true);
            }
            redpowder::window::lifecycle_events::RESTORED => {
                self.taskbar.set_window_minimized(evt.window_id, false);
            }
            _ => {}
        }
        self.dirty = true;
    }

    /// Toggle minimizar/restaurar janela
    fn toggle_window(&self, id: u32) {
        let is_minimized = self.taskbar.get_window_state(id).unwrap_or(false);

        if let Ok(compositor) = Port::connect(COMPOSITOR_PORT) {
            let op = if is_minimized {
                opcodes::RESTORE_WINDOW
            } else {
                opcodes::MINIMIZE_WINDOW
            };

            let req = redpowder::window::DestroyWindowRequest { op, window_id: id };

            let bytes = unsafe {
                core::slice::from_raw_parts(
                    &req as *const _ as *const u8,
                    core::mem::size_of::<redpowder::window::DestroyWindowRequest>(),
                )
            };
            let _ = compositor.send(bytes, 0);
        }
    }

    /// Lança um app pelo índice
    fn launch_app_by_index(&mut self, idx: usize) {
        if let Some(app) = self.available_apps.get(idx) {
            println!("[Shell] Lançando app: {}", app.name);
            let path = app.path.clone();
            let _ = launch_app(&path);
        }
    }

    /// Atualiza estado da janela do menu
    fn update_menu_state(&mut self) {
        if self.taskbar.start_menu_open && self.menu_window.is_none() {
            // Abrir menu
            let menu_h: u32 = 320;
            let menu_w: u32 = 220;
            let screen_h = self.window.height;

            println!("[Shell] Abrindo menu iniciar...");
            match Window::create_with_flags(
                0,
                screen_h - 40 - menu_h,
                menu_w,
                menu_h,
                0x01, // Overlay
                "Start Menu",
            ) {
                Ok(w) => self.menu_window = Some(w),
                Err(e) => println!("[Shell] Erro ao criar menu: {:?}", e),
            }
        } else if !self.taskbar.start_menu_open && self.menu_window.is_some() {
            // Fechar menu
            println!("[Shell] Fechando menu iniciar");
            if let Some(menu) = self.menu_window.take() {
                let _ = menu.destroy();
            }
        }
    }

    /// Redesenha o desktop
    fn redraw(&mut self) {
        // Desenhar wallpaper e taskbar na janela principal
        self.wallpaper.draw(&mut self.window);
        self.taskbar.draw(&mut self.window);
        let _ = self.window.present();

        // Desenhar conteúdo do menu
        if let Some(ref mut menu_win) = self.menu_window {
            self.taskbar.draw_menu_content(menu_win);
            let _ = menu_win.present();
        }
    }
}

// Uso do Vec
use alloc::vec::Vec;
