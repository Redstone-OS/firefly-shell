//! # Panels
//!
//! Painéis popup do Shell.
//!
//! ## Componentes
//!
//! - **WidgetPanel**: Painel de widgets (esquerda)
//! - **StartMenuPanel**: Menu iniciar (centro)
//! - **QuickSettingsPanel**: Configurações rápidas (direita)

mod quick_settings;
mod start_menu;
mod widget_panel;

pub use quick_settings::QuickSettingsPanel;
pub use start_menu::{StartMenuAction, StartMenuPanel};
pub use widget_panel::WidgetPanel;

use gfx_types::geometry::Rect;

// =============================================================================
// PANEL TRAIT
// =============================================================================

/// Tipo de painel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelType {
    Widget,
    StartMenu,
    QuickSettings,
}

/// Trait para painéis.
pub trait Panel {
    /// Retorna se está visível.
    fn is_visible(&self) -> bool;

    /// Define visibilidade.
    fn set_visible(&mut self, visible: bool);

    /// Toggle visibilidade.
    fn toggle(&mut self) {
        let visible = self.is_visible();
        self.set_visible(!visible);
    }

    /// Retorna bounds.
    fn bounds(&self) -> Rect;

    /// Desenha o painel.
    fn draw(&self, buffer: &mut [u32], buffer_size: gfx_types::geometry::Size);

    /// Processa clique. Retorna true se consumiu.
    fn handle_click(&mut self, x: i32, y: i32) -> bool;

    /// Atualiza animação. Retorna true se ainda animando.
    fn update_animation(&mut self) -> bool;
}
