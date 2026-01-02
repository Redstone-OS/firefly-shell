//! # Desktop - Desktop Environment
//!
//! Gerencia o ambiente de desktop do Shell.
//!
//! ## Responsabilidades
//!
//! - Gerenciar janela principal do desktop
//! - Coordenar componentes visuais (wallpaper, taskbar)
//! - Processar eventos de entrada
//! - Loop principal de renderização

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

/// Intervalo de idle entre frames (ms) - reservado para uso futuro
#[allow(dead_code)]
const IDLE_INTERVAL_MS: u64 = 16;
const LISTENER_PORT_NAME: &str = "shell.taskbar";

// ============================================================================
// DESKTOP
// ============================================================================

pub struct Desktop {
    /// Janela principal do desktop (Wallpaper + Taskbar)
    window: Window,
    /// Porta para receber eventos globais (lifecycle)
    listener_port: Port,
    /// Janela do menu iniciar (aberta sob demanda)
    menu_window: Option<Window>,
    /// Componente de wallpaper
    wallpaper: Wallpaper,
    /// Componente de taskbar
    taskbar: Taskbar,
    /// Flag de dirty (precisa redesenhar)
    dirty: bool,
}

impl Desktop {
    /// Cria e inicializa o desktop.
    pub fn new() -> SysResult<Self> {
        println!("[Shell] Iniciando...");

        // Obter resolução via framebuffer
        let fb_info = redpowder::graphics::get_framebuffer_info()?;
        let screen_width = fb_info.width;
        let screen_height = fb_info.height;

        println!(
            "[Shell] Resolução da tela: {}x{}",
            screen_width, screen_height
        );

        // Criar janela única com flag 0x08 (FULLSCREEN/Background)
        let window = Window::create_with_flags(0, 0, screen_width, screen_height, 0x08, "Shell")?;

        println!("[Shell] Desktop inicializado com sucesso!");

        // Criar componentes
        let taskbar = Taskbar::new(screen_width, screen_height);
        let wallpaper = Wallpaper::with_bounds(0, 0, screen_width, screen_height);

        // Criar porta listener para eventos
        let listener_port = Port::create(LISTENER_PORT_NAME, 4096)?;
        println!("[Shell] Listener port criada: '{}'", LISTENER_PORT_NAME);

        // Registrar como taskbar no compositor
        // Usamos uma porta temporária para enviar ao compositor
        // TODO: encapsular isso melhor se possível
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
                println!("[Shell] Solicitacao de registro enviada");
            }
            Err(e) => println!("[Shell] Erro ao conectar compositor: {:?}", e),
        }

        Ok(Self {
            window,
            listener_port,
            menu_window: None,
            wallpaper,
            taskbar,
            dirty: true,
        })
    }

    /// Cria janela com retry para aguardar compositor.
    fn create_window_with_retry(width: u32, height: u32) -> SysResult<Window> {
        const MAX_RETRIES: u32 = 10;
        const RETRY_DELAY_MS: u64 = 100;

        for attempt in 1..=MAX_RETRIES {
            // API: Window::create(x, y, width, height)
            match Window::create(0, 0, width, height, "Shell (Retry)") {
                Ok(w) => {
                    println!("[Shell] Conectado ao compositor (tentativa {})", attempt);
                    return Ok(w);
                }
                Err(_) if attempt < MAX_RETRIES => {
                    let _ = redpowder::time::sleep(RETRY_DELAY_MS);
                }
                Err(e) => return Err(e),
            }
        }

        Err(redpowder::syscall::SysError::NotFound)
    }

    /// Executa o loop principal do shell.
    pub fn run(&mut self) -> ! {
        println!("[Shell] Iniciando Desktop Environment...");

        // Renderização inicial
        self.redraw();

        println!("[Shell] Desktop pronto!");

        // Loop de eventos
        loop {
            // Processar eventos da janela
            self.process_events();

            // Redesenhar se necessário
            if self.dirty {
                self.redraw();
                self.dirty = false;
            }

            // Sleep para não consumir 100% CPU
            let _ = redpowder::time::sleep(50);
        }
    }

    /// Processa eventos pendentes da janela.
    fn process_events(&mut self) {
        // 1. Processar eventos da janela principal
        let mut toggle_requested = false;
        let mut close_requested = false;

        for event in self.window.poll_events() {
            match event {
                redpowder::event::Event::Input(input) => {
                    if input.event_type == redpowder::event::event_type::MOUSE_DOWN {
                        let click_x = input.param1 as i32;
                        let click_y = (input.param2 >> 16) as i32;

                        // Se clicar na taskbar (parte inferior)
                        match self.taskbar.handle_click(click_x, click_y) {
                            TaskbarAction::ToggleStartMenu => {
                                toggle_requested = true;
                            }
                            TaskbarAction::ToggleWindow(id) => {
                                // Enviar comando de Toggle para a janela
                                println!("[Shell] Toggle Window {}", id);

                                // Determinar estado atual e inverter
                                let is_minimized =
                                    self.taskbar.get_window_state(id).unwrap_or(false);

                                // Precisamos enviar MINIMIZE ou RESTORE.
                                // Usamos uma porta temporária para o Compositor.
                                // Idealmente a struct Window teria helper static
                                if let Ok(compositor) = Port::connect(COMPOSITOR_PORT) {
                                    let op = if is_minimized {
                                        opcodes::RESTORE_WINDOW
                                    } else {
                                        opcodes::MINIMIZE_WINDOW
                                    };

                                    // Reusando struct de DestroyWindow que tem apenas window_id
                                    // Ou criando a request na mão. Minimize/Restore usam protocol simples?
                                    // Compositor: handle_minimize_window usa o proprio ID ou struct?
                                    // Compositor check: if data.len() < sizeof(DestroyWindowRequest)
                                    // DestroyWindowRequest tem window_id. Vamos reusar.
                                    let req = redpowder::window::DestroyWindowRequest {
                                        op,
                                        window_id: id,
                                    };
                                    let bytes = unsafe {
                                        core::slice::from_raw_parts(
                                            &req as *const _ as *const u8,
                                            core::mem::size_of::<
                                                redpowder::window::DestroyWindowRequest,
                                            >(),
                                        )
                                    };
                                    let _ = compositor.send(bytes, 0);
                                }
                            }
                            TaskbarAction::None => {
                                // Se clicar fora do menu, fechar o menu se estiver aberto
                                if self.taskbar.start_menu_open {
                                    close_requested = true;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // 2. Processar eventos de Lifecycle (listener port)
        let mut msg_buf = [0u8; 256];
        while let Ok(n) = self.listener_port.recv(&mut msg_buf, 0) {
            if n >= core::mem::size_of::<WindowLifecycleEvent>() {
                let evt = unsafe { &*(msg_buf.as_ptr() as *const WindowLifecycleEvent) };
                if evt.op == opcodes::EVENT_WINDOW_LIFECYCLE {
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
                            self.taskbar.add_window(evt.window_id, title)
                        }
                        redpowder::window::lifecycle_events::DESTROYED => {
                            self.taskbar.remove_window(evt.window_id)
                        }
                        redpowder::window::lifecycle_events::MINIMIZED => {
                            self.taskbar.set_window_minimized(evt.window_id, true)
                        }
                        redpowder::window::lifecycle_events::RESTORED => {
                            self.taskbar.set_window_minimized(evt.window_id, false)
                        }
                        _ => {}
                    }
                    self.dirty = true;
                }
            }
        }

        if toggle_requested {
            self.dirty = true;
            self.update_menu_state();
        } else if close_requested {
            self.taskbar.start_menu_open = false;
            self.update_menu_state();
            self.dirty = true;
        }

        // 2. Processar eventos da janela do menu (se existir)
        if let Some(ref mut menu_win) = self.menu_window {
            for _ in menu_win.poll_events() {}
        }
    }

    /// Sincroniza a janela do menu com o estado do componente Taskbar.
    fn update_menu_state(&mut self) {
        if self.taskbar.start_menu_open && self.menu_window.is_none() {
            // Abrir janela de menu (Overlay layer = 0x01 BORDERLESS ou flag específica)
            // No nosso compositor, 0x01 mapeia para Panel/Overlay
            let menu_h: u32 = 300;
            let menu_w: u32 = 200;
            let screen_h = self.window.height;

            println!("[Shell] Abrindo janela de menu...");
            match Window::create_with_flags(
                0u32,
                (screen_h - 40 - menu_h) as u32,
                menu_w,
                menu_h,
                0x01u32,
                "Start Menu",
            ) {
                Ok(w) => self.menu_window = Some(w),
                Err(e) => println!("[Shell] Erro ao criar janela de menu: {:?}", e),
            }
        } else if !self.taskbar.start_menu_open && self.menu_window.is_some() {
            // Fechar janela de menu
            println!("[Shell] Fechando janela de menu.");
            self.menu_window = None; // O drop da Window deve enviar DESTROY_WINDOW no futuro ou tratamos manual
                                     // Por enquanto, o compositor remove janelas órfãs ou tratamos via IPC se necessário
        }
    }

    /// Redesenha todo o desktop.
    fn redraw(&mut self) {
        // 1. Desenhar fundo e barra na janela principal
        self.wallpaper.draw(&mut self.window);
        self.taskbar.draw(&mut self.window);
        let _ = self.window.present();

        // 2. Desenhar menu na janela dedicada (se aberta)
        if let Some(ref mut menu_win) = self.menu_window {
            self.taskbar.draw_menu_content(menu_win);
            let _ = menu_win.present();
        }
    }
}
