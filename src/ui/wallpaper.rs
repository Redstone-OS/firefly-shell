//! # Wallpaper - Fundo de Tela
//!
//! Componente responsável por renderizar o fundo do desktop.
//! Suporta cor sólida ou gradiente.

use crate::theme::colors;
use redpowder::window::Window;

// ============================================================================
// WALLPAPER
// ============================================================================

/// Componente de wallpaper do desktop.
pub struct Wallpaper {
    /// Área do wallpaper (x, y, width, height)
    pub bounds: (u32, u32, u32, u32),
    /// Usar gradiente ao invés de cor sólida
    pub use_gradient: bool,
}

impl Wallpaper {
    /// Cria um novo wallpaper com área padrão.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            bounds: (0, 0, width, height),
            use_gradient: true,
        }
    }

    /// Cria wallpaper com área personalizada (para excluir taskbar).
    pub fn with_bounds(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            bounds: (x, y, width, height),
            use_gradient: true,
        }
    }

    /// Desenha o wallpaper na janela.
    pub fn draw(&self, window: &mut Window) {
        let (x, y, w, h) = self.bounds;

        if self.use_gradient {
            self.draw_gradient(window, x, y, w, h);
        } else {
            window.fill_rect(x, y, w, h, colors::WALLPAPER_COLOR);
        }
    }

    /// Desenha gradiente vertical (laranja para vermelho escuro)
    fn draw_gradient(&self, window: &mut Window, x: u32, y: u32, w: u32, h: u32) {
        // Cores do gradiente (ARGB)
        // Topo: Laranja vibrante
        let top_r: u32 = 0xFF;
        let top_g: u32 = 0x6B;
        let top_b: u32 = 0x35;

        // Base: Laranja escuro / Vermelho
        let bot_r: u32 = 0xCC;
        let bot_g: u32 = 0x33;
        let bot_b: u32 = 0x00;

        for row in 0..h {
            // Calcular interpolação (0.0 a 1.0)
            let t = row as u32;
            let h_val = h;

            // Interpolar cada componente de cor
            let r = top_r + ((bot_r as i32 - top_r as i32) * t as i32 / h_val as i32) as u32;
            let g = top_g + ((bot_g as i32 - top_g as i32) * t as i32 / h_val as i32) as u32;
            let b = top_b + ((bot_b as i32 - top_b as i32) * t as i32 / h_val as i32) as u32;

            let color = 0xFF000000 | (r << 16) | (g << 8) | b;

            // Desenhar linha horizontal
            window.fill_rect(x, y + row, w, 1, color);
        }
    }
}
