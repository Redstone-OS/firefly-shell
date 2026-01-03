//! # Colors - Paleta de Cores
//!
//! Definições centralizadas usando gfx_types::Color.

#![allow(dead_code)]

use gfx_types::color::Color;

// =============================================================================
// CORES PRIMÁRIAS (REDSTONE THEME)
// =============================================================================

/// Cor de destaque principal (Redstone Orange)
pub const ACCENT: Color = Color(0xFFFF6B35);

/// Cor de destaque secundária
pub const ACCENT_LIGHT: Color = Color(0xFFFF8B55);

/// Cor de destaque escura
pub const ACCENT_DARK: Color = Color(0xFFE55520);

// =============================================================================
// CORES DE FUNDO
// =============================================================================

/// Fundo do desktop (fallback)
pub const DESKTOP_BG: Color = Color(0xFF1a1a2e);

/// Fundo escuro
pub const BG_DARK: Color = Color(0xFF0D0D15);

/// Fundo médio
pub const BG_MEDIUM: Color = Color(0xFF1E1E28);

/// Fundo claro
pub const BG_LIGHT: Color = Color(0xFF2A2A38);

// =============================================================================
// CORES DE GLASS (EFEITO VIDRO)
// =============================================================================

/// Fundo glass (semi-transparente escuro)
pub const GLASS_BG: Color = Color(0xD0181822);

/// Fundo glass hover
pub const GLASS_BG_HOVER: Color = Color(0xE0252535);

/// Fundo glass ativo
pub const GLASS_BG_ACTIVE: Color = Color(0xF0303045);

/// Borda glass
pub const GLASS_BORDER: Color = Color(0x40FFFFFF);

/// Borda glass highlight (topo)
pub const GLASS_BORDER_LIGHT: Color = Color(0x30FFFFFF);

// =============================================================================
// CORES DA TASKBAR
// =============================================================================

/// Fundo da barra (glass)
pub const TASKBAR_BG: Color = Color(0xD8181822);

/// Borda da barra
pub const TASKBAR_BORDER: Color = Color(0x50FFFFFF);

/// Highlight interno (efeito glass)
pub const TASKBAR_HIGHLIGHT: Color = Color(0x15FFFFFF);

// =============================================================================
// CORES DO MENU
// =============================================================================

/// Fundo do menu
pub const MENU_BG: Color = Color(0xE5181822);

/// Item do menu hover
pub const MENU_ITEM_HOVER: Color = Color(0xFF2A2A3C);

/// Item do menu selecionado
pub const MENU_ITEM_SELECTED: Color = Color(0xFF3A3A4C);

/// Separador do menu
pub const MENU_SEPARATOR: Color = Color(0x30FFFFFF);

// =============================================================================
// CORES DE TEXTO
// =============================================================================

/// Texto primário (branco)
pub const TEXT_PRIMARY: Color = Color::WHITE;

/// Texto secundário
pub const TEXT_SECONDARY: Color = Color(0xFFAAAAAA);

/// Texto desabilitado
pub const TEXT_DISABLED: Color = Color(0xFF666666);

/// Texto sobre accent
pub const TEXT_ON_ACCENT: Color = Color::WHITE;

// =============================================================================
// CORES DE ÍCONES
// =============================================================================

/// Ícone normal
pub const ICON_NORMAL: Color = Color(0xFFDDDDDD);

/// Ícone ativo
pub const ICON_ACTIVE: Color = Color::WHITE;

/// Ícone desabilitado
pub const ICON_DISABLED: Color = Color(0xFF555555);

// =============================================================================
// CORES DE STATUS
// =============================================================================

/// Sucesso (verde)
pub const SUCCESS: Color = Color(0xFF3FB950);

/// Aviso (amarelo)
pub const WARNING: Color = Color(0xFFF0B429);

/// Erro (vermelho)
pub const ERROR: Color = Color(0xFFE53935);

/// Info (azul)
pub const INFO: Color = Color(0xFF4A90D9);

// =============================================================================
// GRADIENTE DO WALLPAPER (FALLBACK)
// =============================================================================

/// Cor do topo do gradiente
pub const WALLPAPER_GRADIENT_TOP: Color = Color(0xFFFF4500);

/// Cor do fundo do gradiente
pub const WALLPAPER_GRADIENT_BOTTOM: Color = Color(0xFFCC3500);
