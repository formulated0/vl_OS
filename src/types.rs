pub const BLACK: u8 = 0x0;
pub const BLUE: u8 = 0x1;
pub const GREEN: u8 = 0x2;
pub const CYAN: u8 = 0x3;
pub const RED: u8 = 0x4;
pub const MAGENTA: u8 = 0x5;
pub const BROWN: u8 = 0x6;
pub const LIGHTGREY: u8 = 0x7;
pub const DARKGREY: u8 = 0x8;
pub const LIGHTBLUE: u8 = 0x9;
pub const LIGHTGREEN: u8 = 0xA;
pub const LIGHTCYAN: u8 = 0xB;
pub const LIGHTRED: u8 = 0xC;
pub const LIGHTMAGENTA: u8 = 0xD;
pub const LIGHTBROWN: u8 = 0xE;
pub const WHITE: u8 = 0xF;

pub const FRAMEBUFFER: *mut u16 = 0x000B8000 as *mut u16;
pub const CRTC_CMD_PORT: u16 = 0x3D4;
pub const CRTC_DATA_PORT: u16 = 0x3D5;
pub const CURSOR_POS_HIGH_BYTE_CMD: u8 = 0x0E;
pub const CURSOR_POS_LOW_BYTE_CMD: u8 = 0x0F;
pub const SCREEN_START_POS_HIGH_BYTE_CMD: u8 = 0x0C;
pub const SCREEN_START_POS_LOW_BYTE_CMD: u8 = 0x0D;
pub const CURSOR_STYLE_START_CMD: u8 = 0x0A;
pub const CURSOR_STYLE_END_CMD: u8 = 0x0B;

pub const DELAY_SHORT: u32 = 100;
pub const DELAY_MEDIUM: u32 = 500;
pub const DELAY_LONG: u32 = 2000;
pub const SCREEN_COLS: u16 = 80;
pub const SCREEN_ROWS: u16 = 25;

pub enum CursorStyle {
    Big,
    Small,
    Disable,
    Enable,
}
