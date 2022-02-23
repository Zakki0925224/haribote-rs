#![no_std]
#![feature(asm)]
#![feature(start)]

use core::{panic::PanicInfo};

use asm::{hlt};
use font::FONTPACK;
use vga::{Screen, Color};

mod asm;
mod font;
mod vga;

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> !
{
    let mut screen = Screen::new();
    screen.init();
    screen.pubfont8(10, 10, Color::White, &FONTPACK['@' as usize]);
    screen.pubfont8(20, 10, Color::White, &FONTPACK['%' as usize]);
    screen.pubfont8(30, 10, Color::White, &FONTPACK['$' as usize]);
    screen.pubfont8(40, 10, Color::White, &FONTPACK['&' as usize]);
    screen.pubfont8(50, 10, Color::White, &FONTPACK['Z' as usize]);

    loop { hlt(); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop { hlt(); }
}