//! # Colors - Paleta de Cores
//!
//! Definições centralizadas de todas as cores usadas no Shell.
//! Formato: ARGB (Alpha, Red, Green, Blue)

#![allow(dead_code)]

// ============================================================================
// CORES DO DESKTOP
// ============================================================================

/// Cor do wallpaper (Laranja Redstone)
pub const WALLPAPER_COLOR: u32 = 0xFFFF4500;

/// Cor de fundo alternativa (cinza escuro)
pub const DESKTOP_BG_DARK: u32 = 0xFF1a1a2e;

// ============================================================================
// CORES DA TASKBAR
// ============================================================================

/// Cor de fundo da taskbar (escuro translúcido)
pub const TASKBAR_BG: u32 = 0xFF1E1E28;

/// Cor da borda superior da taskbar
pub const TASKBAR_BORDER: u32 = 0xFF3C3C50;

/// Cor do botão iniciar
pub const START_BUTTON_BG: u32 = 0xFF4682B4;

/// Cor do botão de app ativo
pub const APP_BUTTON_ACTIVE: u32 = 0xFF3A3A4C;

/// Cor do botão de app hover
pub const APP_BUTTON_HOVER: u32 = 0xFF4A4A5C;

/// Cor de destaque (indicador de app ativo)
pub const ACCENT: u32 = 0xFFFF6B35;

// ============================================================================
// CORES DO MENU
// ============================================================================

/// Cor de fundo do menu
pub const MENU_BG: u32 = 0xFF252530;

/// Cor do item hover
pub const MENU_ITEM_HOVER: u32 = 0xFF3A3A4C;

// ============================================================================
// CORES DE TEXTO/ÍCONES
// ============================================================================

/// Branco puro
pub const WHITE: u32 = 0xFFFFFFFF;

/// Texto secundário (cinza claro)
pub const TEXT_SECONDARY: u32 = 0xFFAAAAAA;

/// Texto desativado
pub const TEXT_DISABLED: u32 = 0xFF666666;

/// Verde (indicadores positivos)
pub const GREEN: u32 = 0xFF3FB950;

// ============================================================================
// CORES DO SISTEMA
// ============================================================================

/// Transparente
pub const TRANSPARENT: u32 = 0x00000000;

/// Preto
pub const BLACK: u32 = 0xFF000000;

/// Vermelho (erro/fechar)
pub const RED: u32 = 0xFFFF0000;

/// Azul (informação)
pub const BLUE: u32 = 0xFF0000FF;
