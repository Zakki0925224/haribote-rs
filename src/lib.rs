#![no_std]
#![feature(asm)]
#![feature(start)]

use core::{panic::PanicInfo, fmt::Write};

use asm::{hlt};
use vga::{Screen, Color, ScreenWriter};
use sgm::Segmentation;

mod asm;
mod font;
mod vga;
mod sgm;
mod int;

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> !
{
    let mut screen = Screen::new();
    screen.init();
    let mut segmentation = Segmentation::new();
    segmentation.init();
    int::init_pic();
    let mut writer = ScreenWriter::new(screen, Color::White, 10, 10);
    write!(writer, "Hello, world!\naaa").unwrap();

    loop { hlt(); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop { hlt(); }
}