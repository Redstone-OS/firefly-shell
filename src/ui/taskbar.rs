//! # Taskbar
//!
//! Barra de tarefas com 3 barras flutuantes.

// TODO: Revisar no futuro
#[allow(unused)]
use gfx_types::color::Color;
use gfx_types::geometry::{Point, Rect, Size};

use crate::app::AppInfo;
use crate::theme::{colors, metrics, GlassRenderer, GlassStyle};

use alloc::string::String;
use alloc::vec::Vec;

// =============================================================================
// TIPOS
// =============================================================================

// TODO: Revisar no futuro
#[allow(unused)]
/// Ação retornada pelo tratamento de clique.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskbarAction {
    None,
    ToggleWidgetPanel,
    ToggleStartMenu,
    ToggleQuickSettings,
    ToggleWindow(u32),
    LaunchApp(usize),
}

// TODO: Revisar no futuro
#[allow(unused)]
/// Entrada de janela na taskbar.
#[derive(Clone)]
struct WindowEntry {
    id: u32,
    title: String,
    minimized: bool,
}

// =============================================================================
// TASKBAR
// =============================================================================

/// Barra de tarefas com 3 barras flutuantes.
pub struct Taskbar {
    /// Largura da tela.
    screen_width: u32,
    /// Altura da tela.
    screen_height: u32,

    // Barras
    /// Rect da barra de widgets (esquerda).
    widget_bar: Rect,
    /// Rect da barra central (menu).
    center_bar: Rect,
    /// Rect da barra de status (direita).
    status_bar: Rect,

    // Estado
    /// Janelas abertas.
    entries: Vec<WindowEntry>,
    /// Apps disponíveis.
    pub available_apps: Vec<AppInfo>,
    /// Uptime em segundos.
    uptime_secs: u64,

    // Hover state
    /// Barra atualmente com hover (0=none, 1=widget, 2=center, 3=status).
    // TODO: Revisar no futuro
    #[allow(unused)]
    hover_bar: u8,
}

impl Taskbar {
    /// Cria nova taskbar.
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let mut taskbar = Self {
            screen_width,
            screen_height,
            widget_bar: Rect::ZERO,
            center_bar: Rect::ZERO,
            status_bar: Rect::ZERO,
            entries: Vec::new(),
            available_apps: Vec::new(),
            uptime_secs: 0,
            hover_bar: 0,
        };

