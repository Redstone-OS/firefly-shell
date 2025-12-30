//! # Firefly Shell - Terminal Gráfico
//!
//! Terminal gráfico do RedstoneOS Desktop Environment.
//!
//! Esta versão simplificada desenha diretamente no framebuffer.
//! Uma versão futura usará o protocolo de janelas para comunicação
//! com o compositor.

#![no_std]
#![no_main]

extern crate alloc;

mod font;

use alloc::string::String;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use font::{get_char_bitmap, CHAR_HEIGHT, CHAR_WIDTH};
use redpowder::graphics::{Color, Framebuffer};
use redpowder::input::{read_key, KeyEvent};
use redpowder::println;

#[global_allocator]
static ALLOCATOR: redpowder::mem::heap::SyscallAllocator = redpowder::mem::heap::SyscallAllocator;

// ============================================================================
// CONSTANTES
// ============================================================================

const BG_COLOR: Color = Color::rgb(20, 20, 30);
const TEXT_COLOR: Color = Color::rgb(0, 255, 100);
const BORDER_COLOR: Color = Color::rgb(100, 100, 120);
const PADDING: u32 = 12;

const BANNER: &str = "Redstone OS v0.1.0\nType 'help' for commands.\n";
const PROMPT: &str = "redstone> ";

// ============================================================================
// RENDERIZAÇÃO DE TEXTO
// ============================================================================

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
// SHELL
// ============================================================================

struct Shell {
    fb: Framebuffer,
    win_x: u32,
    win_y: u32,
    win_w: u32,
    win_h: u32,
    cursor_x: u32,
    cursor_y: u32,
    input_buffer: String,
    input_port: redpowder::ipc::Port,
    cursor_visible: bool,
    frame_count: usize,
}

impl Shell {
    fn new() -> Result<Self, ()> {
        let fb = Framebuffer::new().map_err(|_| ())?;
        let screen_w = fb.width();
        let screen_h = fb.height();

        let win_w = (screen_w * 60) / 100;
        let win_h = (screen_h * 60) / 100;
        let win_x = (screen_w - win_w) / 2;
        let win_y = (screen_h - win_h) / 2;

        // Cria porta de entrada
        let input_port = redpowder::ipc::Port::create("shell_input", 128).map_err(|_| ())?;

        Ok(Self {
            fb,
            win_x,
            win_y,
            win_w,
            win_h,
            cursor_x: win_x + PADDING,
            cursor_y: win_y + PADDING,
            input_buffer: String::new(),
            input_port,
            cursor_visible: true,
            frame_count: 0,
        })
    }

    fn init_display(&mut self) {
        let _ = self
            .fb
            .fill_rect(self.win_x, self.win_y, self.win_w, self.win_h, BG_COLOR);

        // Bordas
        let _ = self
            .fb
            .fill_rect(self.win_x, self.win_y, self.win_w, 2, BORDER_COLOR);
        let _ = self.fb.fill_rect(
            self.win_x,
            self.win_y + self.win_h - 2,
            self.win_w,
            2,
            BORDER_COLOR,
        );
        let _ = self
            .fb
            .fill_rect(self.win_x, self.win_y, 2, self.win_h, BORDER_COLOR);
        let _ = self.fb.fill_rect(
            self.win_x + self.win_w - 2,
            self.win_y,
            2,
            self.win_h,
            BORDER_COLOR,
        );

        // Banner
        self.print(BANNER);
        self.print_prompt();
    }

    fn draw_cursor(&mut self) {
        let color = if self.cursor_visible {
            TEXT_COLOR
        } else {
            BG_COLOR
        };
        // Cursor simples (underscore)
        let _ = self.fb.fill_rect(
            self.cursor_x,
            self.cursor_y + CHAR_HEIGHT,
            CHAR_WIDTH,
            2,
            color,
        );
    }

    fn print(&mut self, text: &str) {
        // Apaga cursor antes de imprimir para evitar sujeira
        let old_vis = self.cursor_visible;
        self.cursor_visible = false;
        self.draw_cursor();
        self.cursor_visible = old_vis;

        // Implementação simplificada: não rola a tela ainda, apenas avança
        for c in text.chars() {
            if c == '\n' {
                self.new_line();
            } else {
                draw_char(&mut self.fb, self.cursor_x, self.cursor_y, c, TEXT_COLOR);
                self.cursor_x += CHAR_WIDTH;
                // Wrap simples
                if self.cursor_x >= self.win_x + self.win_w - PADDING {
                    self.new_line();
                }
            }
        }
    }

    fn new_line(&mut self) {
        self.cursor_x = self.win_x + PADDING;
        self.cursor_y += CHAR_HEIGHT + 2;

        // TODO: Scroll
        if self.cursor_y >= self.win_y + self.win_h - PADDING {
            // Reset para topo (hack temporário)
            self.init_display();
        }
    }

    fn print_prompt(&mut self) {
        self.print(PROMPT);
    }

