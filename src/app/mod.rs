//! # App Module
//!
//! Módulos de aplicativos do Shell.

mod desktop;
mod discovery;
mod launcher;

pub use desktop::Desktop;
pub use discovery::{discover_apps, AppCategory, AppIcon, AppInfo};
pub use launcher::launch_app;
