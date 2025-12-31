//! # Firefly Shell
//!
//! Shell gráfico para RedstoneOS.
//!
//! ## Arquitetura
//!
//! ```text
//! shell/src/
//! ├── main.rs         # Entry point
//! ├── app/            # Desktop environment
//! │   ├── mod.rs
//! │   └── desktop.rs
//! ├── ui/             # Componentes visuais
//! │   ├── mod.rs
//! │   ├── taskbar.rs
//! │   └── wallpaper.rs
//! ├── theme/          # Sistema de temas
//! │   ├── mod.rs
//! │   └── colors.rs
//! └── render/         # Renderização
//!     ├── mod.rs
//!     └── font.rs
//! ```

#![no_std]
#![no_main]

extern crate alloc;

// Módulos do Shell
mod app;
mod render;
mod theme;
mod ui;

use redpowder::println;

// ============================================================================
// RUNTIME NO_STD
// ============================================================================

/// Allocator global usando syscalls.
#[global_allocator]
static ALLOCATOR: redpowder::mem::heap::SyscallAllocator = redpowder::mem::heap::SyscallAllocator;

/// Panic handler.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("[Shell] PANIC: {:?}", info);
    loop {
        let _ = redpowder::time::sleep(1000);
    }
}

// ============================================================================
// PONTO DE ENTRADA
// ============================================================================

/// Ponto de entrada do shell.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    main()
}

/// Função principal.
fn main() -> ! {
    // Criar e executar o desktop
    match app::Desktop::new() {
        Ok(mut desktop) => {
            desktop.run();
        }
        Err(e) => {
            println!("[Shell] FATAL: Erro ao inicializar desktop: {:?}", e);
            loop {
                let _ = redpowder::time::sleep(1000);
            }
        }
    }
}
