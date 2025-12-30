//! # Firefly Shell - Terminal Gráfico
//!
//! Terminal gráfico do RedstoneOS Desktop Environment.

#![no_std]
#![no_main]

mod font;

use core::panic::PanicInfo;
use font::{get_char_bitmap, CHAR_HEIGHT, CHAR_WIDTH};
use redpowder::graphics::{Color, Framebuffer};
use redpowder::println;

// ============================================================================
// CONSTANTES
// ============================================================================

/// Cor de fundo do terminal (preto escuro)
const BG_COLOR: Color = Color::rgb(20, 20, 30);

/// Cor do texto (verde terminal clássico)
const TEXT_COLOR: Color = Color::rgb(0, 255, 100);

/// Cor da borda
const BORDER_COLOR: Color = Color::rgb(100, 100, 120);

/// Margem interna
const PADDING: u32 = 8;

/// Banner de boas-vindas
const BANNER: &str = r#"
  ____          _     _                    
 |  _ \ ___  __| |___| |_ ___  _ __   ___ 
 | |_) / _ \/ _` / __| __/ _ \| '_ \ / _ \
 |  _ <  __/ (_| \__ \ || (_) | | | |  __/
 |_| \_\___|\__,_|___/\__\___/|_| |_|\___|

 Redstone OS v0.1.0 (firefly)
 Type 'help' for commands.
"#;

// ============================================================================
// RENDERIZAÇÃO DE TEXTO
// ============================================================================

/// Desenha um caractere no framebuffer
fn draw_char(fb: &mut Framebuffer, x: u32, y: u32, c: char, color: Color) {
    if let Some(bitmap) = get_char_bitmap(c) {
        for row in 0..CHAR_HEIGHT {
            let byte = bitmap[row as usize];
            for col in 0..CHAR_WIDTH {
                // Bit mais significativo = pixel mais à esquerda
                if (byte >> (7 - col)) & 1 != 0 {
                    let _ = fb.put_pixel(x + col, y + row, color);
                }
            }
        }
    }
}

/// Desenha uma string no framebuffer
fn draw_text(fb: &mut Framebuffer, x: u32, y: u32, text: &str, color: Color) {
    let mut cursor_x = x;
    let mut cursor_y = y;

    for c in text.chars() {
        if c == '\n' {
            cursor_x = x;
            cursor_y += CHAR_HEIGHT + 2; // Espaçamento entre linhas
        } else {
            draw_char(fb, cursor_x, cursor_y, c, color);
            cursor_x += CHAR_WIDTH;
        }
    }
}

/// Desenha um retângulo preenchido
fn fill_rect(fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32, color: Color) {
    for dy in 0..h {
        for dx in 0..w {
            let _ = fb.put_pixel(x + dx, y + dy, color);
        }
    }
}

/// Desenha a borda de um retângulo
fn draw_border(fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32, color: Color, thickness: u32) {
    // Top
    fill_rect(fb, x, y, w, thickness, color);
    // Bottom
    fill_rect(fb, x, y + h - thickness, w, thickness, color);
    // Left
    fill_rect(fb, x, y, thickness, h, color);
    // Right
    fill_rect(fb, x + w - thickness, y, thickness, h, color);
}

// ============================================================================
// ENTRADA
// ============================================================================

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    println!("[Shell] Start!");

    // Obter framebuffer
    if let Ok(mut fb) = Framebuffer::new() {
        println!("[Shell] FB OK");

        // Dimensões da tela
        let screen_w = fb.width();
        let screen_h = fb.height();

        // Dimensões da janela do terminal (80% da tela)
        let win_w = (screen_w * 80) / 100;
        let win_h = (screen_h * 80) / 100;
        let win_x = (screen_w - win_w) / 2;
        let win_y = (screen_h - win_h) / 2;

        // Desenhar fundo da janela
        fill_rect(&mut fb, win_x, win_y, win_w, win_h, BG_COLOR);

        // Desenhar borda
        draw_border(&mut fb, win_x, win_y, win_w, win_h, BORDER_COLOR, 2);

        // Desenhar banner
        let text_x = win_x + PADDING;
        let text_y = win_y + PADDING;
        draw_text(&mut fb, text_x, text_y, BANNER, TEXT_COLOR);

        // Desenhar prompt
        let prompt_y = text_y + 12 * (CHAR_HEIGHT + 2); // Após o banner
        draw_text(&mut fb, text_x, prompt_y, "redstone> _", TEXT_COLOR);

        println!("[Shell] Rendered!");
    } else {
        println!("[Shell] FB FAIL");
    }

    println!("[Shell] Done!");
    loop {
        core::hint::spin_loop();
    }
}

// ============================================================================
// PANIC HANDLER
// ============================================================================

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[Shell] PANIC!");
    if let Some(location) = info.location() {
        println!("at {}:{}", location.file(), location.line());
    }
    loop {
        core::hint::spin_loop();
    }
}
