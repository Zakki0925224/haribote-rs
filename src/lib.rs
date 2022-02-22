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
    let xsize = 320;
    let ysize = 200;

    // 背景
    boxfill8(vram, xsize, Color::DarkCyan, 0, 0, xsize - 1, ysize - 29);

    // タスクバー
    boxfill8(vram, xsize, Color::LightGray, 0, ysize - 28, xsize - 1, ysize - 28);
    boxfill8(vram, xsize, Color::White,     0, ysize - 27, xsize - 1, ysize - 27);
    boxfill8(vram, xsize, Color::LightGray, 0, ysize - 26, xsize - 1, ysize - 1);

    // スタートボタン？
    boxfill8(vram, xsize, Color::White,    3, ysize - 24, 59, ysize - 24);
    boxfill8(vram, xsize, Color::White,    2, ysize - 24, 2, ysize - 4);
    boxfill8(vram, xsize, Color::DarkGray, 3, ysize - 4, 59, ysize - 4);
    boxfill8(vram, xsize, Color::DarkGray, 59, ysize - 23, 59, ysize - 5);
    boxfill8(vram, xsize, Color::Black,    2, ysize - 3, 59, ysize - 3);
    boxfill8(vram, xsize, Color::Black,    60, ysize - 24, 60, ysize - 3);

    // 通知
    boxfill8(vram, xsize, Color::DarkGray, xsize - 47, ysize - 24, xsize - 4, ysize - 24);
    boxfill8(vram, xsize, Color::DarkGray, xsize - 47, ysize - 23, xsize - 47, ysize - 4);
    boxfill8(vram, xsize, Color::White,    xsize - 47, ysize - 3, xsize - 4, ysize - 3);
    boxfill8(vram, xsize, Color::White,    xsize - 3, ysize - 24, xsize - 3, ysize - 3);

    loop { hlt(); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop { hlt(); }
}