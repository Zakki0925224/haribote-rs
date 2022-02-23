#![no_std]
#![feature(asm)]
#![feature(start)]

use core::{panic::PanicInfo};

use asm::{hlt};
use vga::{Screen};

mod asm;
mod vga;

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> !
{
    let mut screen = Screen::new();
    screen.init();

    loop { hlt(); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop { hlt(); }
}