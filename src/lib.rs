#![no_std]
#![feature(asm)]
#![feature(start)]

use core::{panic::PanicInfo, fmt::Write};

use asm::{hlt};
use vga::{Screen, Color, ScreenWriter};

mod asm;
mod font;
mod vga;

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> !
{
    let mut screen = Screen::new();
    screen.init();
    let mut writer = ScreenWriter::new(screen, Color::White, 10, 10);
    write!(writer, "Hello, world!\naaa").unwrap();

    loop { hlt(); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop { hlt(); }
}