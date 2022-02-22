#![no_std]
#![feature(asm)]
#![feature(start)]

use core::{panic::PanicInfo};

use asm::{hlt};
use vga::{set_palette, boxfill8, Color};

mod asm;
mod vga;

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> !
{
    set_palette(0, 15);
    let vram = unsafe { &mut *(0xa0000 as *mut u8) };

    boxfill8(vram, 320, Color::LightRed, 20, 20, 120, 120);
    boxfill8(vram, 320, Color::LightGreen, 70, 50, 170, 150);
    boxfill8(vram, 320, Color::LightBlue, 120, 80, 220, 180);

    loop { hlt(); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop { hlt(); }
}