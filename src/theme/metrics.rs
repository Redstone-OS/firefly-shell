//! # Metrics - Métricas de Layout
//!
//! Constantes de layout para o Shell.

// =============================================================================
// TASKBAR
// =============================================================================

/// Altura das barras da taskbar.
pub const TASKBAR_HEIGHT: u32 = 48;

/// Margem das barras em relação à borda da tela.
pub const TASKBAR_MARGIN: u32 = 8;

/// Espaçamento entre as barras.
pub const TASKBAR_GAP: u32 = 12;

/// Raio dos cantos das barras.
pub const TASKBAR_RADIUS: u32 = 12;

/// Padding interno das barras.
pub const TASKBAR_PADDING: u32 = 8;

// =============================================================================
// BARRAS INDIVIDUAIS
// =============================================================================

/// Largura da barra de widgets (esquerda).
pub const WIDGET_BAR_WIDTH: u32 = 56;

/// Largura mínima da barra central (menu).
pub const CENTER_BAR_MIN_WIDTH: u32 = 200;

/// Largura da barra de status (direita).
pub const STATUS_BAR_WIDTH: u32 = 140;

// =============================================================================
// BOTÕES
// =============================================================================

/// Tamanho dos botões de ícone.
pub const ICON_BUTTON_SIZE: u32 = 40;

/// Tamanho dos ícones.
pub const ICON_SIZE: u32 = 24;

/// Espaçamento entre ícones.
pub const ICON_GAP: u32 = 4;

// =============================================================================
// PAINÉIS
// =============================================================================

/// Largura do painel de widgets.
pub const WIDGET_PANEL_WIDTH: u32 = 380;

/// Altura máxima do painel de widgets.
pub const WIDGET_PANEL_HEIGHT: u32 = 500;

/// Largura do painel de quick settings.
pub const QUICK_SETTINGS_WIDTH: u32 = 320;

/// Altura do painel de quick settings.
pub const QUICK_SETTINGS_HEIGHT: u32 = 280;

/// Largura do menu iniciar.
pub const START_MENU_WIDTH: u32 = 400;

/// Altura do menu iniciar.
pub const START_MENU_HEIGHT: u32 = 500;

/// Raio dos painéis.
pub const PANEL_RADIUS: u32 = 16;

/// Padding interno dos painéis.
pub const PANEL_PADDING: u32 = 16;

// =============================================================================
// ITENS DO MENU
// =============================================================================

/// Altura de um item de app no menu.
pub const APP_ITEM_HEIGHT: u32 = 56;

/// Tamanho do ícone do app.
pub const APP_ICON_SIZE: u32 = 40;

/// Espaçamento entre ícone e texto.
pub const APP_ICON_GAP: u32 = 12;

// =============================================================================
// GRID DE APPS
// =============================================================================

/// Colunas no grid de apps.
pub const APP_GRID_COLS: u32 = 4;

/// Tamanho da célula do grid.
pub const APP_GRID_CELL_SIZE: u32 = 88;

/// Espaçamento do grid.
pub const APP_GRID_GAP: u32 = 8;

// =============================================================================
// ANIMAÇÃO
// =============================================================================

/// Duração de animação (frames @ 60fps).
pub const ANIMATION_DURATION: u32 = 15;

/// Duração curta.
pub const ANIMATION_SHORT: u32 = 8;
