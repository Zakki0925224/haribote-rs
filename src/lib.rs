#![no_std]
#![feature(asm)]
#![feature(start)]

use core::{panic::PanicInfo};

use asm::{hlt};
use vga::set_palette;

mod asm;
mod vga;

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> !
{
    set_palette(0, 15);
    for i in 0xa000..0xaffff
    {
        let vram = unsafe { &mut *(i as *mut u8) };
        *vram = i as u8 & 0x0f;
    }
    loop { hlt(); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop { hlt(); }
}