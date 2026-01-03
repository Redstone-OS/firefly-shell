//! # Widget Panel
//!
//! Painel de widgets que abre da esquerda.

use gfx_types::geometry::{Rect, Size};

use crate::theme::{colors, metrics, GlassRenderer, GlassStyle};

use super::Panel;

// =============================================================================
// WIDGET PANEL
// =============================================================================

/// Painel de widgets.
pub struct WidgetPanel {
    /// Bounds do painel.
    bounds: Rect,
    /// Visível.
    visible: bool,
    /// Progresso de animação (0.0 - 1.0).
    animation_progress: f32,
    /// Altura da tela.
    screen_height: u32,
}

impl WidgetPanel {
    /// Cria novo painel.
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let margin = metrics::TASKBAR_MARGIN;
        let taskbar_y = screen_height as i32 - metrics::TASKBAR_HEIGHT as i32 - margin as i32;

        Self {
            bounds: Rect::new(
                margin as i32,
                taskbar_y - metrics::WIDGET_PANEL_HEIGHT as i32 - 8,
                metrics::WIDGET_PANEL_WIDTH,
                metrics::WIDGET_PANEL_HEIGHT,
            ),
            visible: false,
            animation_progress: 0.0,
            screen_height,
        }
    }

    /// Desenha texto "Ainda não há widgets".
    fn draw_empty_message(&self, buffer: &mut [u32], buffer_size: Size) {
        let stride = buffer_size.width as usize;
        let color = colors::TEXT_SECONDARY.as_u32();

        // Posição central
        let cx = self.bounds.x + self.bounds.width as i32 / 2;
        let cy = self.bounds.y + self.bounds.height as i32 / 2;

        // Ícone de widgets (4 quadrados grandes)
        let icon_size = 48;
        let ix = cx - icon_size / 2;
        let iy = cy - 40;
        let half = icon_size / 2 - 4;
        let gap = 8;

        let icon_color = colors::ICON_DISABLED.as_u32();
        Self::fill_rect(
            buffer,
            stride,
            buffer_size,
            ix,
            iy,
            half as u32,
            half as u32,
            icon_color,
        );
        Self::fill_rect(
            buffer,
            stride,
            buffer_size,
            ix + half + gap,
            iy,
            half as u32,
            half as u32,
            icon_color,
        );
        Self::fill_rect(
            buffer,
            stride,
            buffer_size,
            ix,
            iy + half + gap,
            half as u32,
            half as u32,
            icon_color,
        );
        Self::fill_rect(
            buffer,
            stride,
            buffer_size,
            ix + half + gap,
            iy + half + gap,
            half as u32,
            half as u32,
            icon_color,
        );

        // Texto placeholder
        let text_y = cy + 30;
        Self::fill_rect(buffer, stride, buffer_size, cx - 80, text_y, 160, 2, color);
        Self::fill_rect(
            buffer,
            stride,
            buffer_size,
            cx - 60,
            text_y + 10,
            120,
            2,
            color,
        );
    }

    fn fill_rect(
        buffer: &mut [u32],
        stride: usize,
        size: Size,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        color: u32,
    ) {
        for dy in 0..h as i32 {
            let py = y + dy;
            if py < 0 || py >= size.height as i32 {
                continue;
            }
            for dx in 0..w as i32 {
                let px = x + dx;
                if px < 0 || px >= size.width as i32 {
                    continue;
                }
                let idx = py as usize * stride + px as usize;
                if idx < buffer.len() {
                    buffer[idx] = color;
                }
            }
        }
    }
}

impl Panel for WidgetPanel {
    fn is_visible(&self) -> bool {
        self.visible || self.animation_progress > 0.0
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn draw(&self, buffer: &mut [u32], buffer_size: Size) {
        if !self.is_visible() {
            return;
        }

        // Calcular posição animada (slide up)
        let target_y = self.bounds.y;
        let start_y = self.screen_height as i32;
        let current_y = start_y + ((target_y - start_y) as f32 * self.animation_progress) as i32;

        let animated_bounds = Rect::new(
            self.bounds.x,
            current_y,
            self.bounds.width,
            self.bounds.height,
        );

        // Desenhar fundo glass
        let style = GlassStyle::panel();
        GlassRenderer::draw_rect(buffer, buffer_size, animated_bounds, &style);

        // Desenhar conteúdo (com offset)
        if self.animation_progress > 0.5 {
            self.draw_empty_message(buffer, buffer_size);
        }
    }

    fn handle_click(&mut self, x: i32, y: i32) -> bool {
        if !self.visible {
            return false;
        }
        self.bounds
            .contains_point(gfx_types::geometry::Point::new(x, y))
    }

    fn update_animation(&mut self) -> bool {
        let target = if self.visible { 1.0 } else { 0.0 };
        let speed = 0.15;

        if (self.animation_progress - target).abs() < 0.01 {
            self.animation_progress = target;
            return false;
        }

        if self.animation_progress < target {
            self.animation_progress = (self.animation_progress + speed).min(1.0);
        } else {
            self.animation_progress = (self.animation_progress - speed).max(0.0);
        }

        true
    }
}
