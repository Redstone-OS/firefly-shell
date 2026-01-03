//! # Start Menu Panel
//!
//! Menu iniciar com lista de aplicativos.

use alloc::string::String;
use alloc::vec::Vec;
use gfx_types::geometry::{Point, Rect, Size};

use crate::app::AppInfo;
use crate::theme::{colors, metrics, GlassRenderer, GlassStyle};

use super::Panel;

// =============================================================================
// START MENU
// =============================================================================

/// Ação do menu.
#[derive(Debug, Clone)]
pub enum StartMenuAction {
    None,
    LaunchApp(String), // Path do app
}

/// Painel do menu iniciar.
pub struct StartMenuPanel {
    /// Bounds do painel.
    bounds: Rect,
    /// Visível.
    visible: bool,
    /// Progresso de animação.
    animation_progress: f32,
    /// Altura da tela.
    screen_height: u32,
    /// Apps disponíveis.
    apps: Vec<AppInfo>,
    /// Item hover.
    hover_index: Option<usize>,
    /// Última ação.
    last_action: StartMenuAction,
    /// Scroll offset.
    scroll_offset: i32,
}

impl StartMenuPanel {
    /// Cria novo painel.
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let margin = metrics::TASKBAR_MARGIN;
        let taskbar_y = screen_height as i32 - metrics::TASKBAR_HEIGHT as i32 - margin as i32;

        // Centralizar horizontalmente
        let x = (screen_width as i32 - metrics::START_MENU_WIDTH as i32) / 2;

