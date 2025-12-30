//! # Firefly Shell - Terminal Gráfico
//!
//! Terminal gráfico do RedstoneOS Desktop Environment.
//!
//! Esta versão simplificada desenha diretamente no framebuffer.
//! Uma versão futura usará o protocolo de janelas para comunicação
//! com o compositor.

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
const PADDING: u32 = 12;

/// Banner de boas-vindas
const BANNER: &str = "Redstone OS v0.1.0

Type 'help' for commands.

redstone> _";

// ============================================================================
// RENDERIZAÇÃO DE TEXTO
// ============================================================================

/// Desenha um caractere no framebuffer
fn draw_char(fb: &mut Framebuffer, x: u32, y: u32, c: char, color: Color) {
    if let Some(bitmap) = get_char_bitmap(c) {
        for row in 0..CHAR_HEIGHT {
            let byte = bitmap[row as usize];
            for col in 0..CHAR_WIDTH {
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
            cursor_y += CHAR_HEIGHT + 2;
        } else {
            draw_char(fb, cursor_x, cursor_y, c, color);
            cursor_x += CHAR_WIDTH;
        }
    }
}

// ============================================================================
// ENTRADA
// ============================================================================

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    println!("[Shell] Start!");

    if let Ok(mut fb) = Framebuffer::new() {
        println!("[Shell] FB OK");

        let screen_w = fb.width();
        let screen_h = fb.height();

        // Janela do terminal (60% da tela, centralizada)
        let win_w = (screen_w * 60) / 100;
        let win_h = (screen_h * 60) / 100;
        let win_x = (screen_w - win_w) / 2;
        let win_y = (screen_h - win_h) / 2;

        // Desenhar fundo da janela (otimizado com fill_rect)
        let _ = fb.fill_rect(win_x, win_y, win_w, win_h, BG_COLOR);

        // Desenhar borda (4 linhas)
        let _ = fb.fill_rect(win_x, win_y, win_w, 2, BORDER_COLOR);
        let _ = fb.fill_rect(win_x, win_y + win_h - 2, win_w, 2, BORDER_COLOR);
        let _ = fb.fill_rect(win_x, win_y, 2, win_h, BORDER_COLOR);
        let _ = fb.fill_rect(win_x + win_w - 2, win_y, 2, win_h, BORDER_COLOR);

        // Desenhar texto do banner
        let text_x = win_x + PADDING;
        let text_y = win_y + PADDING;
        draw_text(&mut fb, text_x, text_y, BANNER, TEXT_COLOR);

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
