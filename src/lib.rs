#![no_std]
#![feature(asm)]
#![feature(start)]

use core::{panic::PanicInfo, arch::asm};

#[no_mangle]
fn hlt()
{
    unsafe { asm!("hlt"); }
}

#[no_mangle]
fn show_color(index: u32, color_code: u8)
{
    let ptr = unsafe { &mut *(index as *mut u8) };
    *ptr = color_code;
}

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> !
{
    for i in 0xa000..0xaffff
    {
        show_color(i, (i & 0x0f) as u8);
    }
    loop { hlt(); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop { hlt(); }
}