        Self {
            bounds: Rect::new(
                x,
                taskbar_y - metrics::START_MENU_HEIGHT as i32 - 8,
                metrics::START_MENU_WIDTH,
                metrics::START_MENU_HEIGHT,
            ),
            visible: false,
            animation_progress: 0.0,
            screen_height,
            apps: Vec::new(),
            hover_index: None,
            last_action: StartMenuAction::None,
            scroll_offset: 0,
        }
    }

    /// Define apps disponíveis.
    pub fn set_apps(&mut self, apps: Vec<AppInfo>) {
        self.apps = apps;
    }

    /// Retorna última ação e limpa.
    pub fn take_action(&mut self) -> StartMenuAction {
        core::mem::replace(&mut self.last_action, StartMenuAction::None)
    }

    /// Desenha lista de apps.
    fn draw_app_list(&self, buffer: &mut [u32], buffer_size: Size) {
        let stride = buffer_size.width as usize;
        let padding = metrics::PANEL_PADDING as i32;

        // Título "Aplicativos"
        let title_y = self.bounds.y + padding;
        Self::fill_rect(
            buffer,
            stride,
            buffer_size,
            self.bounds.x + padding,
            title_y,
            100,
            3,
            colors::TEXT_PRIMARY.as_u32(),
        );

        // Separador
        let sep_y = title_y + 24;
        let sep_width = (self.bounds.width as i32 - padding * 2).max(0) as u32;
        Self::fill_rect(
            buffer,
            stride,
            buffer_size,
            self.bounds.x + padding,
            sep_y,
            sep_width,
            1,
            colors::MENU_SEPARATOR.as_u32(),
        );

        // Lista de apps
        let list_y = sep_y + 12;
        let item_height = metrics::APP_ITEM_HEIGHT as i32;
        let icon_size = metrics::APP_ICON_SIZE as i32;

        let visible_height = self.bounds.height as i32 - (list_y - self.bounds.y) - padding;
        let max_visible = (visible_height / item_height) as usize;

        for (i, app) in self.apps.iter().take(max_visible).enumerate() {
            let item_y = list_y + (i as i32) * item_height - self.scroll_offset;

            if item_y + item_height < self.bounds.y || item_y > self.bounds.bottom() {
                continue;
            }

            // Fundo do item (hover)
            if self.hover_index == Some(i) {
                Self::fill_rect(
                    buffer,
                    stride,
                    buffer_size,
                    self.bounds.x + padding / 2,
                    item_y,
                    self.bounds.width - padding as u32,
                    item_height as u32,
                    colors::MENU_ITEM_HOVER.as_u32(),
                );
            }

            // Placeholder do ícone
            let icon_x = self.bounds.x + padding;
            let icon_y = item_y + (item_height - icon_size) / 2;

            // Quadrado colorido como placeholder do ícone
            let icon_color = self.get_app_color(i);
            Self::fill_rect(
                buffer,
                stride,
                buffer_size,
                icon_x,
                icon_y,
                icon_size as u32,
                icon_size as u32,
                icon_color,
            );

            // Inicial do app no ícone
            let initial_x = icon_x + icon_size / 2 - 4;
            let initial_y = icon_y + icon_size / 2 - 4;
            Self::fill_rect(
                buffer,
                stride,
                buffer_size,
                initial_x,
                initial_y,
                8,
                8,
                colors::TEXT_ON_ACCENT.as_u32(),
            );

            // Nome do app (placeholder - linha)
            let name_x = icon_x + icon_size + metrics::APP_ICON_GAP as i32;
            let name_y = item_y + item_height / 2 - 2;
            let name_width = (app.name.len() * 8).min(200) as u32;
            Self::fill_rect(
                buffer,
                stride,
                buffer_size,
                name_x,
                name_y,
                name_width,
                4,
                colors::TEXT_PRIMARY.as_u32(),
            );
        }

        // Indicador de scroll se necessário
        let total_height = self.apps.len() as i32 * item_height;
        if total_height > visible_height {
            let scroll_ratio = self.scroll_offset as f32 / (total_height - visible_height) as f32;
            let scrollbar_height =
                (visible_height as f32 * visible_height as f32 / total_height as f32) as i32;
            let scrollbar_y =
                list_y + (scroll_ratio * (visible_height - scrollbar_height) as f32) as i32;

            Self::fill_rect(
                buffer,
                stride,
                buffer_size,
                self.bounds.right() - 6,
                scrollbar_y,
                3,
                scrollbar_height as u32,
                colors::GLASS_BORDER.as_u32(),
            );
        }
    }

    /// Cor do app baseada no índice.
    fn get_app_color(&self, index: usize) -> u32 {
        const COLORS: [u32; 6] = [
            0xFF4A90D9, // Azul
            0xFF3FB950, // Verde
            0xFFE53935, // Vermelho
            0xFFF0B429, // Amarelo
            0xFF9C27B0, // Roxo
            0xFFFF6B35, // Laranja
        ];
        COLORS[index % COLORS.len()]
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

    /// Encontra app pelo ponto.
    fn app_at_point(&self, x: i32, y: i32) -> Option<usize> {
        let padding = metrics::PANEL_PADDING as i32;
        let list_y = self.bounds.y + padding + 24 + 12; // Título + separador
        let item_height = metrics::APP_ITEM_HEIGHT as i32;

        if x < self.bounds.x + padding / 2 || x > self.bounds.right() - padding / 2 {
            return None;
        }

        if y < list_y {
            return None;
        }

        let relative_y = y - list_y + self.scroll_offset;
        let index = (relative_y / item_height) as usize;

        if index < self.apps.len() {
            Some(index)
        } else {
            None
        }
    }
}

impl Panel for StartMenuPanel {
    fn is_visible(&self) -> bool {
        self.visible || self.animation_progress > 0.0
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
        if !visible {
            self.hover_index = None;
            self.scroll_offset = 0;
        }
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

        // Lista de apps
        if self.animation_progress > 0.5 {
            self.draw_app_list(buffer, buffer_size);
        }
    }

    fn handle_click(&mut self, x: i32, y: i32) -> bool {
        if !self.visible {
            return false;
        }

        if !self.bounds.contains_point(Point::new(x, y)) {
            return false;
        }

        if let Some(index) = self.app_at_point(x, y) {
            if index < self.apps.len() {
                let path = self.apps[index].path.clone();
                self.last_action = StartMenuAction::LaunchApp(path);
                self.set_visible(false);
            }
        }

        true
    }

    fn update_animation(&mut self) -> bool {
        let target = if self.visible { 1.0 } else { 0.0 };
        let speed = 0.12;

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
