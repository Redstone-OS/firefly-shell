//! # Quick Settings Panel
//!
//! Painel de configurações rápidas.

use gfx_types::geometry::{Point, Rect, Size};

use crate::theme::{colors, metrics, GlassRenderer, GlassStyle};

use super::Panel;

// =============================================================================
// QUICK SETTINGS
// =============================================================================

/// Item de configuração rápida.
#[derive(Clone, Copy)]
struct QuickSettingItem {
    /// Tipo do item.
    item_type: QuickSettingType,
    /// Ativo.
    active: bool,
}

/// Tipo de item.
#[derive(Clone, Copy, Debug)]
enum QuickSettingType {
    Wifi,
    Bluetooth,
    Volume,
    Brightness,
    DoNotDisturb,
    AirplaneMode,
}

/// Painel de configurações rápidas.
pub struct QuickSettingsPanel {
    /// Bounds do painel.
    bounds: Rect,
    /// Visível.
    visible: bool,
    /// Progresso de animação.
    animation_progress: f32,
    /// Altura da tela.
    screen_height: u32,
    /// Itens.
    items: [QuickSettingItem; 6],
}

impl QuickSettingsPanel {
    /// Cria novo painel.
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let margin = metrics::TASKBAR_MARGIN;
        let taskbar_y = screen_height as i32 - metrics::TASKBAR_HEIGHT as i32 - margin as i32;

