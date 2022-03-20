#![no_std]
#![feature(asm)]
#![feature(start)]
#![feature(core_intrinsics)]

use core::{panic::PanicInfo, fmt::Write};

use asm::{hlt, sti};
use int::{init_pic, allow_input};
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
    let mut segmentation = Segmentation::new();
    segmentation.init();
    init_pic();
    allow_input();
    sti();

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