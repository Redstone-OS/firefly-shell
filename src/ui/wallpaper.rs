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

        if h == 0 || w == 0 {
            return; // Evitar divisão por zero
        }

        if self.use_gradient {
            self.draw_gradient(window, x, y, w, h);
        } else {
            window.fill_rect(x, y, w, h, colors::WALLPAPER_COLOR);
        }
    }

    /// Desenha gradiente vertical (laranja para vermelho escuro)
    fn draw_gradient(&self, window: &mut Window, x: u32, y: u32, w: u32, h: u32) {
        // Cores do gradiente (RGB como u8)
        // Topo: Laranja vibrante
        let top_r: u8 = 0xFF;
        let top_g: u8 = 0x6B;
        let top_b: u8 = 0x35;

        // Base: Laranja escuro / Vermelho
        let bot_r: u8 = 0xCC;
        let bot_g: u8 = 0x33;
        let bot_b: u8 = 0x00;

        for row in 0..h {
            // Interpolação linear usando aritmética saturada
            let r = lerp_u8(top_r, bot_r, row, h);
            let g = lerp_u8(top_g, bot_g, row, h);
            let b = lerp_u8(top_b, bot_b, row, h);

            let color = 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);

            // Desenhar linha horizontal
            window.fill_rect(x, y + row, w, 1, color);
        }
    }
}

/// Interpolação linear segura entre dois valores u8
fn lerp_u8(start: u8, end: u8, current: u32, total: u32) -> u8 {
    if total == 0 {
        return start;
    }

    // Usar i32 para evitar overflow e depois clampar
    let start_i = start as i32;
    let end_i = end as i32;
    let diff = end_i - start_i;

    let result = start_i + (diff * current as i32 / total as i32);

    // Clampar para range válido de u8
    result.clamp(0, 255) as u8
}
