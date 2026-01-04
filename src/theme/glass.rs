//! # Glass Effects
//!
//! Efeitos visuais de vidro/blur para a UI.

use gfx_types::color::Color;
use gfx_types::geometry::{Rect, Size};

use super::colors;

// =============================================================================
// GLASS STYLE
// =============================================================================

/// Estilo de glass effect.
#[derive(Clone, Copy, Debug)]
pub struct GlassStyle {
    /// Cor de fundo.
    pub background: Color,
    /// Cor de borda.
    pub border: Color,
    /// Cor de highlight (topo).
    pub highlight: Color,
    /// Raio dos cantos.
    pub corner_radius: u32,
    /// Espessura da borda.
    pub border_thickness: u32,
}

impl GlassStyle {
    /// Estilo padrão para barras.
    pub const fn bar() -> Self {
        Self {
            background: colors::GLASS_BG,
            border: colors::GLASS_BORDER,
            highlight: colors::GLASS_BORDER_LIGHT,
            corner_radius: 12,
            border_thickness: 1,
        }
    }

    /// Estilo para painéis/menus.
    pub const fn panel() -> Self {
        Self {
            background: colors::MENU_BG,
            border: colors::GLASS_BORDER,
            highlight: colors::GLASS_BORDER_LIGHT,
            corner_radius: 16,
            border_thickness: 1,
        }
    }

    // TODO: Revisar no futuro
    #[allow(unused)]
    /// Estilo para botões.
    pub const fn button() -> Self {
        Self {
            background: Color(0x00000000), // Transparente por padrão
            border: Color(0x00000000),
            highlight: Color(0x00000000),
            corner_radius: 8,
            border_thickness: 0,
        }
    }

    // TODO: Revisar no futuro
    #[allow(unused)]
    /// Estilo hover.
    pub const fn button_hover() -> Self {
        Self {
            background: colors::GLASS_BG_HOVER,
            border: Color(0x20FFFFFF),
            highlight: Color(0x10FFFFFF),
            corner_radius: 8,
            border_thickness: 1,
        }
    }

    // TODO: Revisar no futuro
    #[allow(unused)]
    /// Estilo ativo/pressionado.
    pub const fn button_active() -> Self {
        Self {
            background: colors::GLASS_BG_ACTIVE,
            border: colors::ACCENT,
            highlight: Color(0x05FFFFFF),
            corner_radius: 8,
            border_thickness: 1,
        }
    }
}

// =============================================================================
// GLASS RENDERER
// =============================================================================

/// Renderiza efeitos glass em um buffer.
pub struct GlassRenderer;

impl GlassRenderer {
    /// Desenha um retângulo com efeito glass.
    pub fn draw_rect(buffer: &mut [u32], buffer_size: Size, rect: Rect, style: &GlassStyle) {
        let stride = buffer_size.width as usize;

        // Preencher fundo com cantos arredondados
        Self::fill_rounded_rect(
            buffer,
            stride,
            buffer_size,
            rect,
            style.background,
            style.corner_radius,
        );

        // Desenhar borda se tiver espessura
        if style.border_thickness > 0 {
            Self::stroke_rounded_rect(
                buffer,
                stride,
                buffer_size,
                rect,
                style.border,
                style.corner_radius,
            );
        }

        // Highlight no topo (efeito vidro)
        if style.highlight.alpha() > 0 {
            let highlight_rect = Rect::new(
                rect.x + style.corner_radius as i32,
                rect.y + 1,
                rect.width - style.corner_radius * 2,
                1,
            );
            Self::fill_rect_blend(buffer, stride, buffer_size, highlight_rect, style.highlight);
        }
    }

    /// Preenche retângulo com cantos arredondados.
    fn fill_rounded_rect(
        buffer: &mut [u32],
        stride: usize,
        size: Size,
        rect: Rect,
        color: Color,
        radius: u32,
    ) {
        let r = radius as i32;

        for y in 0..rect.height as i32 {
            let sy = rect.y + y;
            if sy < 0 || sy >= size.height as i32 {
                continue;
            }

            // Calcular inset horizontal baseado na posição vertical
            let inset = if y < r {
                Self::corner_inset(r, r - 1 - y)
            } else if y >= (rect.height as i32 - r) {
                Self::corner_inset(r, y - (rect.height as i32 - r))
            } else {
                0
            };

            let x_start = (rect.x + inset).max(0);
            let x_end = (rect.x + rect.width as i32 - inset).min(size.width as i32);

            if x_start < x_end {
                let row_start = sy as usize * stride + x_start as usize;
                let row_end = sy as usize * stride + x_end as usize;

                if row_end <= buffer.len() {
                    let color_u32 = color.as_u32();
                    let alpha = color.alpha();

                    if alpha == 255 {
                        buffer[row_start..row_end].fill(color_u32);
                    } else if alpha > 0 {
                        for px in &mut buffer[row_start..row_end] {
                            *px = Self::blend(*px, color_u32);
                        }
                    }
                }
            }
        }
    }

