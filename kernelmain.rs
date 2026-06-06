#![no_std]
#![no_main]

#[path = "src/io.rs"]
mod io;
#[path = "src/ioasm.rs"]
mod ioasm;
#[path = "src/types.rs"]
mod types;

use core::panic::PanicInfo;
use io::*;
use types::CursorStyle::*;
use types::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() {
    clear_screen();
    // write_to_screen(b"Hello, world!");

    style_cursor(Disable);

    let message: &[u8] = b"vl_OS";
    let subheading: &[u8] = b"a REAL os for REAL programmers";
    let stars: &[u8] = b"* * *";
    let msg_len: u16 = message.len() as u16;
    let sub_len: u16 = subheading.len() as u16;
    let stars_len: u16 = stars.len() as u16;

    let msg_row: u16 = 11;
    let msg_col: u16 = (SCREEN_COLS - msg_len) / 2;
    let sub_col: u16 = (SCREEN_COLS - sub_len) / 2;
    let stars_col: u16 = (SCREEN_COLS - stars_len) / 2;

    draw_border(msg_row - 3, MAGENTA);

    draw_letter(stars, msg_row - 1, stars_col, WHITE, BLACK);

    draw_letter(message, msg_row, msg_col, LIGHTCYAN, BLACK);

    draw_letter(subheading, msg_row + 1, sub_col, LIGHTMAGENTA, BLACK);

    draw_letter(stars, msg_row + 2, stars_col, WHITE, BLACK);

    draw_border(msg_row + 4, MAGENTA);
}
