//! # Desktop
//!
//! Desktop Environment principal do Shell.

use alloc::string::ToString;
use alloc::vec::Vec;
use gfx_types::geometry::Size;
use gfx_types::window::WindowFlags;

use crate::app::{discover_apps, launch_app, AppInfo};
use crate::ui::panels::StartMenuAction;
use crate::ui::{
    Panel, QuickSettingsPanel, StartMenuPanel, Taskbar, TaskbarAction, Wallpaper, WidgetPanel,
};

use redpowder::event::event_type;
use redpowder::ipc::Port;
use redpowder::println;
use redpowder::syscall::SysResult;
use redpowder::window::{
    lifecycle_events, opcodes, RegisterTaskbarRequest, Window, WindowLifecycleEvent,
    WindowOpRequest, COMPOSITOR_PORT,
};

// =============================================================================
// CONSTANTES
// =============================================================================

const LISTENER_PORT_NAME: &str = "shell.taskbar";
const FRAME_INTERVAL_MS: u64 = 16;

// =============================================================================
// DESKTOP
// =============================================================================

/// Desktop Environment principal.
pub struct Desktop {
    /// Janela principal.
    window: Window,
    /// Porta para eventos de lifecycle.
    listener_port: Port,

    /// Tamanho da tela.
    screen_size: Size,

    // Componentes visuais
    /// Wallpaper.
    wallpaper: Wallpaper,
    /// Taskbar.
    taskbar: Taskbar,

    // Painéis
    /// Painel de widgets.
    widget_panel: WidgetPanel,
    /// Menu iniciar.
    start_menu: StartMenuPanel,
    /// Configurações rápidas.
    quick_settings: QuickSettingsPanel,

    /// Apps descobertos.
    available_apps: Vec<AppInfo>,

    /// Flag de dirty.
    dirty: bool,
    /// Contador de frames.
    frame_count: u64,
}

impl Desktop {
    /// Cria e inicializa o desktop.
    pub fn new() -> SysResult<Self> {
        println!("[Shell] Iniciando Firefly Shell v0.3.0...");

        // Obter resolução
        let fb_info = redpowder::graphics::get_info()?;
        let screen_width = fb_info.width;
        let screen_height = fb_info.height;
        let screen_size = Size::new(screen_width, screen_height);

        println!("[Shell] Resolução: {}x{}", screen_width, screen_height);

        // Criar janela fullscreen (flag BACKGROUND para desktop)
        let flags = WindowFlags::BACKGROUND;
        let window = Window::create_with_flags(0, 0, screen_width, screen_height, flags, "Shell")?;
        println!("[Shell] Janela desktop criada");

        // Criar componentes visuais
        let wallpaper = Wallpaper::new(screen_width, screen_height);
        let mut taskbar = Taskbar::new(screen_width, screen_height);

        // Criar painéis
        let widget_panel = WidgetPanel::new(screen_width, screen_height);
        let mut start_menu = StartMenuPanel::new(screen_width, screen_height);
        let quick_settings = QuickSettingsPanel::new(screen_width, screen_height);

        // Descobrir apps
        println!("[Shell] Descobrindo apps...");
        let available_apps = discover_apps();
        println!("[Shell] {} apps encontrados", available_apps.len());

        // Configurar componentes com apps
        taskbar.set_available_apps(available_apps.clone());
        start_menu.set_apps(available_apps.clone());

        // Criar porta listener
        let listener_port = Port::create(LISTENER_PORT_NAME, 4096)?;
        println!("[Shell] Porta listener criada");

        // Registrar como taskbar
        Self::register_with_compositor();

        Ok(Self {
            window,
            listener_port,
            screen_size,
            wallpaper,
            taskbar,
            widget_panel,
            start_menu,
            quick_settings,
            available_apps,
            dirty: true,
            frame_count: 0,
        })
    }

