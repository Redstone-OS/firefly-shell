//! # Taskbar - Barra de Tarefas
//!
//! Componente visual da barra de tarefas na parte inferior da tela.
//!
//! ## Elementos
//!
//! - Botão Iniciar (canto esquerdo)
//! - Área de aplicações abertas (centro)
//! - System Tray + Relógio (canto direito - futuro)

use crate::theme::colors;
use redpowder::window::Window;

// ============================================================================
// CONSTANTES DE LAYOUT
// ============================================================================

/// Altura da taskbar em pixels
pub const HEIGHT: u32 = 40;

/// Largura do botão iniciar
pub const START_BUTTON_WIDTH: u32 = 48;

/// Tamanho dos ícones de aplicação
pub const APP_ICON_SIZE: u32 = 32;

/// Margem entre ícones
pub const ICON_MARGIN: u32 = 4;

/// Padding interno da taskbar
const PADDING: u32 = 4;

// ============================================================================
// TASKBAR
// ============================================================================

/// Componente da barra de tarefas.
pub struct Taskbar {
    /// Posição X
    pub x: u32,
    /// Posição Y
    pub y: u32,
    /// Largura
    pub width: u32,
    /// Altura
    pub height: u32,
}

impl Taskbar {
    /// Cria uma nova taskbar posicionada na parte inferior da tela.
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            x: 0,
            y: screen_height.saturating_sub(HEIGHT),
            width: screen_width,
            height: HEIGHT,
        }
    }

    /// Retorna a posição Y da taskbar.
    pub fn top(&self) -> u32 {
        self.y
    }

    /// Desenha a taskbar na janela.
    pub fn draw(&self, window: &mut Window) {
        // Fundo da taskbar
        window.fill_rect(self.x, self.y, self.width, self.height, colors::TASKBAR_BG);

        // Borda superior
        window.fill_rect(self.x, self.y, self.width, 1, colors::TASKBAR_BORDER);

        // Botão Iniciar
        self.draw_start_button(window);

        // Área de apps (placeholder)
        self.draw_app_button(window, 0);
    }

    /// Desenha o botão iniciar.
    fn draw_start_button(&self, window: &mut Window) {
        let btn_x = self.x + PADDING;
        let btn_y = self.y + PADDING;
        let btn_h = self.height - (PADDING * 2);

        // Fundo do botão
        window.fill_rect(
            btn_x,
            btn_y,
            START_BUTTON_WIDTH,
            btn_h,
            colors::START_BUTTON_BG,
        );

        // Ícone (4 quadrados - logo do Windows invertido)
        self.draw_windows_icon(window, btn_x, btn_y, btn_h);
    }

    /// Desenha o ícone de 4 quadrados do botão iniciar.
    fn draw_windows_icon(&self, window: &mut Window, btn_x: u32, btn_y: u32, btn_h: u32) {
        let icon_size = 16;
        let icon_x = btn_x + (START_BUTTON_WIDTH - icon_size) / 2;
        let icon_y = btn_y + (btn_h - icon_size) / 2;
        let half = icon_size / 2 - 1;
        let gap = 2;

        // 4 quadrados
        window.fill_rect(icon_x, icon_y, half, half, colors::WHITE);
        window.fill_rect(icon_x + half + gap, icon_y, half, half, colors::WHITE);
        window.fill_rect(icon_x, icon_y + half + gap, half, half, colors::WHITE);
        window.fill_rect(
            icon_x + half + gap,
            icon_y + half + gap,
            half,
            half,
            colors::WHITE,
        );
    }

    /// Desenha um botão de aplicação na taskbar.
    fn draw_app_button(&self, window: &mut Window, index: u32) {
        let start_x = self.x + START_BUTTON_WIDTH + 12;
        let btn_x = start_x + (index * (APP_ICON_SIZE + ICON_MARGIN + 8));
        let btn_y = self.y + PADDING;
        let btn_h = self.height - (PADDING * 2);
        let btn_w = APP_ICON_SIZE + 8;

        // Fundo do botão
        window.fill_rect(btn_x, btn_y, btn_w, btn_h, colors::APP_BUTTON_ACTIVE);

        // Ícone placeholder (borda de janela)
        self.draw_window_icon(window, btn_x + 4, btn_y + 4, btn_h - 8);
    }

    /// Desenha um ícone placeholder de janela.
    fn draw_window_icon(&self, window: &mut Window, x: u32, y: u32, size: u32) {
        let border_width = 2;

        // Borda da janela
        window.fill_rect(x, y, size, border_width, colors::WHITE); // Top
        window.fill_rect(x, y, border_width, size, colors::WHITE); // Left
        window.fill_rect(
            x + size - border_width,
            y,
            border_width,
            size,
            colors::WHITE,
        ); // Right
        window.fill_rect(
            x,
            y + size - border_width,
            size,
            border_width,
            colors::WHITE,
        ); // Bottom
    }

    /// Retorna a área de trabalho disponível (excluindo a taskbar).
    pub fn get_work_area(&self, screen_width: u32) -> (u32, u32, u32, u32) {
        (0, 0, screen_width, self.y)
    }
}