        Self {
            bounds: Rect::new(
                screen_width as i32 - metrics::QUICK_SETTINGS_WIDTH as i32 - margin as i32,
                taskbar_y - metrics::QUICK_SETTINGS_HEIGHT as i32 - 8,
                metrics::QUICK_SETTINGS_WIDTH,
                metrics::QUICK_SETTINGS_HEIGHT,
            ),
            visible: false,
            animation_progress: 0.0,
            screen_height,
            items: [
                QuickSettingItem {
                    item_type: QuickSettingType::Wifi,
                    active: true,
                },
                QuickSettingItem {
                    item_type: QuickSettingType::Bluetooth,
                    active: false,
                },
                QuickSettingItem {
                    item_type: QuickSettingType::Volume,
                    active: true,
                },
                QuickSettingItem {
                    item_type: QuickSettingType::Brightness,
                    active: true,
                },
                QuickSettingItem {
                    item_type: QuickSettingType::DoNotDisturb,
                    active: false,
                },
                QuickSettingItem {
                    item_type: QuickSettingType::AirplaneMode,
                    active: false,
                },
            ],
        }
    }

    /// Desenha o grid de configurações.
    fn draw_settings_grid(&self, buffer: &mut [u32], buffer_size: Size) {
        let stride = buffer_size.width as usize;
        let padding = metrics::PANEL_PADDING as i32;

        let cell_size = 80i32;
        let gap = 12i32;
        let cols = 3;

        for (i, item) in self.items.iter().enumerate() {
            let col = (i % cols) as i32;
            let row = (i / cols) as i32;

            let x = self.bounds.x + padding + col * (cell_size + gap);
            let y = self.bounds.y + padding + row * (cell_size + gap);

            // Fundo do botão
            let bg_color = if item.active {
                colors::ACCENT.as_u32()
            } else {
                colors::BG_MEDIUM.as_u32()
            };

            // Desenhar botão arredondado
            Self::fill_rounded_rect(
                buffer,
                stride,
                buffer_size,
                x,
                y,
                cell_size as u32,
                cell_size as u32,
                bg_color,
                12,
            );

            // Ícone
            let icon_color = if item.active {
                colors::TEXT_ON_ACCENT.as_u32()
            } else {
                colors::ICON_NORMAL.as_u32()
            };

            self.draw_setting_icon(
                buffer,
                stride,
                buffer_size,
                x + cell_size / 2,
                y + cell_size / 2 - 8,
                item.item_type,
                icon_color,
            );
        }
    }

    /// Desenha ícone de configuração.
    fn draw_setting_icon(
        &self,
        buffer: &mut [u32],
        stride: usize,
        size: Size,
        cx: i32,
        cy: i32,
        item_type: QuickSettingType,
        color: u32,
    ) {
        match item_type {
            QuickSettingType::Wifi => {
                // Arcos de wifi
                Self::fill_rect(buffer, stride, size, cx - 12, cy - 8, 24, 2, color);
                Self::fill_rect(buffer, stride, size, cx - 8, cy - 2, 16, 2, color);
                Self::fill_rect(buffer, stride, size, cx - 4, cy + 4, 8, 2, color);
                Self::fill_rect(buffer, stride, size, cx - 1, cy + 8, 2, 4, color);
            }
            QuickSettingType::Bluetooth => {
                // B estilizado
                Self::fill_rect(buffer, stride, size, cx - 2, cy - 10, 4, 20, color);
                Self::fill_rect(buffer, stride, size, cx + 2, cy - 8, 4, 2, color);
                Self::fill_rect(buffer, stride, size, cx + 2, cy - 2, 4, 2, color);
                Self::fill_rect(buffer, stride, size, cx + 2, cy + 4, 4, 2, color);
            }
            QuickSettingType::Volume => {
                // Alto-falante
                Self::fill_rect(buffer, stride, size, cx - 8, cy - 4, 6, 8, color);
                Self::fill_rect(buffer, stride, size, cx - 2, cy - 8, 4, 16, color);
                Self::fill_rect(buffer, stride, size, cx + 4, cy - 6, 2, 12, color);
                Self::fill_rect(buffer, stride, size, cx + 8, cy - 8, 2, 16, color);
            }
            QuickSettingType::Brightness => {
                // Sol
                Self::fill_rect(buffer, stride, size, cx - 4, cy - 4, 8, 8, color);
                Self::fill_rect(buffer, stride, size, cx - 1, cy - 10, 2, 4, color);
                Self::fill_rect(buffer, stride, size, cx - 1, cy + 6, 2, 4, color);
                Self::fill_rect(buffer, stride, size, cx - 10, cy - 1, 4, 2, color);
                Self::fill_rect(buffer, stride, size, cx + 6, cy - 1, 4, 2, color);
            }
            QuickSettingType::DoNotDisturb => {
                // Círculo com linha
                Self::fill_rect(buffer, stride, size, cx - 8, cy - 1, 16, 2, color);
            }
            QuickSettingType::AirplaneMode => {
                // Avião simplificado
                Self::fill_rect(buffer, stride, size, cx - 1, cy - 10, 2, 20, color);
                Self::fill_rect(buffer, stride, size, cx - 10, cy - 2, 20, 4, color);
                Self::fill_rect(buffer, stride, size, cx - 4, cy + 6, 8, 3, color);
            }
        }
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

    fn fill_rounded_rect(
        buffer: &mut [u32],
        stride: usize,
        size: Size,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        color: u32,
        _radius: u32,
    ) {
        // Versão simplificada (sem cantos arredondados por enquanto)
        Self::fill_rect(buffer, stride, size, x, y, w, h, color);
    }
}

impl Panel for QuickSettingsPanel {
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

        // Calcular posição animada
        let target_y = self.bounds.y;
        let start_y = self.screen_height as i32;
        let current_y = start_y + ((target_y - start_y) as f32 * self.animation_progress) as i32;

        let animated_bounds = Rect::new(
            self.bounds.x,
            current_y,
            self.bounds.width,
            self.bounds.height,
        );

        // Fundo glass
        let style = GlassStyle::panel();
        GlassRenderer::draw_rect(buffer, buffer_size, animated_bounds, &style);

        // Grid de configurações
        if self.animation_progress > 0.5 {
            self.draw_settings_grid(buffer, buffer_size);
        }
    }

    fn handle_click(&mut self, x: i32, y: i32) -> bool {
        if !self.visible {
            return false;
        }

        if self.bounds.contains_point(Point::new(x, y)) {
            // TODO: Toggle item clicado
            return true;
        }

        false
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
