//! # Taskbar - Barra de Tarefas
//!
//! Componente visual da barra de tarefas na parte inferior da tela.
//!
//! ## Elementos
//!
//! - Botão Iniciar (canto esquerdo)
//! - Área de aplicações abertas (centro)
//! - System Tray + Relógio (canto direito)
//!
//! ## v0.2.0 Melhorias
//!
//! - Relógio mostra uptime real
//! - Menu lista apps descobertos
//! - Ícones das janelas abertas

use crate::app::{AppIcon, AppInfo};
use crate::theme::colors;
use alloc::string::String;
use alloc::vec::Vec;
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

/// Largura da área de system tray (HH:MM:SS + ícones)
const SYSTEM_TRAY_WIDTH: u32 = 130;

/// Altura de um item no menu
const MENU_ITEM_HEIGHT: u32 = 36;

// ============================================================================
// TIPOS
// ============================================================================

/// Ação retornada pelo tratamento de clique
pub enum TaskbarAction {
    None,
    ToggleStartMenu,
    ToggleWindow(u32),
    LaunchApp(usize), // Índice do app no menu
}

/// Entrada de janela na taskbar
struct WindowEntry {
    id: u32,
    title: String,
    minimized: bool,
}

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
    /// Menu iniciar aberto
    pub start_menu_open: bool,
    /// Janelas abertas
    entries: Vec<WindowEntry>,
    /// Apps disponíveis (para o menu)
    pub available_apps: Vec<AppInfo>,
    /// Item hover no menu (-1 se nenhum)
    menu_hover_index: isize,
    /// Último uptime em segundos
    cached_uptime_secs: u64,
}

