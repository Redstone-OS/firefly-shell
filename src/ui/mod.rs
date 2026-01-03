//! # UI Module
//!
//! Componentes visuais do Shell.
//!
//! ## Componentes
//!
//! - **wallpaper**: Papel de parede (webp ou gradiente)
//! - **taskbar**: Barras flutuantes na parte inferior
//! - **panels**: Painéis popup (widgets, quick settings, start menu)

pub mod panels;
mod taskbar;
mod wallpaper;

pub use panels::{Panel, PanelType, QuickSettingsPanel, StartMenuPanel, WidgetPanel};
pub use taskbar::{Taskbar, TaskbarAction};
pub use wallpaper::Wallpaper;
