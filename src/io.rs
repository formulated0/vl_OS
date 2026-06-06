use crate::ioasm::{outb, inb};
use crate::types::*;

pub fn clear_screen() {
    for row in 0..25u16 {
        for col in 0..80u16 {
            write_letter_to_framebuffer(b' ', row, col, BLACK, BLACK);
        }
    }
}

pub fn write_letter_to_framebuffer(letter: u8, row: u16, col: u16, text_color: u8, bg_color: u8) {
    unsafe {
        let letter_fb: u16 = 0x00FF & letter as u16;
        let bg_color_fb: u16 = (0x000F & bg_color as u16) << 12;
        let text_color_fb: u16 = (0x000F & text_color as u16) << 8;

        *FRAMEBUFFER.add((col + row * 80) as usize) = letter_fb | text_color_fb | bg_color_fb;
    }
}

pub fn move_cursor(pos: u16) {
    let pos_low_byte: u8 = (pos & 0x00FF) as u8;
    let pos_high_byte: u8 = ((pos >> 8) & 0x00FF) as u8;

    outb(CRTC_CMD_PORT, CURSOR_POS_HIGH_BYTE_CMD);
    outb(CRTC_DATA_PORT, pos_high_byte);
    outb(CRTC_CMD_PORT, CURSOR_POS_LOW_BYTE_CMD);
    outb(CRTC_DATA_PORT, pos_low_byte);
}

pub fn scroll(row: u16) {
    let pos: u16 = 80 * row;
    let pos_low_byte: u8 = (pos & 0x00FF) as u8;
    let pos_high_byte: u8 = ((pos >> 8) & 0x00FF) as u8;

    outb(CRTC_CMD_PORT, SCREEN_START_POS_HIGH_BYTE_CMD);
    outb(CRTC_DATA_PORT, pos_high_byte);
    outb(CRTC_CMD_PORT, SCREEN_START_POS_LOW_BYTE_CMD);
    outb(CRTC_DATA_PORT, pos_low_byte);
}

pub fn write_letter_to_screen(c: u8, pos: u16) {
    write_letter_to_framebuffer(c, 0, pos, WHITE, BLACK);
}

pub fn write_to_screen(buf: &[u8], pos: u16) {
    for (i, &letter) in buf.iter().enumerate() {
        write_letter_to_framebuffer(letter, 0, pos + i as u16, WHITE, BLACK);
    }
    move_cursor(pos + buf.len() as u16);
}

pub fn print_byte(byte: u8, pos: u16) {
    for bit in 0..8u8 {
        let mask = 1u8 << (7 - bit);
        if byte & mask != 0 {
            write_to_screen(b"1", pos + bit as u16);
        } else {
            write_to_screen(b"0", pos + bit as u16);
        }
    }
}

pub fn style_cursor(cstyle: CursorStyle) {
    match cstyle {
        CursorStyle::Big => {
            outb(CRTC_CMD_PORT, CURSOR_STYLE_START_CMD);
            outb(CRTC_DATA_PORT, 0x00);
        }
        CursorStyle::Small => {
            outb(CRTC_CMD_PORT, CURSOR_STYLE_START_CMD);
            outb(CRTC_DATA_PORT, 0x0C);
        }
        CursorStyle::Disable => {
            outb(CRTC_CMD_PORT, CURSOR_STYLE_START_CMD);
            let start = inb(CRTC_DATA_PORT);
            outb(CRTC_DATA_PORT, start | 0x20);
        }
        CursorStyle::Enable => {
            outb(CRTC_CMD_PORT, CURSOR_STYLE_START_CMD);
            let start = inb(CRTC_DATA_PORT);
            outb(CRTC_DATA_PORT, start & 0xBF);
        }
    }
}


pub fn wait(count: u32) {
    for _ in 0..count {
        core::hint::spin_loop();
    }
}


pub fn draw_border(row: u16, color: u8) {
    for i in 0..(SCREEN_COLS / 2) {
        write_letter_to_framebuffer(b'=', row, i, color, BLACK);
        write_letter_to_framebuffer(b'=', row, SCREEN_COLS - 1 - i, color, BLACK);
        wait(DELAY_SHORT);
    }
}

pub fn draw_letter(string: &[u8], row: u16, col: u16, foreground_color: u8, background_color: u8) {
    for (i, &c) in string.iter().enumerate() {
        write_letter_to_framebuffer(c, row, col + i as u16, foreground_color, background_color);
        wait(DELAY_MEDIUM);
    }
}

