//! # Taskbar - Firefly Shell
//!
//! Barra de tarefas na parte inferior da tela.

use redpowder::window::Window;

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

// Cores (ARGB u32)
pub const TASKBAR_BG_COLOR: u32 = 0xFF1E1E28;
pub const TASKBAR_BORDER_COLOR: u32 = 0xFF3C3C50;
pub const START_BUTTON_COLOR: u32 = 0xFF4682B4;
pub const APP_BUTTON_ACTIVE: u32 = 0xFF46465A;
pub const WHITE: u32 = 0xFFFFFFFF;

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

    pub fn draw(&self, fb: &mut Window) {
        // Fundo
        fb.fill_rect(self.x, self.y, self.width, self.height, TASKBAR_BG_COLOR);
        // Borda
        fb.fill_rect(self.x, self.y, self.width, 1, TASKBAR_BORDER_COLOR);

        // Botão Iniciar
        self.draw_start_button(fb);

        // Placeholder app
        self.draw_app_placeholder(fb, 0, "Term");
    }

    fn draw_start_button(&self, fb: &mut Window) {
        let btn_x = self.x + 4;
        let btn_y = self.y + 4;
        let btn_h = self.height - 8;
        fb.fill_rect(btn_x, btn_y, START_BUTTON_WIDTH, btn_h, START_BUTTON_COLOR);

        // Icone (quadradinhos)
        let icon_size = 16u32;
        // Assegura cálculo u32 (width > size, então sem underflow)
        let icon_x = btn_x + (START_BUTTON_WIDTH - icon_size) / 2;
        let icon_y = btn_y + (btn_h - icon_size) / 2;
        let half = icon_size / 2 - 1; // 7px

        fb.fill_rect(icon_x, icon_y, half, half, WHITE);
        fb.fill_rect(icon_x + half + 2, icon_y, half, half, WHITE);
        fb.fill_rect(icon_x, icon_y + half + 2, half, half, WHITE);
        fb.fill_rect(icon_x + half + 2, icon_y + half + 2, half, half, WHITE);
    }

    fn draw_app_placeholder(&self, fb: &mut Window, index: u32, _name: &str) {
        let start_x = self.x + START_BUTTON_WIDTH + 12;
        let btn_x = start_x + (index * (APP_ICON_SIZE + ICON_MARGIN + 8));
        let btn_y = self.y + 4;
        let btn_h = self.height - 8;
        let btn_w = APP_ICON_SIZE + 8;

        fb.fill_rect(btn_x, btn_y, btn_w, btn_h, APP_BUTTON_ACTIVE);

        // Simples ícone
        let icon_x = btn_x + 4;
        let icon_y = btn_y + 4;
        let icon_size = btn_h - 8;
        fb.fill_rect(icon_x, icon_y, icon_size, 2, WHITE);
        fb.fill_rect(icon_x, icon_y, 2, icon_size, WHITE);
        fb.fill_rect(icon_x + icon_size - 2, icon_y, 2, icon_size, WHITE);
        fb.fill_rect(icon_x, icon_y + icon_size - 2, icon_size, 2, WHITE);
    }

    pub fn get_work_area(&self, screen_width: u32, screen_height: u32) -> (u32, u32, u32, u32) {
        (0, 0, screen_width, screen_height - self.height)
    }
}
