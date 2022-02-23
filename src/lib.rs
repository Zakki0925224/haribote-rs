#![no_std]
#![feature(asm)]
#![feature(start)]

use core::{panic::PanicInfo};

use asm::{hlt};
use font::FONT_A;
use vga::{Screen};

mod asm;
mod font;
mod vga;

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> !
{
    let mut screen = Screen::new();
    screen.init();
    screen.pubfont8(10, 10, vga::Color::White, &FONT_A);

    loop { hlt(); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop { hlt(); }
}