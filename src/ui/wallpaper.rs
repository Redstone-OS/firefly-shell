//! # Wallpaper
//!
//! Componente de papel de parede.
//! Tenta carregar webp, fallback para gradiente.
// TODO: Revisar no futuro
#[allow(unused)]
use gfx_types::color::Color;
use gfx_types::geometry::{Rect, Size};

use crate::theme::colors;

// =============================================================================
// CONSTANTES
// =============================================================================

// TODO: Revisar no futuro
#[allow(unused)]
/// Caminho do wallpaper padrão.
const DEFAULT_WALLPAPER_PATH: &str = "/system/resources/wallpapers/default.webp";

// =============================================================================
// WALLPAPER
// =============================================================================

/// Componente de papel de parede.
pub struct Wallpaper {
    /// Bounds do wallpaper.
    bounds: Rect,
    /// Imagem carregada (se disponível).
    image_data: Option<WallpaperImage>,
    /// Usa gradiente fallback.
    use_gradient: bool,
}

/// Dados da imagem de wallpaper.
struct WallpaperImage {
    width: u32,
    height: u32,
    pixels: alloc::vec::Vec<u32>,
}

impl Wallpaper {
    /// Cria wallpaper com bounds.
    pub fn new(width: u32, height: u32) -> Self {
        let mut wallpaper = Self {
            bounds: Rect::new(0, 0, width, height),
            image_data: None,
            use_gradient: true,
        };

        // Tentar carregar imagem
        wallpaper.try_load_image();

        wallpaper
    }

    /// Tenta carregar a imagem do wallpaper.
    fn try_load_image(&mut self) {
        // TODO: Implementar carregamento de webp quando tivermos decoder
        // Por enquanto, sempre usa gradiente
        redpowder::println!("[Wallpaper] Usando gradiente fallback (webp não suportado ainda)");
        self.use_gradient = true;
    }

    /// Desenha o wallpaper no buffer.
    pub fn draw(&self, buffer: &mut [u32], buffer_size: Size) {
        if let Some(ref image) = self.image_data {
            self.draw_image(buffer, buffer_size, image);
        } else {
            self.draw_gradient(buffer, buffer_size);
        }
    }

    /// Desenha imagem.
    fn draw_image(&self, buffer: &mut [u32], buffer_size: Size, image: &WallpaperImage) {
        let stride = buffer_size.width as usize;

        // Escalar/centralizar imagem conforme necessário
        for y in 0..self.bounds.height.min(image.height) {
            let src_y = y as usize;
            let dst_y = (self.bounds.y as u32 + y) as usize;

            if dst_y >= buffer_size.height as usize {
                continue;
            }

            for x in 0..self.bounds.width.min(image.width) {
                let src_x = x as usize;
                let dst_x = (self.bounds.x as u32 + x) as usize;

                if dst_x >= buffer_size.width as usize {
                    continue;
                }

                let src_idx = src_y * image.width as usize + src_x;
                let dst_idx = dst_y * stride + dst_x;

                if src_idx < image.pixels.len() && dst_idx < buffer.len() {
                    buffer[dst_idx] = image.pixels[src_idx];
                }
            }
        }
    }

    /// Desenha gradiente vertical (fallback).
    fn draw_gradient(&self, buffer: &mut [u32], buffer_size: Size) {
        let stride = buffer_size.width as usize;

        let top = colors::WALLPAPER_GRADIENT_TOP;
        let bottom = colors::WALLPAPER_GRADIENT_BOTTOM;

        for y in 0..self.bounds.height {
            let dst_y = (self.bounds.y as u32 + y) as usize;
            if dst_y >= buffer_size.height as usize {
                continue;
            }

            // Interpolação linear
            let t = y as f32 / self.bounds.height as f32;
            let color = top.lerp(&bottom, t).as_u32();

            let row_start = dst_y * stride + self.bounds.x as usize;
            let row_end = (row_start + self.bounds.width as usize).min(buffer.len());

            if row_start < buffer.len() {
                buffer[row_start..row_end].fill(color);
            }
        }

        // Adicionar sutil noise/pattern para não ficar flat
        self.add_subtle_pattern(buffer, buffer_size);
    }

    /// Adiciona pattern sutil ao gradiente.
    fn add_subtle_pattern(&self, buffer: &mut [u32], buffer_size: Size) {
        let stride = buffer_size.width as usize;

        // Pattern diagonal sutil
        for y in (0..self.bounds.height).step_by(3) {
            for x in (0..self.bounds.width).step_by(3) {
                let dst_x = (self.bounds.x as u32 + x) as usize;
                let dst_y = (self.bounds.y as u32 + y) as usize;

                if dst_x >= buffer_size.width as usize || dst_y >= buffer_size.height as usize {
                    continue;
                }

                let idx = dst_y * stride + dst_x;
                if idx < buffer.len() {
                    // Sutil variação (+/- 5 em brightness)
                    let pixel = buffer[idx];
                    let variation = if ((x + y) / 3) % 2 == 0 { 5 } else { -5i32 };

                    let r = ((pixel >> 16) & 0xFF) as i32;
                    let g = ((pixel >> 8) & 0xFF) as i32;
                    let b = (pixel & 0xFF) as i32;

                    let r = (r + variation).clamp(0, 255) as u32;
                    let g = (g + variation).clamp(0, 255) as u32;
                    let b = (b + variation).clamp(0, 255) as u32;

                    buffer[idx] = 0xFF000000 | (r << 16) | (g << 8) | b;
                }
            }
        }
    }

    // TODO: Revisar no futuro
    #[allow(unused)]
    /// Define bounds.
    pub fn set_bounds(&mut self, x: i32, y: i32, width: u32, height: u32) {
        self.bounds = Rect::new(x, y, width, height);
    }
}
