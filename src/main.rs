//! # Firefly Shell - Desktop Environment
//!
//! Shell gráfico do RedstoneOS, responsável pela interface do usuário.
//!
//! ## Responsabilidades
//!
//! - Renderizar wallpaper (fundo do desktop)
//! - Renderizar taskbar (barra de tarefas)
//! - Gerenciar menu iniciar (futuro)
//! - System tray (futuro)
//!
//! ## Arquitetura
//!
//! O Shell é um cliente do Firefly Compositor. Ele cria uma janela
//! fullscreen e desenha nela usando a API `Window` do redpowder.
//! Não acessa o framebuffer diretamente.
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │           Firefly Shell             │
//! ├─────────────────────────────────────┤
//! │  Desktop   │  Taskbar   │  Launcher │
//! │  (Background) (Barra)    (Menu)     │
//! └─────────────────────────────────────┘
//!              │
//!              ▼ (IPC)
//!     ┌─────────────────┐
//!     │ Firefly Compositor│
//!     └─────────────────┘
//! ```

#![no_std]
#![no_main]

extern crate alloc;

mod font;
mod taskbar;

use core::panic::PanicInfo;
use redpowder::graphics::get_framebuffer_info;
use redpowder::println;
use redpowder::window::Window;
use taskbar::Taskbar;

// ============================================================================
// ALOCADOR
// ============================================================================

/// Alocador global usando syscalls do kernel.
#[global_allocator]
static ALLOCATOR: redpowder::mem::heap::SyscallAllocator = redpowder::mem::heap::SyscallAllocator;

// ============================================================================
// CONSTANTES
// ============================================================================

/// Cor de fundo do desktop (azul profundo - estilo Windows)
const WALLPAPER_COLOR: u32 = 0xFF0078D7;

/// Número máximo de tentativas para conectar ao compositor
const MAX_COMPOSITOR_RETRIES: u32 = 10;

/// Intervalo entre tentativas de conexão (ms)
const RETRY_INTERVAL_MS: u64 = 500;

/// Intervalo do loop de eventos quando inativo (ms)
const IDLE_INTERVAL_MS: u64 = 100;

// ============================================================================
// DESKTOP SHELL
// ============================================================================

/// Shell principal do desktop.
///
/// Gerencia a janela fullscreen, taskbar e interações do usuário.
struct DesktopShell {
    /// Janela principal (fullscreen)
    window: Window,

    /// Barra de tarefas
    taskbar: Taskbar,

    /// Flag indicando necessidade de redesenho
    dirty: bool,
}

impl DesktopShell {
    /// Cria e inicializa o shell do desktop.
    ///
    /// Tenta conectar ao compositor com retry automático.
    fn new() -> Result<Self, ()> {
        // 1. Obter resolução da tela
        let info = get_framebuffer_info().map_err(|_| {
            println!("[Shell] Erro: não foi possível obter info do framebuffer");
        })?;

        let screen_w = info.width;
        let screen_h = info.height;

        println!("[Shell] Resolução da tela: {}x{}", screen_w, screen_h);

        // 2. Criar janela fullscreen (com retry)
        let window = Self::create_window_with_retry(screen_w, screen_h)?;

        // 3. Inicializar taskbar
        let taskbar = Taskbar::new(screen_w, screen_h);

        println!("[Shell] Desktop inicializado com sucesso!");

        Ok(Self {
            window,
            taskbar,
            dirty: true,
        })
    }

    /// Tenta criar janela com múltiplas tentativas.
    fn create_window_with_retry(width: u32, height: u32) -> Result<Window, ()> {
        for attempt in 1..=MAX_COMPOSITOR_RETRIES {
            match Window::create(0, 0, width, height) {
                Ok(window) => {
                    println!("[Shell] Conectado ao compositor (tentativa {})", attempt);
                    return Ok(window);
                }
                Err(_) => {
                    println!(
                        "[Shell] Aguardando compositor... ({}/{})",
                        attempt, MAX_COMPOSITOR_RETRIES
                    );
                    let _ = redpowder::time::sleep(RETRY_INTERVAL_MS);
                }
            }
        }

        println!("[Shell] Erro: compositor não disponível");
        Err(())
    }

    /// Executa o loop principal do shell.
    ///
    /// Esta função nunca retorna.
    fn run(&mut self) -> ! {
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
            let _ = redpowder::time::sleep(IDLE_INTERVAL_MS);
        }
    }

    /// Processa eventos pendentes da janela.
    fn process_events(&mut self) {
        for event in self.window.poll_events() {
            match event {
                redpowder::event::Event::Input(_input) => {
                    // TODO: Processar cliques na taskbar
                }
                redpowder::event::Event::Resize(resize) => {
                    println!(
                        "[Shell] Evento de resize: {}x{}",
                        resize.width, resize.height
                    );
                    // TODO: Ajustar layout
                    self.dirty = true;
                }
                _ => {}
            }
        }
    }

    /// Redesenha todo o desktop.
    fn redraw(&mut self) {
        // 1. Desenhar wallpaper (área de trabalho)
        let (x, y, w, h) = self
            .taskbar
            .get_work_area(self.window.width /*, self.window.height*/);
        self.window.fill_rect(x, y, w, h, WALLPAPER_COLOR);

        // 2. Desenhar taskbar
        self.taskbar.draw(&mut self.window);

        // 3. Enviar para o compositor
        if let Err(e) = self.window.present() {
            println!("[Shell] Erro ao apresentar frame: {:?}", e);
        }
    }
}

// ============================================================================
// PONTO DE ENTRADA
// ============================================================================

/// Ponto de entrada do shell.
#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    println!("[Shell] Iniciando...");

    // Pequeno delay para garantir que o compositor iniciou
    let _ = redpowder::time::sleep(500);

    match DesktopShell::new() {
        Ok(mut shell) => {
            shell.run();
        }
        Err(_) => {
            println!("[Shell] ERRO FATAL: Falha ao iniciar Desktop");
            loop {}
        }
    }
}

// ============================================================================
// PANIC HANDLER
// ============================================================================

/// Handler de panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[Shell] PANIC: {:?}", info);
    loop {}
}
