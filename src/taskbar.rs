//! # Taskbar - Firefly Shell
//!
//! Barra de tarefas na parte inferior da tela.

use redpowder::graphics::{Color, Framebuffer};

// ============================================================================
// CONSTANTES
// ============================================================================

/// Altura da taskbar
pub const TASKBAR_HEIGHT: u32 = 40;

/// Largura do botão do menu iniciar
pub const START_BUTTON_WIDTH: u32 = 48;

/// Tamanho dos ícones de app na taskbar
pub const APP_ICON_SIZE: u32 = 32;

/// Margem entre ícones
pub const ICON_MARGIN: u32 = 4;

// Cores
pub const TASKBAR_BG_COLOR: Color = Color::rgb(30, 30, 40);
pub const TASKBAR_BORDER_COLOR: Color = Color::rgb(60, 60, 80);
pub const START_BUTTON_COLOR: Color = Color::rgb(70, 130, 180);
pub const APP_BUTTON_ACTIVE: Color = Color::rgb(70, 70, 90);

// ============================================================================
// TASKBAR
// ============================================================================

pub struct Taskbar {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Taskbar {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            x: 0,
            y: screen_height - TASKBAR_HEIGHT,
            width: screen_width,
            height: TASKBAR_HEIGHT,
        }
    }

    pub fn draw(&self, fb: &mut Framebuffer) {
        // Fundo
        let _ = fb.fill_rect(self.x, self.y, self.width, self.height, TASKBAR_BG_COLOR);
        // Borda
        let _ = fb.fill_rect(self.x, self.y, self.width, 1, TASKBAR_BORDER_COLOR);

        // Botão Iniciar
        self.draw_start_button(fb);

        // Placeholder app
        self.draw_app_placeholder(fb, 0, "Term");
    }

    fn draw_start_button(&self, fb: &mut Framebuffer) {
        let btn_x = self.x + 4;
        let btn_y = self.y + 4;
        let btn_h = self.height - 8;
        let _ = fb.fill_rect(btn_x, btn_y, START_BUTTON_WIDTH, btn_h, START_BUTTON_COLOR);

        // Icone (quadradinhos)
        let icon_size = 16u32;
        let icon_x = btn_x + (START_BUTTON_WIDTH - icon_size) / 2;
        let icon_y = btn_y + (btn_h - icon_size) / 2;
        let half = icon_size / 2 - 1;

        let _ = fb.fill_rect(icon_x, icon_y, half, half, Color::WHITE);
        let _ = fb.fill_rect(icon_x + half + 2, icon_y, half, half, Color::WHITE);
        let _ = fb.fill_rect(icon_x, icon_y + half + 2, half, half, Color::WHITE);
        let _ = fb.fill_rect(
            icon_x + half + 2,
            icon_y + half + 2,
            half,
            half,
            Color::WHITE,
        );
    }

    fn draw_app_placeholder(&self, fb: &mut Framebuffer, index: u32, _name: &str) {
        let start_x = self.x + START_BUTTON_WIDTH + 12;
        let btn_x = start_x + index * (APP_ICON_SIZE + ICON_MARGIN + 8);
        let btn_y = self.y + 4;
        let btn_h = self.height - 8;
        let btn_w = APP_ICON_SIZE + 8;

        let _ = fb.fill_rect(btn_x, btn_y, btn_w, btn_h, APP_BUTTON_ACTIVE);

        // Simples ícone
        let icon_x = btn_x + 4;
        let icon_y = btn_y + 4;
        let icon_size = btn_h - 8;
        let _ = fb.fill_rect(icon_x, icon_y, icon_size, 2, Color::WHITE);
        let _ = fb.fill_rect(icon_x, icon_y, 2, icon_size, Color::WHITE);
        let _ = fb.fill_rect(icon_x + icon_size - 2, icon_y, 2, icon_size, Color::WHITE);
        let _ = fb.fill_rect(icon_x, icon_y + icon_size - 2, icon_size, 2, Color::WHITE);
    }

    pub fn get_work_area(&self, screen_width: u32, screen_height: u32) -> (u32, u32, u32, u32) {
        (0, 0, screen_width, screen_height - self.height)
    }
}