    /// Desenha borda de retângulo arredondado.
    fn stroke_rounded_rect(
        buffer: &mut [u32],
        stride: usize,
        size: Size,
        rect: Rect,
        color: Color,
        radius: u32,
    ) {
        let r = radius as i32;
        let color_u32 = color.as_u32();

        // Bordas horizontais (topo e fundo)
        for x in r..(rect.width as i32 - r) {
            let px_top = rect.x + x;
            let px_bot = rect.x + x;
            let py_top = rect.y;
            let py_bot = rect.y + rect.height as i32 - 1;

            Self::put_pixel_blend(buffer, stride, size, px_top, py_top, color_u32);
            Self::put_pixel_blend(buffer, stride, size, px_bot, py_bot, color_u32);
        }

        // Bordas verticais (esquerda e direita)
        for y in r..(rect.height as i32 - r) {
            let px_left = rect.x;
            let px_right = rect.x + rect.width as i32 - 1;
            let py = rect.y + y;

            Self::put_pixel_blend(buffer, stride, size, px_left, py, color_u32);
            Self::put_pixel_blend(buffer, stride, size, px_right, py, color_u32);
        }

        // Cantos arredondados
        Self::draw_corner_arc(
            buffer,
            stride,
            size,
            rect.x + r,
            rect.y + r,
            r,
            color_u32,
            0,
        ); // Top-left
        Self::draw_corner_arc(
            buffer,
            stride,
            size,
            rect.x + rect.width as i32 - r - 1,
            rect.y + r,
            r,
            color_u32,
            1,
        ); // Top-right
        Self::draw_corner_arc(
            buffer,
            stride,
            size,
            rect.x + r,
            rect.y + rect.height as i32 - r - 1,
            r,
            color_u32,
            2,
        ); // Bottom-left
        Self::draw_corner_arc(
            buffer,
            stride,
            size,
            rect.x + rect.width as i32 - r - 1,
            rect.y + rect.height as i32 - r - 1,
            r,
            color_u32,
            3,
        ); // Bottom-right
    }

    /// Desenha arco de canto.
    fn draw_corner_arc(
        buffer: &mut [u32],
        stride: usize,
        size: Size,
        cx: i32,
        cy: i32,
        r: i32,
        color: u32,
        quadrant: u8,
    ) {
        // Bresenham circle adapted for quarter
        let mut x = 0;
        let mut y = r;
        let mut d = 3 - 2 * r;

        while x <= y {
            let points = match quadrant {
                0 => [(-x, -y), (-y, -x)], // Top-left
                1 => [(x, -y), (y, -x)],   // Top-right
                2 => [(-x, y), (-y, x)],   // Bottom-left
                3 => [(x, y), (y, x)],     // Bottom-right
                _ => return,
            };

            for (dx, dy) in points {
                Self::put_pixel_blend(buffer, stride, size, cx + dx, cy + dy, color);
            }

            if d < 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }

    /// Calcula inset para cantos arredondados.
    fn corner_inset(radius: i32, distance: i32) -> i32 {
        if distance >= radius {
            return radius;
        }
        // Aproximação circular
        let r = radius as f32;
        let d = distance as f32;
        let inset = r - rdsmath::sqrtf(r * r - d * d);
        inset as i32
    }

    /// Preenche retângulo com blend.
    fn fill_rect_blend(buffer: &mut [u32], stride: usize, size: Size, rect: Rect, color: Color) {
        let color_u32 = color.as_u32();

        for y in 0..rect.height as i32 {
            let sy = rect.y + y;
            if sy < 0 || sy >= size.height as i32 {
                continue;
            }

            for x in 0..rect.width as i32 {
                let sx = rect.x + x;
                if sx < 0 || sx >= size.width as i32 {
                    continue;
                }

                let idx = sy as usize * stride + sx as usize;
                if idx < buffer.len() {
                    buffer[idx] = Self::blend(buffer[idx], color_u32);
                }
            }
        }
    }

    /// Coloca pixel com blend.
    fn put_pixel_blend(buffer: &mut [u32], stride: usize, size: Size, x: i32, y: i32, color: u32) {
        if x < 0 || y < 0 || x >= size.width as i32 || y >= size.height as i32 {
            return;
        }
        let idx = y as usize * stride + x as usize;
        if idx < buffer.len() {
            buffer[idx] = Self::blend(buffer[idx], color);
        }
    }

    /// Alpha blend.
    fn blend(dst: u32, src: u32) -> u32 {
        let sa = (src >> 24) & 0xFF;
        if sa == 0 {
            return dst;
        }
        if sa == 255 {
            return src;
        }

        let sr = (src >> 16) & 0xFF;
        let sg = (src >> 8) & 0xFF;
        let sb = src & 0xFF;

        let dr = (dst >> 16) & 0xFF;
        let dg = (dst >> 8) & 0xFF;
        let db = dst & 0xFF;

        let inv_sa = 255 - sa;
        let out_r = (sr * sa + dr * inv_sa) / 255;
        let out_g = (sg * sa + dg * inv_sa) / 255;
        let out_b = (sb * sa + db * inv_sa) / 255;

        0xFF000000 | (out_r << 16) | (out_g << 8) | out_b
    }
}
