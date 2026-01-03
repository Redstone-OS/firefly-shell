//! # Firefly Shell v0.3.0
//!
//! Shell gráfico para RedstoneOS.
//!
//! ## Features v0.3.0
//!
//! - **Taskbar Flutuante**: 3 barras com efeito glass
//! - **Painel de Widgets**: Slide-up da esquerda
//! - **Quick Settings**: Configurações rápidas da direita
//! - **Menu Iniciar**: Lista apps com ícones do app.toml
//! - **Wallpaper**: Suporte a webp com fallback gradiente
//!
//! ## Arquitetura
//!
//! ```text
//! shell/src/
//! ├── main.rs           # Entry point
//! ├── app/              # Gerenciamento de apps
//! │   ├── desktop.rs    # Desktop Environment
//! │   ├── discovery.rs  # Descoberta de apps (app.toml)
//! │   └── launcher.rs   # Lançamento de apps
//! ├── ui/               # Componentes visuais
//! │   ├── wallpaper.rs  # Papel de parede
//! │   ├── taskbar.rs    # Barras flutuantes
//! │   └── panels/       # Painéis popup
//! │       ├── widget_panel.rs
//! │       ├── start_menu.rs
//! │       └── quick_settings.rs
//! ├── theme/            # Sistema de temas
//! │   ├── colors.rs     # Paleta de cores
//! │   ├── glass.rs      # Efeitos de vidro
//! │   └── metrics.rs    # Métricas de layout
//! └── render/           # Renderização
//!     └── font.rs       # Fontes
//! ```

#![no_std]
#![no_main]

extern crate alloc;

mod app;
mod render;
mod theme;
mod ui;

use redpowder::println;

// =============================================================================
// RUNTIME NO_STD
// =============================================================================

/// Allocator global.
#[global_allocator]
static ALLOCATOR: redpowder::mem::heap::SyscallAllocator = redpowder::mem::heap::SyscallAllocator;

/// Panic handler.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("[Shell] PANIC: {:?}", info);
    redpowder::process::exit(1);
}

// =============================================================================
// ENTRY POINT
// =============================================================================

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main()
}

fn main() -> ! {
    println!("[Shell] ========================================");
    println!("[Shell] Firefly Shell v0.3.0");
    println!("[Shell] ========================================");

    match app::Desktop::new() {
        Ok(mut desktop) => {
            desktop.run();
        }
        Err(e) => {
            println!("[Shell] FATAL: Erro ao inicializar: {:?}", e);
            redpowder::process::exit(1);
        }
    }
}
