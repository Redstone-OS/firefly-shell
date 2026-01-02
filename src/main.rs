//! # Firefly Shell v0.2.0
//!
//! Shell gráfico para RedstoneOS.
//!
//! ## Features v0.2.0
//!
//! - **Menu Iniciar Funcional**: Lista apps de `/apps/` recursivamente
//! - **Lançamento de Apps**: Clique para executar aplicativos
//! - **Taskbar Dinâmica**: Mostra janelas abertas com ícones
//! - **Relógio de Uptime**: Mostra tempo desde o boot
//!
//! ## Arquitetura
//!
//! ```text
//! shell/src/
//! ├── main.rs          # Entry point
//! ├── app/             # Gerenciamento de apps
//! │   ├── desktop.rs   # Desktop Environment
//! │   ├── discovery.rs # Descoberta de apps
//! │   └── launcher.rs  # Lançamento de apps
//! ├── ui/              # Componentes visuais
//! │   ├── taskbar.rs   # Barra de tarefas
//! │   └── wallpaper.rs # Papel de parede
//! ├── theme/           # Sistema de temas
//! │   └── colors.rs    # Paleta de cores
//! └── render/          # Renderização
//!     └── font.rs      # Fontes
//! ```

#![no_std]
#![no_main]

extern crate alloc;

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
    redpowder::process::exit(1);
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
    println!("[Shell] ========================================");
    println!("[Shell] Firefly Shell v0.2.0");
    println!("[Shell] ========================================");

    match app::Desktop::new() {
        Ok(mut desktop) => {
            desktop.run();
        }
        Err(e) => {
            println!("[Shell] FATAL: Erro ao inicializar desktop: {:?}", e);
            redpowder::process::exit(1);
        }
    }
}
