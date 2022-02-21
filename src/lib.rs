#![no_std]
#![feature(asm)]
#![feature(start)]

use core::{panic::PanicInfo, arch::asm};

use asm::{load_eflags, out8};

mod asm;

#[no_mangle]
fn hlt()
{
    unsafe { asm!("hlt"); }
}

#[no_mangle]
fn show_color(addr: u32, color_code: u8)
{
    let ptr = unsafe { &mut *(addr as *mut u8) };
    *ptr = color_code;
}

const RGB_TABLE: [[u8; 3]; 16] =
    [
        [0x00, 0x00, 0x00], // 黒
        [0xff, 0x00, 0x00], // 明るい赤
        [0x00, 0xff, 0x00], // 明るい緑
        [0xff, 0xff, 0x00], // 明るい黄色
        [0x00, 0x00, 0xff], // 明るい青
        [0xff, 0x00, 0xff], // 明るい紫
        [0x00, 0xff, 0xff], // 明るい水色
        [0xff, 0xff, 0xff], // 白
        [0xc6, 0xc6, 0xc6], // 明るい灰色
        [0x84, 0x00, 0x00], // 暗い赤
        [0x00, 0x84, 0x00], // 暗い緑
        [0x84, 0x84, 0x00], // 暗い黄色
        [0x00, 0x00, 0x84], // 暗い青
        [0x84, 0x00, 0x84], // 暗い紫
        [0x00, 0x84, 0x84], // 暗い水色
        [0x84, 0x84, 0x84] // 暗い灰色
    ];

fn set_palette(start: u32, end: u32)
{
    let rgb = &RGB_TABLE;
    let eflags = load_eflags();
    out8(0x03c8, start as u8);

    for i in start..end+1
    {
        out8(0x03c9, rgb[i as usize][0] / 4);
        out8(0x03c9, rgb[i as usize][1] / 4);
        out8(0x03c9, rgb[i as usize][2] / 4);
    }
}

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> !
{
    set_palette(0, 15);

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