        taskbar.calculate_bars();
        taskbar
    }

    /// Calcula posições das barras.
    fn calculate_bars(&mut self) {
        let margin = metrics::TASKBAR_MARGIN;
        let height = metrics::TASKBAR_HEIGHT;
        let y = self.screen_height as i32 - height as i32 - margin as i32;

        // Barra de widgets (esquerda)
        self.widget_bar = Rect::new(margin as i32, y, metrics::WIDGET_BAR_WIDTH, height);

        // Barra de status (direita)
        self.status_bar = Rect::new(
            self.screen_width as i32 - metrics::STATUS_BAR_WIDTH as i32 - margin as i32,
            y,
            metrics::STATUS_BAR_WIDTH,
            height,
        );

        // Barra central (entre as duas)
        let center_x = self.widget_bar.right() + metrics::TASKBAR_GAP as i32;
        let center_width = (self.status_bar.x - center_x - metrics::TASKBAR_GAP as i32) as u32;

        self.center_bar = Rect::new(
            center_x,
            y,
            center_width.max(metrics::CENTER_BAR_MIN_WIDTH),
            height,
        );
    }

    /// Define apps disponíveis.
    pub fn set_available_apps(&mut self, apps: Vec<AppInfo>) {
        self.available_apps = apps;
    }

    /// Adiciona janela.
    pub fn add_window(&mut self, id: u32, title: String) {
        if !self.entries.iter().any(|e| e.id == id) {
            self.entries.push(WindowEntry {
                id,
                title,
                minimized: false,
            });
        }
    }

    /// Remove janela.
    pub fn remove_window(&mut self, id: u32) {
        self.entries.retain(|e| e.id != id);
    }

    /// Define estado de minimizado.
    pub fn set_window_minimized(&mut self, id: u32, minimized: bool) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.minimized = minimized;
        }
    }

    /// Obtém estado de minimizado.
    pub fn get_window_state(&self, id: u32) -> Option<bool> {
        self.entries
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.minimized)
    }

    /// Atualiza uptime.
    pub fn update_uptime(&mut self) {
        if let Ok(ms) = redpowder::time::clock() {
            self.uptime_secs = ms / 1000;
        }
    }

    // =========================================================================
    // DESENHO
    // =========================================================================

    /// Desenha a taskbar.
    pub fn draw(&mut self, buffer: &mut [u32], buffer_size: Size) {
        self.update_uptime();

        let style = GlassStyle::bar();

        // Desenhar as 3 barras
        GlassRenderer::draw_rect(buffer, buffer_size, self.widget_bar, &style);
        GlassRenderer::draw_rect(buffer, buffer_size, self.center_bar, &style);
        GlassRenderer::draw_rect(buffer, buffer_size, self.status_bar, &style);

        // Conteúdo das barras
        self.draw_widget_button(buffer, buffer_size);
        self.draw_center_content(buffer, buffer_size);
        self.draw_status_content(buffer, buffer_size);
    }

    /// Desenha botão de widgets.
    fn draw_widget_button(&self, buffer: &mut [u32], buffer_size: Size) {
        // Ícone de grid (4 quadrados)
        let icon_size = 20;
        let cx = self.widget_bar.x + (self.widget_bar.width as i32 - icon_size) / 2;
        let cy = self.widget_bar.y + (self.widget_bar.height as i32 - icon_size) / 2;

        let half = icon_size / 2 - 2;
        let gap = 3;
        let color = colors::ICON_NORMAL.as_u32();
        let stride = buffer_size.width as usize;

        // 4 quadrados
        Self::fill_rect_simple(
            buffer,
            stride,
            buffer_size,
            cx,
            cy,
            half as u32,
            half as u32,
            color,
        );
        Self::fill_rect_simple(
            buffer,
            stride,
            buffer_size,
            cx + half + gap,
            cy,
            half as u32,
            half as u32,
            color,
        );
        Self::fill_rect_simple(
            buffer,
            stride,
            buffer_size,
            cx,
            cy + half + gap,
            half as u32,
            half as u32,
            color,
        );
        Self::fill_rect_simple(
            buffer,
            stride,
            buffer_size,
            cx + half + gap,
            cy + half + gap,
            half as u32,
            half as u32,
            color,
        );
    }

    /// Desenha conteúdo central (ícone do menu + apps abertos).
    fn draw_center_content(&self, buffer: &mut [u32], buffer_size: Size) {
        let stride = buffer_size.width as usize;
        let padding = metrics::TASKBAR_PADDING as i32;

        // Ícone do menu (3 linhas horizontais)
        let menu_x = self.center_bar.x + padding;
        let menu_y = self.center_bar.y + (self.center_bar.height as i32 - 16) / 2;
        let color = colors::ICON_NORMAL.as_u32();

        // 3 linhas
        Self::fill_rect_simple(buffer, stride, buffer_size, menu_x, menu_y, 18, 2, color);
        Self::fill_rect_simple(
            buffer,
            stride,
            buffer_size,
            menu_x,
            menu_y + 6,
            18,
            2,
            color,
        );
        Self::fill_rect_simple(
            buffer,
            stride,
            buffer_size,
            menu_x,
            menu_y + 12,
            18,
            2,
            color,
        );

        // Separador
        let sep_x = menu_x + 28;
        Self::fill_rect_simple(
            buffer,
            stride,
            buffer_size,
            sep_x,
            self.center_bar.y + 8,
            1,
            self.center_bar.height - 16,
            colors::MENU_SEPARATOR.as_u32(),
        );

        // Ícones das janelas abertas
        let mut icon_x = sep_x + 12;
        let icon_size = 32u32;
        let icon_y = self.center_bar.y + (self.center_bar.height as i32 - icon_size as i32) / 2;

        for entry in &self.entries {
            if icon_x + icon_size as i32 > self.center_bar.right() - padding {
                break;
            }

            // Fundo do ícone
            let icon_color = if entry.minimized {
                colors::BG_MEDIUM.as_u32()
            } else {
                colors::GLASS_BG_ACTIVE.as_u32()
            };

            Self::fill_rect_simple(
                buffer,
                stride,
                buffer_size,
                icon_x,
                icon_y,
                icon_size,
                icon_size,
                icon_color,
            );

            // Indicador de ativo
            if !entry.minimized {
                Self::fill_rect_simple(
                    buffer,
                    stride,
                    buffer_size,
                    icon_x + 4,
                    icon_y + icon_size as i32 - 3,
                    icon_size - 8,
                    2,
                    colors::ACCENT.as_u32(),
                );
            }

            icon_x += icon_size as i32 + metrics::ICON_GAP as i32;
        }
    }

    /// Desenha conteúdo de status (uptime).
    fn draw_status_content(&self, buffer: &mut [u32], buffer_size: Size) {
        let stride = buffer_size.width as usize;

        // Formatar uptime como HH:MM:SS
        let hours = self.uptime_secs / 3600;
        let minutes = (self.uptime_secs % 3600) / 60;
        let seconds = self.uptime_secs % 60;

        // Desenhar cada dígito
        let text_y = self.status_bar.y + (self.status_bar.height as i32 - 12) / 2;
        let mut x = self.status_bar.x + 12;

        // HH
        self.draw_digit(buffer, stride, buffer_size, x, text_y, (hours / 10) as u8);
        x += 10;
        self.draw_digit(buffer, stride, buffer_size, x, text_y, (hours % 10) as u8);
        x += 12;

        // :
        Self::fill_rect_simple(
            buffer,
            stride,
            buffer_size,
            x,
            text_y + 2,
            2,
            2,
            colors::TEXT_PRIMARY.as_u32(),
        );
        Self::fill_rect_simple(
            buffer,
            stride,
            buffer_size,
            x,
            text_y + 8,
            2,
            2,
            colors::TEXT_PRIMARY.as_u32(),
        );
        x += 6;

        // MM
        self.draw_digit(buffer, stride, buffer_size, x, text_y, (minutes / 10) as u8);
        x += 10;
        self.draw_digit(buffer, stride, buffer_size, x, text_y, (minutes % 10) as u8);
        x += 12;

        // :
        Self::fill_rect_simple(
            buffer,
            stride,
            buffer_size,
            x,
            text_y + 2,
            2,
            2,
            colors::TEXT_PRIMARY.as_u32(),
        );
        Self::fill_rect_simple(
            buffer,
            stride,
            buffer_size,
            x,
            text_y + 8,
            2,
            2,
            colors::TEXT_PRIMARY.as_u32(),
        );
        x += 6;

        // SS
        self.draw_digit(buffer, stride, buffer_size, x, text_y, (seconds / 10) as u8);
        x += 10;
        self.draw_digit(buffer, stride, buffer_size, x, text_y, (seconds % 10) as u8);
    }

    /// Desenha um dígito simples (7-segment style).
    fn draw_digit(&self, buffer: &mut [u32], stride: usize, size: Size, x: i32, y: i32, digit: u8) {
        let color = colors::TEXT_PRIMARY.as_u32();
        let w = 6u32;
        let h = 12u32;
        let t = 2u32;

        // Segmentos: top, top-left, top-right, middle, bottom-left, bottom-right, bottom
        let segments: [u8; 10] = [
            0b1110111, // 0
            0b0010010, // 1
            0b1011101, // 2
            0b1011011, // 3
            0b0111010, // 4
            0b1101011, // 5
            0b1101111, // 6
            0b1010010, // 7
            0b1111111, // 8
            0b1111011, // 9
        ];

        let seg = segments[digit as usize % 10];

        if seg & 0b1000000 != 0 {
            // top
            Self::fill_rect_simple(buffer, stride, size, x, y, w, t, color);
        }
        if seg & 0b0100000 != 0 {
            // top-left
            Self::fill_rect_simple(buffer, stride, size, x, y, t, h / 2, color);
        }
        if seg & 0b0010000 != 0 {
            // top-right
            Self::fill_rect_simple(
                buffer,
                stride,
                size,
                x + w as i32 - t as i32,
                y,
                t,
                h / 2,
                color,
            );
        }
        if seg & 0b0001000 != 0 {
            // middle
            Self::fill_rect_simple(
                buffer,
                stride,
                size,
                x,
                y + h as i32 / 2 - t as i32 / 2,
                w,
                t,
                color,
            );
        }
        if seg & 0b0000100 != 0 {
            // bottom-left
            Self::fill_rect_simple(buffer, stride, size, x, y + h as i32 / 2, t, h / 2, color);
        }
        if seg & 0b0000010 != 0 {
            // bottom-right
            Self::fill_rect_simple(
                buffer,
                stride,
                size,
                x + w as i32 - t as i32,
                y + h as i32 / 2,
                t,
                h / 2,
                color,
            );
        }
        if seg & 0b0000001 != 0 {
            // bottom
            Self::fill_rect_simple(
                buffer,
                stride,
                size,
                x,
                y + h as i32 - t as i32,
                w,
                t,
                color,
            );
        }
    }

    // =========================================================================
    // INPUT
    // =========================================================================

    /// Processa clique.
    pub fn handle_click(&mut self, x: i32, y: i32) -> TaskbarAction {
        let point = Point::new(x, y);

        if self.widget_bar.contains_point(point) {
            return TaskbarAction::ToggleWidgetPanel;
        }

        if self.status_bar.contains_point(point) {
            return TaskbarAction::ToggleQuickSettings;
        }

        if self.center_bar.contains_point(point) {
            let padding = metrics::TASKBAR_PADDING as i32;
            let menu_area_end = self.center_bar.x + padding + 28;

            if x < menu_area_end {
                return TaskbarAction::ToggleStartMenu;
            }

            // Verificar clique em janela
            let sep_x = menu_area_end + 12;
            let icon_size = 32i32;
            let mut icon_x = sep_x;

            for entry in &self.entries {
                if x >= icon_x && x < icon_x + icon_size {
                    return TaskbarAction::ToggleWindow(entry.id);
                }
                icon_x += icon_size + metrics::ICON_GAP as i32;
            }
        }

        TaskbarAction::None
    }

    /// Verifica se ponto está sobre a taskbar.
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        let point = Point::new(x, y);
        self.widget_bar.contains_point(point)
            || self.center_bar.contains_point(point)
            || self.status_bar.contains_point(point)
    }

    // =========================================================================
    // HELPERS
    // =========================================================================

    fn fill_rect_simple(
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
