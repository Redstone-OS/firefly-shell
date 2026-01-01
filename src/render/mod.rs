//! # Render Module - Renderização
//!
//! Funções de renderização e fonte bitmap.
//! Nota: O módulo font existe para uso futuro quando texto for necessário no shell.

pub mod font;

// Remover re-export não utilizado para evitar warnings
// A fonte será usada quando implementarmos texto na UI