impl Taskbar {
    /// Cria uma nova taskbar posicionada na parte inferior da tela.
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            x: 0,
            y: screen_height.saturating_sub(HEIGHT),
            width: screen_width,
            height: HEIGHT,
            start_menu_open: false,
            entries: Vec::new(),
            available_apps: Vec::new(),
            menu_hover_index: -1,
            cached_uptime_secs: 0,
        }
    }

    /// Define os apps disponíveis
    pub fn set_available_apps(&mut self, apps: Vec<AppInfo>) {
        self.available_apps = apps;
    }

    /// Adiciona janela à taskbar
    pub fn add_window(&mut self, id: u32, title: String) {
        if !self.entries.iter().any(|e| e.id == id) {
            self.entries.push(WindowEntry {
                id,
                title,
                minimized: false,
            });
        }
    }

    /// Remove janela da taskbar
    pub fn remove_window(&mut self, id: u32) {
        self.entries.retain(|e| e.id != id);
    }

    /// Define estado de minimizado
    pub fn set_window_minimized(&mut self, id: u32, minimized: bool) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.minimized = minimized;
        }
    }

    /// Obtém estado de minimizado
    pub fn get_window_state(&self, id: u32) -> Option<bool> {
        self.entries
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.minimized)
    }

    /// Atualiza o uptime cache
    pub fn update_uptime(&mut self) {
        if let Ok(ms) = redpowder::time::clock() {
            self.cached_uptime_secs = ms / 1000;
        }
    }

    // ========================================================================
    // DESENHO
    // ========================================================================

    /// Desenha a taskbar na janela.
    pub fn draw(&mut self, window: &mut Window) {
        // Atualizar uptime
        self.update_uptime();

        // Fundo da taskbar
        window.fill_rect(self.x, self.y, self.width, self.height, colors::TASKBAR_BG);

        // Borda superior
        window.fill_rect(self.x, self.y, self.width, 1, colors::TASKBAR_BORDER);

        // Botão Iniciar
        self.draw_start_button(window);

        // Ícones das janelas abertas
        for (i, entry) in self.entries.iter().enumerate() {
            self.draw_app_button(window, i as u32, entry);
        }

        // System Tray
        self.draw_system_tray(window);
    }

    /// Desenha o botão iniciar
    fn draw_start_button(&self, window: &mut Window) {
        let btn_x = self.x + PADDING;
        let btn_y = self.y + PADDING;
        let btn_h = self.height - (PADDING * 2);

        // Cor muda se menu aberto
        let bg = if self.start_menu_open {
            colors::START_BUTTON_ACTIVE
        } else {
            colors::START_BUTTON_BG
        };

        window.fill_rect(btn_x, btn_y, START_BUTTON_WIDTH, btn_h, bg);

        // Ícone de 4 quadrados
        let icon_size = 16;
        let icon_x = btn_x + (START_BUTTON_WIDTH - icon_size) / 2;
        let icon_y = btn_y + (btn_h - icon_size) / 2;
        let half = icon_size / 2 - 1;
        let gap = 2;

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

    /// Desenha um ícone de app na taskbar
    fn draw_app_button(&self, window: &mut Window, index: u32, entry: &WindowEntry) {
        let start_x = self.x + START_BUTTON_WIDTH + 12;
        let btn_x = start_x + (index * (APP_ICON_SIZE + ICON_MARGIN + 8));
        let btn_y = self.y + PADDING;
        let btn_h = self.height - (PADDING * 2);
        let btn_w = APP_ICON_SIZE + 8;

        let bg_color = if entry.minimized {
            colors::TASKBAR_BG
        } else {
            colors::APP_BUTTON_ACTIVE
        };
        window.fill_rect(btn_x, btn_y, btn_w, btn_h, bg_color);

        // Indicador de estado
        if !entry.minimized {
            window.fill_rect(btn_x + 4, btn_y + btn_h - 3, btn_w - 8, 2, colors::ACCENT);
        } else {
            window.fill_rect(
                btn_x + 8,
                btn_y + btn_h - 2,
                btn_w - 16,
                1,
                colors::TEXT_SECONDARY,
            );
        }

        // Ícone (baseado no título)
        self.draw_app_icon_by_title(window, btn_x + 4, btn_y + 4, btn_h - 8, &entry.title);
    }

    /// Desenha ícone baseado no título do app
    fn draw_app_icon_by_title(&self, window: &mut Window, x: u32, y: u32, size: u32, title: &str) {
        let title_lower = title.to_lowercase();

        if title_lower.contains("terminal") {
            self.draw_terminal_icon(window, x, y, size);
        } else if title_lower.contains("settings") || title_lower.contains("config") {
            self.draw_settings_icon(window, x, y, size);
        } else {
            self.draw_generic_icon(window, x, y, size);
        }
    }

    /// Desenha ícone de terminal
    fn draw_terminal_icon(&self, window: &mut Window, x: u32, y: u32, size: u32) {
        window.fill_rect(x, y, size, size, colors::BLACK);
        window.fill_rect(x, y, size, 2, colors::WHITE);
        window.fill_rect(x, y, 2, size, colors::WHITE);
        window.fill_rect(x + size - 2, y, 2, size, colors::WHITE);
        window.fill_rect(x, y + size - 2, size, 2, colors::WHITE);

        // Prompt
        let prompt_x = x + 4;
        let prompt_y = y + size / 2 - 2;
        window.fill_rect(prompt_x, prompt_y, 4, 4, colors::GREEN);
        window.fill_rect(prompt_x + 6, prompt_y + 1, 8, 2, colors::GREEN);
    }

    /// Desenha ícone de configurações
    fn draw_settings_icon(&self, window: &mut Window, x: u32, y: u32, size: u32) {
        let center_x = x + size / 2;
        let center_y = y + size / 2;
        let r = size / 3;

        // Círculo central
        window.fill_rect(center_x - r / 2, center_y - r / 2, r, r, colors::WHITE);

        // "Dentes" da engrenagem (simplificado)
        window.fill_rect(center_x - 1, y + 2, 2, 4, colors::WHITE);
        window.fill_rect(center_x - 1, y + size - 6, 2, 4, colors::WHITE);
        window.fill_rect(x + 2, center_y - 1, 4, 2, colors::WHITE);
        window.fill_rect(x + size - 6, center_y - 1, 4, 2, colors::WHITE);
    }

    /// Desenha ícone genérico
    fn draw_generic_icon(&self, window: &mut Window, x: u32, y: u32, size: u32) {
        window.fill_rect(x, y, size, size, colors::ACCENT);
        window.fill_rect(x + 4, y + 4, size - 8, size - 8, colors::WHITE);
    }

    /// Desenha a área de system tray
    fn draw_system_tray(&self, window: &mut Window) {
        let tray_x = self.width - SYSTEM_TRAY_WIDTH - PADDING;
        let tray_y = self.y + PADDING;
        let tray_h = self.height - (PADDING * 2);

        // Separador
        window.fill_rect(
            tray_x - 4,
            tray_y + 4,
            1,
            tray_h - 8,
            colors::TASKBAR_BORDER,
        );

        // Ícone de rede
        self.draw_network_icon(window, tray_x, tray_y + (tray_h - 16) / 2);

        // Ícone de som
        self.draw_sound_icon(window, tray_x + 24, tray_y + (tray_h - 16) / 2);

        // Relógio com uptime
        self.draw_uptime_clock(window, tray_x + 48, tray_y, tray_h);
    }

    /// Desenha ícone de rede
    fn draw_network_icon(&self, window: &mut Window, x: u32, y: u32) {
        window.fill_rect(x + 6, y + 12, 4, 4, colors::WHITE);
        window.fill_rect(x + 4, y + 8, 8, 2, colors::WHITE);
        window.fill_rect(x + 2, y + 4, 12, 2, colors::WHITE);
        window.fill_rect(x, y, 16, 2, colors::WHITE);
    }

    /// Desenha ícone de som
    fn draw_sound_icon(&self, window: &mut Window, x: u32, y: u32) {
        window.fill_rect(x + 2, y + 5, 4, 6, colors::WHITE);
        window.fill_rect(x + 6, y + 3, 2, 10, colors::WHITE);
        window.fill_rect(x + 10, y + 4, 2, 8, colors::TEXT_SECONDARY);
        window.fill_rect(x + 14, y + 2, 2, 12, colors::TEXT_SECONDARY);
    }

    /// Desenha relógio mostrando uptime HH:MM:SS
    fn draw_uptime_clock(&self, window: &mut Window, x: u32, y: u32, h: u32) {
        let clock_y = y + (h - 14) / 2;

        let total_secs = self.cached_uptime_secs;
        let hours = (total_secs / 3600) % 100;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;

        // Formatar HH:MM:SS
        let h1 = (hours / 10) as u8;
        let h2 = (hours % 10) as u8;
        let m1 = (mins / 10) as u8;
        let m2 = (mins % 10) as u8;
        let s1 = (secs / 10) as u8;
        let s2 = (secs % 10) as u8;

        let mut offset = 0;

        // Hora
        self.draw_digit(window, x + offset, clock_y, h1);
        offset += 8;
        self.draw_digit(window, x + offset, clock_y, h2);
        offset += 8;

        // :
        window.fill_rect(x + offset + 1, clock_y + 3, 2, 2, colors::WHITE);
        window.fill_rect(x + offset + 1, clock_y + 9, 2, 2, colors::WHITE);
        offset += 6;

        // Minuto
        self.draw_digit(window, x + offset, clock_y, m1);
        offset += 8;
        self.draw_digit(window, x + offset, clock_y, m2);
        offset += 8;

        // :
        window.fill_rect(x + offset + 1, clock_y + 3, 2, 2, colors::WHITE);
        window.fill_rect(x + offset + 1, clock_y + 9, 2, 2, colors::WHITE);
        offset += 6;

        // Segundos
        self.draw_digit(window, x + offset, clock_y, s1);
        offset += 8;
        self.draw_digit(window, x + offset, clock_y, s2);
    }

    /// Desenha um dígito 0-9 (5x7 pixels simplificado)
    fn draw_digit(&self, window: &mut Window, x: u32, y: u32, digit: u8) {
        let patterns: [[u8; 7]; 10] = [
            [0b111, 0b101, 0b101, 0b101, 0b101, 0b101, 0b111], // 0
            [0b010, 0b110, 0b010, 0b010, 0b010, 0b010, 0b111], // 1
            [0b111, 0b001, 0b001, 0b111, 0b100, 0b100, 0b111], // 2
            [0b111, 0b001, 0b001, 0b111, 0b001, 0b001, 0b111], // 3
            [0b101, 0b101, 0b101, 0b111, 0b001, 0b001, 0b001], // 4
            [0b111, 0b100, 0b100, 0b111, 0b001, 0b001, 0b111], // 5
            [0b111, 0b100, 0b100, 0b111, 0b101, 0b101, 0b111], // 6
            [0b111, 0b001, 0b001, 0b010, 0b010, 0b010, 0b010], // 7
            [0b111, 0b101, 0b101, 0b111, 0b101, 0b101, 0b111], // 8
            [0b111, 0b101, 0b101, 0b111, 0b001, 0b001, 0b111], // 9
        ];

        let pattern = if digit < 10 {
            &patterns[digit as usize]
        } else {
            &patterns[0]
        };

        for (row, &bits) in pattern.iter().enumerate() {
            for col in 0..3 {
                if (bits >> (2 - col)) & 1 == 1 {
                    window.fill_rect(x + col * 2, y + row as u32 * 2, 2, 2, colors::WHITE);
                }
            }
        }
    }

    // ========================================================================
    // MENU CONTENT
    // ========================================================================

    /// Desenha o conteúdo do menu iniciar
    pub fn draw_menu_content(&self, window: &mut Window) {
        let w = window.width;
        let h = window.height;

        // Fundo
        window.fill_rect(0, 0, w, h, colors::MENU_BG);

        // Borda
        window.fill_rect(0, 0, w, 1, colors::TASKBAR_BORDER);
        window.fill_rect(0, 0, 1, h, colors::TASKBAR_BORDER);
        window.fill_rect(w - 1, 0, 1, h, colors::TASKBAR_BORDER);
        window.fill_rect(0, h - 1, w, 1, colors::TASKBAR_BORDER);

        // Título "RedstoneOS"
        self.draw_menu_title(window, 16, 12);

        // Separador
        window.fill_rect(16, 42, w - 32, 1, colors::TASKBAR_BORDER);

        // Lista de apps
        let apps_start_y = 50;
        for (i, app) in self.available_apps.iter().enumerate() {
            let item_y = apps_start_y + (i as u32 * MENU_ITEM_HEIGHT);
            if item_y + MENU_ITEM_HEIGHT > h - 10 {
                break;
            }
            let hover = self.menu_hover_index == i as isize;
            self.draw_menu_item(window, 8, item_y, &app.name, app.icon, hover);
        }

        // Se não houver apps
        if self.available_apps.is_empty() {
            window.fill_rect(16, apps_start_y + 10, 100, 4, colors::TEXT_SECONDARY);
        }
    }

    /// Desenha título do menu
    fn draw_menu_title(&self, window: &mut Window, x: u32, y: u32) {
        // "R" estilizado
        window.fill_rect(x, y, 4, 20, colors::ACCENT);
        window.fill_rect(x, y, 12, 4, colors::ACCENT);
        window.fill_rect(x + 8, y + 4, 4, 6, colors::ACCENT);
        window.fill_rect(x, y + 8, 12, 4, colors::ACCENT);
        window.fill_rect(x + 8, y + 12, 4, 8, colors::ACCENT);

        // "edstone" simplificado como linha
        window.fill_rect(x + 20, y + 8, 60, 4, colors::WHITE);
    }

    /// Desenha um item do menu
    fn draw_menu_item(
        &self,
        window: &mut Window,
        x: u32,
        y: u32,
        name: &str,
        icon: AppIcon,
        hover: bool,
    ) {
        let item_w = window.width - 16;

        // Fundo
        let bg = if hover {
            colors::MENU_ITEM_HOVER
        } else {
            colors::MENU_BG
        };
        window.fill_rect(x, y, item_w, MENU_ITEM_HEIGHT, bg);

        // Ícone
        self.draw_menu_icon(window, x + 8, y + 8, 20, icon);

        // Nome (simplificado como linha)
        let text_color = colors::WHITE;
        let name_len = name.len().min(15) as u32;
        window.fill_rect(x + 36, y + 14, name_len * 6, 4, text_color);

        // Primeira letra como indicador
        if !name.is_empty() {
            window.fill_rect(x + 36, y + 10, 4, 12, text_color);
        }
    }

    /// Desenha ícone no menu
    fn draw_menu_icon(&self, window: &mut Window, x: u32, y: u32, size: u32, icon: AppIcon) {
        match icon {
            AppIcon::Terminal => {
                window.fill_rect(x, y, size, size, colors::BLACK);
                window.fill_rect(x + 2, y + 2, size - 4, size - 4, colors::BLACK);
                window.fill_rect(x + 4, y + size / 2 - 1, 6, 2, colors::GREEN);
            }
            AppIcon::Settings => {
                let c = size / 2;
                window.fill_rect(x + c - 4, y + c - 4, 8, 8, colors::TEXT_SECONDARY);
                window.fill_rect(x + c - 1, y, 2, size, colors::TEXT_SECONDARY);
                window.fill_rect(x, y + c - 1, size, 2, colors::TEXT_SECONDARY);
            }
            AppIcon::Game => {
                window.fill_rect(x + 2, y + 4, size - 4, size - 8, colors::ACCENT);
                window.fill_rect(x + 6, y + 8, 4, 4, colors::WHITE);
            }
            _ => {
                window.fill_rect(x + 2, y + 2, size - 4, size - 4, colors::ACCENT);
            }
        }
    }

    // ========================================================================
    // EVENTOS
    // ========================================================================

    /// Processa clique na taskbar
    pub fn handle_click(&mut self, click_x: i32, click_y: i32) -> TaskbarAction {
        // Verificar se está na taskbar
        if click_y < self.y as i32 || click_y >= (self.y + self.height) as i32 {
            return TaskbarAction::None;
        }

        // Botão iniciar
        let start_btn_x = self.x + PADDING;
        let start_btn_right = start_btn_x + START_BUTTON_WIDTH;

        if click_x >= start_btn_x as i32 && click_x < start_btn_right as i32 {
            self.start_menu_open = !self.start_menu_open;
            return TaskbarAction::ToggleStartMenu;
        }

        // Ícones de apps
        let apps_start_x = self.x + START_BUTTON_WIDTH + 12;
        let btn_w = APP_ICON_SIZE + 8;
        let btn_gap = APP_ICON_SIZE + ICON_MARGIN + 8;

        for (i, entry) in self.entries.iter().enumerate() {
            let btn_x = apps_start_x + (i as u32 * btn_gap);
            if click_x >= btn_x as i32 && click_x < (btn_x + btn_w) as i32 {
                return TaskbarAction::ToggleWindow(entry.id);
            }
        }

        TaskbarAction::None
    }

    /// Processa clique no menu
    pub fn handle_menu_click(
        &mut self,
        click_x: i32,
        click_y: i32,
        menu_height: u32,
    ) -> TaskbarAction {
        // Verificar se está na área de itens
        let apps_start_y = 50i32;
        let item_x = 8i32;
        let item_w = 180i32;

        for (i, _app) in self.available_apps.iter().enumerate() {
            let item_y = apps_start_y + (i as i32 * MENU_ITEM_HEIGHT as i32);

            if click_x >= item_x
                && click_x < item_x + item_w
                && click_y >= item_y
                && click_y < item_y + MENU_ITEM_HEIGHT as i32
            {
                self.start_menu_open = false;
                return TaskbarAction::LaunchApp(i);
            }
        }

        TaskbarAction::None
    }

    /// Retorna área de trabalho
    #[allow(dead_code)]
    pub fn get_work_area(&self, screen_width: u32) -> (u32, u32, u32, u32) {
        (0, 0, screen_width, self.y)
    }
}
