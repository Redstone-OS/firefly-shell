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
use redpowder::graphics::{Color, Framebuffer};
use redpowder::println;
use taskbar::Taskbar;

#[global_allocator]
static ALLOCATOR: redpowder::mem::heap::SyscallAllocator = redpowder::mem::heap::SyscallAllocator;

// ============================================================================
// CONSTANTES
// ============================================================================

/// Cor de fundo do desktop (Azul Profundo - Windows-like)
const WALLPAPER_COLOR: Color = Color::rgb(0, 120, 215);

// ============================================================================
// SHELL
// ============================================================================

struct DesktopShell {
    fb: Framebuffer,
    taskbar: Taskbar,
    // input_port: redpowder::ipc::Port,
}

impl DesktopShell {
    fn new() -> Result<Self, ()> {
        // Tenta conectar ao Framebuffer
        // NOTA: Em um sistema ideal, pediríamos uma superfície ao Compositor.
        // Como paliativo temporário, desenhamos direto no FB com cuidado.
        let fb = Framebuffer::new().map_err(|_| ())?;
        let screen_w = fb.width();
        let screen_h = fb.height();

        let taskbar = Taskbar::new(screen_w, screen_h);

        // Cria porta de entrada com capacidade 32
        // Input por enquanto desativado no Shell para permitir que o Terminal o capture
        // let input_port = redpowder::ipc::Port::create("shell_input", 32).map_err(|_| ())?;

        Ok(Self {
            fb,
            taskbar,
            // input_port,
        })
    }

    fn run(&mut self) -> ! {
        println!("[Shell] Starting Desktop Environment...");

        // 1. Desenhar Wallpaper
        self.draw_wallpaper();

        // 2. Desenhar Taskbar inicial
        self.taskbar.draw(&mut self.fb);

        println!("[Shell] Desktop ready!");

        // Loop de eventos
        // let mut msg_buf = [0u8; 32];
        loop {
            // Processar mensagens de input
            // 0 = não bloqueante (retorna 0 bytes se vazio)
            /*
            while let Ok(len) = self.input_port.recv(&mut msg_buf, 0) {
                if len > 0 {
                    // TODO: Processar scancode
                    // Apenas drenamos
                } else {
                    break;
                }
            }
            */

            // Re-desenha taskbar periodicamente (ex: relógio)
            // Futuro: Esperar eventos do mouse/teclado via IPC do Compositor

            // self.taskbar.update();
            // self.taskbar.draw(&mut self.fb);

            redpowder::process::yield_now();
        }
    }

    fn draw_wallpaper(&mut self) {
        // Preenche área de trabalho
        // A taskbar sabe qual a área de trabalho
        let (x, y, w, h) = self
            .taskbar
            .get_work_area(self.fb.width(), self.fb.height());

        // Desenha apenas a área visível do wallpaper
        let _ = self.fb.fill_rect(x, y, w, h, WALLPAPER_COLOR);

        // TODO: Desenhar ícones do desktop
    }
}

// ============================================================================
// MAIN
// ============================================================================

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Pequeno delay para garantir que Compositor iniciou
    // TODO: Usar sincronização real via IPC
    for _ in 0..100000 {
        core::hint::spin_loop();
    }

    match DesktopShell::new() {
        Ok(mut shell) => {
            shell.run();
        }
        Err(_) => {
            println!("[Shell] ERRO FATAL: Falha ao acessar vídeo");
            loop {}
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