    /// Registra como taskbar no compositor.
    fn register_with_compositor() {
        if let Ok(compositor) = Port::connect(COMPOSITOR_PORT) {
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
    }

    /// Executa o loop principal.
    pub fn run(&mut self) -> ! {
        println!("[Shell] Desktop Environment iniciado!");

        // Renderização inicial
        self.redraw();

        println!("[Shell] Entrando no loop principal...");

        let mut msg_buf = [0u8; 256];
        let mut last_heartbeat = 0u64;

        loop {
            self.frame_count += 1;

            // Heartbeat periódico
            if let Ok(ms) = redpowder::time::clock() {
                if ms - last_heartbeat > 10000 {
                    println!("[Shell] Frame {}, dirty={}", self.frame_count, self.dirty);
                    last_heartbeat = ms;
                }
            }

            // Processar eventos do compositor
            self.process_lifecycle_events(&mut msg_buf);

            // Processar input
            self.process_input();

            // Atualizar animações
            let animating = self.update_animations();

            // Redesenhar se necessário
            if self.dirty || animating {
                self.redraw();
                self.dirty = false;
            }

            // Estabilizar framerate
            let _ = redpowder::time::sleep(FRAME_INTERVAL_MS);
        }
    }

    /// Processa eventos de lifecycle do compositor.
    fn process_lifecycle_events(&mut self, buf: &mut [u8; 256]) {
        while let Ok(size) = self.listener_port.recv(buf, 0) {
            if size < 4 {
                break;
            }

            let opcode = unsafe { *(buf.as_ptr() as *const u32) };

            if opcode == opcodes::EVENT_WINDOW_LIFECYCLE {
                let evt = unsafe { &*(buf.as_ptr() as *const WindowLifecycleEvent) };
                self.handle_lifecycle_event(evt);
            }
        }
    }

    /// Trata evento de lifecycle.
    fn handle_lifecycle_event(&mut self, evt: &WindowLifecycleEvent) {
        let title_len = evt
            .title
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(evt.title.len());
        let title = core::str::from_utf8(&evt.title[..title_len])
            .unwrap_or("")
            .to_string();

        match evt.event_type {
            x if x == lifecycle_events::CREATED => {
                if evt.window_id != self.window.id {
                    self.taskbar.add_window(evt.window_id, title);
                    self.dirty = true;
                }
            }
            x if x == lifecycle_events::DESTROYED => {
                self.taskbar.remove_window(evt.window_id);
                self.dirty = true;
            }
            x if x == lifecycle_events::MINIMIZED => {
                self.taskbar.set_window_minimized(evt.window_id, true);
                self.dirty = true;
            }
            x if x == lifecycle_events::RESTORED => {
                self.taskbar.set_window_minimized(evt.window_id, false);
                self.dirty = true;
            }
            _ => {}
        }
    }

    /// Processa input.
    fn process_input(&mut self) {
        // Coletar eventos primeiro para evitar borrow conflict
        let events: Vec<_> = self.window.poll_events().collect();

        for evt in events {
            match evt {
                redpowder::event::Event::Input(input) => {
                    if input.event_type == event_type::MOUSE_DOWN {
                        let mouse_x = input.param1 as i16 as i32;
                        let mouse_y = (input.param2 >> 16) as i16 as i32;
                        self.handle_click(mouse_x, mouse_y);
                    }
                }
                _ => {}
            }
        }
    }

    /// Trata clique.
    fn handle_click(&mut self, x: i32, y: i32) {
        // Verificar painéis primeiro (ordem de cima para baixo)
        if self.quick_settings.is_visible() && self.quick_settings.handle_click(x, y) {
            self.dirty = true;
            return;
        }

        if self.start_menu.is_visible() {
            if self.start_menu.handle_click(x, y) {
                // Verificar se há ação
                if let StartMenuAction::LaunchApp(path) = self.start_menu.take_action() {
                    launch_app(&path);
                }
                self.dirty = true;
                return;
            } else {
                // Clique fora do menu - fechar
                self.start_menu.set_visible(false);
                self.dirty = true;
            }
        }

        if self.widget_panel.is_visible() && self.widget_panel.handle_click(x, y) {
            self.dirty = true;
            return;
        }

        // Verificar taskbar
        if self.taskbar.contains_point(x, y) {
            let action = self.taskbar.handle_click(x, y);
            self.handle_taskbar_action(action);
            return;
        }

        // Clique no desktop - fechar todos os painéis
        self.close_all_panels();
    }

    /// Trata ação da taskbar.
    fn handle_taskbar_action(&mut self, action: TaskbarAction) {
        match action {
            TaskbarAction::ToggleWidgetPanel => {
                self.close_other_panels(Some(crate::ui::PanelType::Widget));
                self.widget_panel.toggle();
                self.dirty = true;
            }
            TaskbarAction::ToggleStartMenu => {
                self.close_other_panels(Some(crate::ui::PanelType::StartMenu));
                self.start_menu.toggle();
                self.dirty = true;
            }
            TaskbarAction::ToggleQuickSettings => {
                self.close_other_panels(Some(crate::ui::PanelType::QuickSettings));
                self.quick_settings.toggle();
                self.dirty = true;
            }
            TaskbarAction::ToggleWindow(id) => {
                self.toggle_window(id);
                self.dirty = true;
            }
            TaskbarAction::LaunchApp(idx) => {
                if idx < self.available_apps.len() {
                    launch_app(&self.available_apps[idx].path);
                }
            }
            TaskbarAction::None => {}
        }
    }

    /// Toggle de janela (minimize/restore).
    fn toggle_window(&mut self, window_id: u32) {
        if let Some(minimized) = self.taskbar.get_window_state(window_id) {
            if minimized {
                Self::send_window_op(window_id, opcodes::RESTORE_WINDOW);
            } else {
                Self::send_window_op(window_id, opcodes::MINIMIZE_WINDOW);
            }
        }
    }

    /// Envia operação de janela para o compositor.
    fn send_window_op(window_id: u32, op: u32) {
        if let Ok(compositor) = Port::connect(COMPOSITOR_PORT) {
            let req = WindowOpRequest { op, window_id };
            let bytes = unsafe {
                core::slice::from_raw_parts(
                    &req as *const _ as *const u8,
                    core::mem::size_of::<WindowOpRequest>(),
                )
            };
            let _ = compositor.send(bytes, 0);
        }
    }

    /// Fecha todos os painéis.
    fn close_all_panels(&mut self) {
        self.widget_panel.set_visible(false);
        self.start_menu.set_visible(false);
        self.quick_settings.set_visible(false);
        self.dirty = true;
    }

    /// Fecha outros painéis exceto o especificado.
    fn close_other_panels(&mut self, keep: Option<crate::ui::PanelType>) {
        if keep != Some(crate::ui::PanelType::Widget) {
            self.widget_panel.set_visible(false);
        }
        if keep != Some(crate::ui::PanelType::StartMenu) {
            self.start_menu.set_visible(false);
        }
        if keep != Some(crate::ui::PanelType::QuickSettings) {
            self.quick_settings.set_visible(false);
        }
    }

    /// Atualiza animações. Retorna true se ainda animando.
    fn update_animations(&mut self) -> bool {
        let a1 = self.widget_panel.update_animation();
        let a2 = self.start_menu.update_animation();
        let a3 = self.quick_settings.update_animation();
        a1 || a2 || a3
    }

    /// Redesenha tudo.
    fn redraw(&mut self) {
        // Obter buffer
        let buffer = self.window.buffer();
        let size = self.screen_size;

        // 1. Wallpaper
        self.wallpaper.draw(buffer, size);

        // 2. Painéis (se visíveis)
        self.widget_panel.draw(buffer, size);
        self.start_menu.draw(buffer, size);
        self.quick_settings.draw(buffer, size);

        // 3. Taskbar (sempre por cima)
        self.taskbar.draw(buffer, size);

        // 4. Present
        let _ = self.window.present();
    }
}
