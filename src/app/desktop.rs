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

use crate::ui::{Taskbar, Wallpaper};
use redpowder::println;
use redpowder::syscall::SysResult;
use redpowder::window::Window;

// ============================================================================
// CONSTANTES
// ============================================================================

/// Intervalo de idle entre frames (ms) - reservado para uso futuro
#[allow(dead_code)]
const IDLE_INTERVAL_MS: u64 = 16;

// ============================================================================
// DESKTOP
// ============================================================================

/// Desktop Environment principal.
pub struct Desktop {
    /// Janela principal do desktop
    window: Window,
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

        // Criar janela fullscreen com retry
        let window = Self::create_window_with_retry(screen_width, screen_height)?;

        println!("[Shell] Desktop inicializado com sucesso!");

        // Criar componentes
        let taskbar = Taskbar::new(screen_width, screen_height);
        let work_area = taskbar.get_work_area(screen_width);
        let wallpaper = Wallpaper::with_bounds(work_area.0, work_area.1, work_area.2, work_area.3);

        Ok(Self {
            window,
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
            match Window::create(0, 0, width, height) {
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
        for event in self.window.poll_events() {
            match event {
                redpowder::event::Event::Resize(resize) => {
                    println!(
                        "[Shell] Evento de resize: {}x{}",
                        resize.width, resize.height
                    );
                    self.dirty = true;
                }
                redpowder::event::Event::Input(input) => {
                    // Processar eventos de mouse
                    if input.event_type == redpowder::event::event_type::MOUSE_DOWN {
                        // Extrair coordenadas
                        let click_x = input.param1 as i32;
                        let click_y = (input.param2 >> 16) as i32;

                        println!("[Shell] Click at ({}, {})", click_x, click_y);

                        // Processar clique na taskbar
                        if self.taskbar.handle_click(click_x, click_y) {
                            self.dirty = true; // Redesenhar se algo mudou
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Redesenha todo o desktop.
    fn redraw(&mut self) {
        // 1. Desenhar wallpaper
        self.wallpaper.draw(&mut self.window);

        // 2. Desenhar taskbar
        self.taskbar.draw(&mut self.window);

        // 3. Enviar para o compositor
        match self.window.present() {
            Ok(_) => println!(
                "[Shell] Frame enviado ao compositor (janela {})",
                self.window.id
            ),
            Err(e) => println!("[Shell] Erro ao apresentar frame: {:?}", e),
        }
    }
}
