//! # Firefly Shell - Desktop Environment
//!
//! Processo responsável pela interface do usuário (Shell).
//! Gerencia: Taskbar, Wallpaper, Menu Iniciar, Tray.
//!
//! Atua como um cliente especial do Compositor.

#![no_std]
#![no_main]

extern crate alloc;

mod taskbar;

use core::panic::PanicInfo;
use redpowder::graphics::get_framebuffer_info;
use redpowder::println;
use redpowder::window::Window;
use taskbar::Taskbar;

#[global_allocator]
static ALLOCATOR: redpowder::mem::heap::SyscallAllocator = redpowder::mem::heap::SyscallAllocator;

// ============================================================================
// CONSTANTES
// ============================================================================

/// Cor de fundo do desktop (Azul Profundo - Windows-like)
const WALLPAPER_COLOR: u32 = 0xFF0078D7; // ARGB

// ============================================================================
// SHELL
// ============================================================================

struct DesktopShell {
    window: Window,
    taskbar: Taskbar,
}

impl DesktopShell {
    fn new() -> Result<Self, ()> {
        // 1. Obter tamanho da tela
        let info = get_framebuffer_info().map_err(|_| ())?;
        let screen_w = info.width;
        let screen_h = info.height;

        println!("[Shell] Screen resolution: {}x{}", screen_w, screen_h);

        // 2. Criar Janela Fullscreen (Desktop Layer)
        // No futuro, isso seria uma layer especial (Desktop).
        // Por enquanto é uma janela normal que cobre tudo.
        let window = Window::create(0, 0, screen_w, screen_h).map_err(|_| ())?;

        let taskbar = Taskbar::new(screen_w, screen_h);

        Ok(Self { window, taskbar })
    }

    fn run(&mut self) -> ! {
        println!("[Shell] Starting Desktop Environment...");

        // 1. Desenhar Wallpaper e Taskbar inicial
        self.redraw();

        println!("[Shell] Desktop ready!");

        // Loop de eventos
        loop {
            // Poll eventos
            let dirty = false;

            for event in self.window.poll_events() {
                match event {
                    redpowder::event::Event::Input(_input) => {
                        // TODO: Processar cliques na taskbar / menu iniciar
                    }
                    redpowder::event::Event::Resize(resize) => {
                        println!("[Shell] Resize event: {}x{}", resize.width, resize.height);
                        // TODO: Reajustar layout se compositor mudar resolução
                    }
                    _ => {}
                }
            }

            if dirty {
                self.redraw();
            } else {
                // Dormir para economizar CPU
                redpowder::time::sleep(100).unwrap();
            }
        }
    }

    fn redraw(&mut self) {
        // 1. Wallpaper
        // A taskbar sabe qual a área de trabalho
        let (x, y, w, h) = self
            .taskbar
            .get_work_area(self.window.width, self.window.height);

        self.window.fill_rect(x, y, w, h, WALLPAPER_COLOR);

        // 2. Taskbar
        self.taskbar.draw(&mut self.window);

        // 3. Present
        if let Err(e) = self.window.present() {
            println!("[Shell] Failed to present: {:?}", e);
        }
    }
}

// ============================================================================
// MAIN
// ============================================================================

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    // Pequeno delay para garantir que Compositor iniciou
    redpowder::time::sleep(500).ok();

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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
