//! # App Module
//!
//! Gerenciamento de aplicações.
//!
//! ## Componentes
//!
//! - **desktop**: Desktop Environment principal
//! - **discovery**: Descoberta de apps com app.toml
//! - **launcher**: Lançamento de apps

mod desktop;
mod discovery;
mod launcher;

pub use desktop::Desktop;
pub use discovery::{discover_apps, AppIcon, AppInfo};
pub use launcher::launch_app;
