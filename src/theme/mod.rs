//! # Theme Module - Sistema de Temas
//!
//! Define cores e estilos visuais do Shell.

pub mod colors;

// Re-export específico ao invés de wildcard
// Removido pub use colors::* pois o taskbar usa crate::theme::colors diretamente
