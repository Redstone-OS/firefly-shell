//! # Wallpaper - Fundo de Tela
//!
//! Componente responsável por renderizar o fundo do desktop.

use crate::theme::colors;
use redpowder::window::Window;

// ============================================================================
// WALLPAPER
// ============================================================================

/// Componente de wallpaper do desktop.
pub struct Wallpaper {
    /// Cor do wallpaper
    pub color: u32,
    /// Área do wallpaper (x, y, width, height)
    pub bounds: (u32, u32, u32, u32),
}

impl Wallpaper {
    /// Cria um novo wallpaper com cor padrão.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            color: colors::WALLPAPER_COLOR,
            bounds: (0, 0, width, height),
        }
    }

    /// Cria wallpaper com área personalizada (para excluir taskbar).
    pub fn with_bounds(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            color: colors::WALLPAPER_COLOR,
            bounds: (x, y, width, height),
        }
    }

    /// Altera a cor do wallpaper.
    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }

    /// Desenha o wallpaper na janela.
    pub fn draw(&self, window: &mut Window) {
        let (x, y, w, h) = self.bounds;
        window.fill_rect(x, y, w, h, self.color);
    }
}
