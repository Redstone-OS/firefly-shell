//! # Taskbar - Barra de Tarefas
//!
//! Componente visual da barra de tarefas na parte inferior da tela.
//!
//! ## Elementos
//!
//! - Botão Iniciar (canto esquerdo)
//! - Área de aplicações abertas (centro)
//! - System Tray + Relógio (canto direito)

use crate::theme::colors;
use alloc::string::String;
use alloc::vec::Vec;
use redpowder::println;
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

/// Largura da área de system tray
const SYSTEM_TRAY_WIDTH: u32 = 120;

// ============================================================================
// TASKBAR
// ============================================================================

pub enum TaskbarAction {
    None,
    ToggleStartMenu,
    ToggleWindow(u32), // Minimizar/Restaurar
}

struct WindowEntry {
    id: u32,
    title: String,
    minimized: bool,
}

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
        }
    }

    pub fn add_window(&mut self, id: u32, title: String) {
        // Evitar duplicatas
        if !self.entries.iter().any(|e| e.id == id) {
            self.entries.push(WindowEntry {
                id,
                title,
                minimized: false,
            });
        }
    }

    pub fn remove_window(&mut self, id: u32) {
        self.entries.retain(|e| e.id != id);
    }

    pub fn set_window_minimized(&mut self, id: u32, minimized: bool) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.minimized = minimized;
        }
    }

    pub fn get_window_state(&self, id: u32) -> Option<bool> {
        self.entries
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.minimized)
    }

    /// Retorna a posição Y da taskbar.
    #[allow(dead_code)]
    pub fn top(&self) -> u32 {
        self.y
    }

    /// Desenha a taskbar na janela.
    pub fn draw(&self, window: &mut Window) {
        // Fundo da taskbar (com leve transparência simulada)
        window.fill_rect(self.x, self.y, self.width, self.height, colors::TASKBAR_BG);

        // Borda superior (linha de destaque)
        window.fill_rect(self.x, self.y, self.width, 1, colors::TASKBAR_BORDER);

        // Botão Iniciar
        self.draw_start_button(window);

        // Área de apps
        for (i, entry) in self.entries.iter().enumerate() {
            self.draw_app_button(window, i as u32, entry);
        }

        // System Tray (canto direito)
        self.draw_system_tray(window);
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

        // Ícone (4 quadrados - logo estilo Windows)
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
    fn draw_app_button(&self, window: &mut Window, index: u32, entry: &WindowEntry) {
        let start_x = self.x + START_BUTTON_WIDTH + 12;
        let btn_x = start_x + (index * (APP_ICON_SIZE + ICON_MARGIN + 8));
        let btn_y = self.y + PADDING;
        let btn_h = self.height - (PADDING * 2);
        let btn_w = APP_ICON_SIZE + 8;

        // Fundo do botão (alterar se minimizado)
        let bg_color = if entry.minimized {
            colors::TASKBAR_BG // Transparente/Escuro se minimizado
        } else {
            colors::APP_BUTTON_ACTIVE // Destacado se ativo
        };
        window.fill_rect(btn_x, btn_y, btn_w, btn_h, bg_color);

        // Indicador de ativo (linha inferior colorida)
        if !entry.minimized {
            window.fill_rect(btn_x + 4, btn_y + btn_h - 3, btn_w - 8, 2, colors::ACCENT);
        } else {
            // Indicador discreto para minimizado
            window.fill_rect(
                btn_x + 8,
                btn_y + btn_h - 2,
                btn_w - 16,
                1,
                colors::TEXT_SECONDARY,
            );
        }

        // Ícone placeholder (terminal) - TODO: Usar título para decidir ícone
        self.draw_terminal_icon(window, btn_x + 4, btn_y + 4, btn_h - 8);
    }

    /// Desenha ícone de terminal.
    fn draw_terminal_icon(&self, window: &mut Window, x: u32, y: u32, size: u32) {
        // Fundo preto
        window.fill_rect(x, y, size, size, colors::BLACK);

        // Borda branca
        window.fill_rect(x, y, size, 2, colors::WHITE);
        window.fill_rect(x, y, 2, size, colors::WHITE);
        window.fill_rect(x + size - 2, y, 2, size, colors::WHITE);
        window.fill_rect(x, y + size - 2, size, 2, colors::WHITE);

        // Prompt ">" simplificado
        let prompt_x = x + 4;
        let prompt_y = y + size / 2 - 2;
        window.fill_rect(prompt_x, prompt_y, 4, 4, colors::GREEN);
        window.fill_rect(prompt_x + 6, prompt_y + 1, 8, 2, colors::GREEN);
    }

    /// Desenha a área de system tray (canto direito).
    fn draw_system_tray(&self, window: &mut Window) {
        let tray_x = self.width - SYSTEM_TRAY_WIDTH - PADDING;
        let tray_y = self.y + PADDING;
        let tray_h = self.height - (PADDING * 2);

        // Separador vertical
        window.fill_rect(
            tray_x - 4,
            tray_y + 4,
            1,
            tray_h - 8,
            colors::TASKBAR_BORDER,
        );

        // Ícone de rede (Wi-Fi simplificado)
        self.draw_network_icon(window, tray_x, tray_y + (tray_h - 16) / 2);

        // Ícone de som
        self.draw_sound_icon(window, tray_x + 24, tray_y + (tray_h - 16) / 2);

        // Relógio
        self.draw_clock(window, tray_x + 52, tray_y, tray_h);
    }

    /// Desenha ícone de rede (Wi-Fi).
    fn draw_network_icon(&self, window: &mut Window, x: u32, y: u32) {
        // Arcos de Wi-Fi (simplificado como barras)
        window.fill_rect(x + 6, y + 12, 4, 4, colors::WHITE);
        window.fill_rect(x + 4, y + 8, 8, 2, colors::WHITE);
        window.fill_rect(x + 2, y + 4, 12, 2, colors::WHITE);
        window.fill_rect(x, y, 16, 2, colors::WHITE);
    }

    /// Desenha ícone de som.
    fn draw_sound_icon(&self, window: &mut Window, x: u32, y: u32) {
        // Alto-falante simplificado
        window.fill_rect(x + 2, y + 5, 4, 6, colors::WHITE);
        window.fill_rect(x + 6, y + 3, 2, 10, colors::WHITE);

        // Ondas de som
        window.fill_rect(x + 10, y + 4, 2, 8, colors::TEXT_SECONDARY);
        window.fill_rect(x + 14, y + 2, 2, 12, colors::TEXT_SECONDARY);
    }

    /// Desenha o relógio.
    fn draw_clock(&self, window: &mut Window, x: u32, y: u32, h: u32) {
        // Fundo do relógio
        let clock_y = y + (h - 16) / 2;

        // Desenhar "12:00" como pixels simplificados
        // "1"
        window.fill_rect(x, clock_y + 2, 2, 12, colors::WHITE);

        // "2"
        window.fill_rect(x + 6, clock_y, 6, 2, colors::WHITE);
        window.fill_rect(x + 10, clock_y + 2, 2, 4, colors::WHITE);
        window.fill_rect(x + 6, clock_y + 6, 6, 2, colors::WHITE);
        window.fill_rect(x + 6, clock_y + 8, 2, 4, colors::WHITE);
        window.fill_rect(x + 6, clock_y + 12, 6, 2, colors::WHITE);

        // ":"
        window.fill_rect(x + 16, clock_y + 3, 2, 2, colors::WHITE);
        window.fill_rect(x + 16, clock_y + 9, 2, 2, colors::WHITE);

        // "0" (primeiro)
        window.fill_rect(x + 22, clock_y, 6, 2, colors::WHITE);
        window.fill_rect(x + 22, clock_y, 2, 14, colors::WHITE);
        window.fill_rect(x + 26, clock_y, 2, 14, colors::WHITE);
        window.fill_rect(x + 22, clock_y + 12, 6, 2, colors::WHITE);

        // "0" (segundo)
        window.fill_rect(x + 32, clock_y, 6, 2, colors::WHITE);
        window.fill_rect(x + 32, clock_y, 2, 14, colors::WHITE);
        window.fill_rect(x + 36, clock_y, 2, 14, colors::WHITE);
        window.fill_rect(x + 32, clock_y + 12, 6, 2, colors::WHITE);
    }

    /// Desenha título do menu.
    fn draw_menu_title(&self, window: &mut Window, x: u32, y: u32) {
        // "R" simplificado (logo)
        window.fill_rect(x, y, 4, 20, colors::ACCENT);
        window.fill_rect(x, y, 12, 4, colors::ACCENT);
        window.fill_rect(x + 8, y + 4, 4, 6, colors::ACCENT);
        window.fill_rect(x, y + 8, 12, 4, colors::ACCENT);
        window.fill_rect(x + 8, y + 12, 4, 8, colors::ACCENT);
    }

    /// Desenha item do menu.
    fn draw_menu_item(&self, window: &mut Window, x: u32, y: u32, _label: &str, enabled: bool) {
        let item_height: u32 = 32;
        let item_width: u32 = 218;

        let bg_color = if enabled {
            colors::MENU_ITEM_HOVER
        } else {
            colors::MENU_BG
        };

        let text_color = if enabled {
            colors::WHITE
        } else {
            colors::TEXT_DISABLED
        };

        // Fundo do item
        window.fill_rect(x, y, item_width, item_height, bg_color);

        // Ícone placeholder (quadrado)
        window.fill_rect(x + 8, y + 8, 16, 16, text_color);

        // Texto seria desenhado aqui com fonte (simplificado por enquanto)
        // Por ora, uma linha para indicar texto
        window.fill_rect(x + 32, y + 14, 80, 4, text_color);
    }

    /// Alterna estado do menu iniciar.
    #[allow(dead_code)]
    pub fn toggle_start_menu(&mut self) {
        self.start_menu_open = !self.start_menu_open;
    }

    /// Retorna a área de trabalho disponível (excluindo a taskbar).
    pub fn get_work_area(&self, screen_width: u32) -> (u32, u32, u32, u32) {
        (0, 0, screen_width, self.y)
    }

    /// Processa um clique do mouse na posição (x, y).
    /// Retorna a ação a ser realizada.
    pub fn handle_click(&mut self, click_x: i32, click_y: i32) -> TaskbarAction {
        // Verificar se o clique está dentro da taskbar
        if click_y < self.y as i32 || click_y >= (self.y + self.height) as i32 {
            return TaskbarAction::None;
        }

        // Área do botão iniciar
        let start_btn_x = self.x + PADDING;
        let start_btn_right = start_btn_x + START_BUTTON_WIDTH;

        if click_x >= start_btn_x as i32 && click_x < start_btn_right as i32 {
            // Clique no botão iniciar
            redpowder::println!("[Shell] Clique no botao iniciar!");
            self.start_menu_open = !self.start_menu_open;
            return TaskbarAction::ToggleStartMenu;
        }

        // Verificar cliques nos apps
        let start_x = self.x + START_BUTTON_WIDTH + 12;
        let btn_w = APP_ICON_SIZE + 8;
        let btn_gap = APP_ICON_SIZE + ICON_MARGIN + 8;

        for (i, entry) in self.entries.iter().enumerate() {
            let btn_x = start_x + (i as u32 * btn_gap);
            if click_x >= btn_x as i32 && click_x < (btn_x + btn_w) as i32 {
                redpowder::println!("[Shell] Clique na janela {} ({})", entry.id, entry.title);
                return TaskbarAction::ToggleWindow(entry.id);
            }
        }

        // Clique em outra área da taskbar
        TaskbarAction::None
    }

    /// Desenha apenas o conteúdo do menu iniciar em uma janela dedicada.
    pub fn draw_menu_content(&self, window: &mut Window) {
        let menu_width = window.width;
        let menu_height = window.height;
        let x = 0;
        let y = 0;

        // Fundo do menu
        window.fill_rect(x, y, menu_width, menu_height, colors::MENU_BG);

        // Borda
        window.fill_rect(x, y, menu_width, 1, colors::TASKBAR_BORDER);
        window.fill_rect(x, y, 1, menu_height, colors::TASKBAR_BORDER);
        window.fill_rect(
            x + menu_width - 1,
            y,
            1,
            menu_height,
            colors::TASKBAR_BORDER,
        );
        window.fill_rect(
            x,
            y + menu_height - 1,
            menu_width,
            1,
            colors::TASKBAR_BORDER,
        );

        // Título "RedstoneOS"
        self.draw_menu_title(window, x + 16, y + 16);

        // Separador
        window.fill_rect(x + 16, y + 50, menu_width - 32, 1, colors::TASKBAR_BORDER);

        // Item: Settings (desabilitado)
        self.draw_menu_item(window, x + 16, y + 100, "Configuracoes", false);

        // Item: Sobre
        self.draw_menu_item(window, x + 16, y + 140, "Sobre", false);
    }
}