    fn backspace(&mut self) {
        if !self.input_buffer.is_empty() {
            // Apaga cursor atual
            self.cursor_visible = false;
            self.draw_cursor();

            self.input_buffer.pop();
            // Apagar visualmente (recuar cursor e desenhar quadrado da cor de fundo)
            self.cursor_x -= CHAR_WIDTH;
            let _ = self.fb.fill_rect(
                self.cursor_x,
                self.cursor_y,
                CHAR_WIDTH,
                CHAR_HEIGHT + 2, // Limpa altura total inc cursor
                BG_COLOR,
            );

            // Força cursor visivel na nova posição
            self.cursor_visible = true;
            self.draw_cursor();
        }
    }

    fn handle_key(&mut self, c: char) {
        if c == '\n' {
            // Remove cursor da linha atual
            self.cursor_visible = false;
            self.draw_cursor();

            self.new_line();
            let cmd = self.input_buffer.clone();
            self.input_buffer.clear();
            self.execute_command(&cmd);
            self.print_prompt();
        } else {
            self.input_buffer.push(c);
            let mut b = [0; 4];
            self.print(c.encode_utf8(&mut b));
        }

        // Força cursor aparecer logo após digitar
        self.cursor_visible = true;
        self.draw_cursor();
        self.frame_count = 0; // Reset blink timer
    }

    fn execute_command(&mut self, cmd: &str) {
        let cmd = cmd.trim();
        match cmd {
            "help" => {
                self.print("Available commands:\n");
                self.print("  help      - Show this list\n");
                self.print("  clear     - Clear screen\n");
                self.print("  reboot    - Restart system\n");
                self.print("  shutdown  - Power off system\n");
            }
            "clear" => {
                self.init_display();
                return; // prompt já é impresso no init
            }
            "reboot" => {
                self.print("Rebooting...\n");
                let _ = redpowder::console::reboot();
            }
            "shutdown" => {
                self.print("Shutting down...\n");
                let _ = redpowder::console::poweroff();
            }
            "" => {} // Ignore empty
            _ => {
                self.print("Unknown command: ");
                self.print(cmd);
                self.print("\n");
            }
        }
    }

    fn run(&mut self) -> ! {
        let mut buf = [0u8; 8]; // KeyEvent pack
        println!("(Shell) Starting event loop...");

        loop {
            // Blink cursor
            self.frame_count += 1;
            if self.frame_count > 20000 {
                // Ajustar conforme velocidade do loop
                self.cursor_visible = !self.cursor_visible;
                self.draw_cursor();
                self.frame_count = 0;
            }

            // Receber evento do input service via IPC
            match self.input_port.recv(&mut buf, 0) {
                // Timeout 0 = non-blocking?
                Ok(len) if len >= 2 => {
                    let scancode = buf[0];
                    let pressed = buf[1] != 0;

                    if pressed {
                        if let Some(c) = scancode_to_char(scancode) {
                            if c == '\x08' {
                                self.backspace();
                            } else {
                                self.handle_key(c);
                            }
                        }
                    }
                }
                _ => {}
            }

            // Pequeno delay para economizar CPU
            redpowder::process::yield_now();
        }
    }
}

// ============================================================================
// KEYMAP
// ============================================================================

fn scancode_to_char(code: u8) -> Option<char> {
    // Mapa QWERTY US Simplificado (Set 1)
    match code {
        0x02 => Some('1'),
        0x03 => Some('2'),
        0x04 => Some('3'),
        0x05 => Some('4'),
        0x06 => Some('5'),
        0x07 => Some('6'),
        0x08 => Some('7'),
        0x09 => Some('8'),
        0x0A => Some('9'),
        0x0B => Some('0'),
        0x0E => Some('\x08'), // Backspace
        0x10 => Some('q'),
        0x11 => Some('w'),
        0x12 => Some('e'),
        0x13 => Some('r'),
        0x14 => Some('t'),
        0x15 => Some('y'),
        0x16 => Some('u'),
        0x17 => Some('i'),
        0x18 => Some('o'),
        0x19 => Some('p'),
        0x1C => Some('\n'), // Enter
        0x1E => Some('a'),
        0x1F => Some('s'),
        0x20 => Some('d'),
        0x21 => Some('f'),
        0x22 => Some('g'),
        0x23 => Some('h'),
        0x24 => Some('j'),
        0x25 => Some('k'),
        0x26 => Some('l'),
        0x2C => Some('z'),
        0x2D => Some('x'),
        0x2E => Some('c'),
        0x2F => Some('v'),
        0x30 => Some('b'),
        0x31 => Some('n'),
        0x32 => Some('m'),
        0x39 => Some(' '), // Space
        _ => None,
    }
}

// ============================================================================
// MAIN
// ============================================================================

#[no_mangle]
pub extern "C" fn _start() -> ! {
    match Shell::new() {
        Ok(mut shell) => {
            shell.init_display();
            shell.run();
        }
        Err(_) => {
            // Tenta imprimir erro direto no framebuffer se possível ou serial via println
            println!("(Shell) Falha fatal ao iniciar: Erro ao criar porta ou FB");
            loop {}
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
