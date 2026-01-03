//! # Theme Module
//!
//! Sistema de temas do Shell.
//!
//! ## Componentes
//!
//! - **colors**: Paleta de cores usando gfx_types
//! - **glass**: Efeitos de vidro/blur
//! - **metrics**: Métricas de layout

pub mod colors;
pub mod glass;
pub mod metrics;

pub use colors::*;
pub use glass::*;
pub use metrics::*;
