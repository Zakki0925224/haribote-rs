use crate::asm::{load_eflags, out8};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color
{
    Black = 0,
    LightRed = 1,
    LightGreen = 2,
    LightYellow = 3,
    LightBlue = 4,
    LightPurple = 5,
    LightCyan = 6,
    White = 7,
    LightGray = 8,
    DarkRed = 9,
    DarkGreen = 10,
    DarkYellow = 11,
    DarkBlue = 12,
    DarkPurple = 13,
    DarkCyan = 14,
    DarkGray = 15
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

pub fn set_palette(start: u32, end: u32)
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

pub fn boxfill8(vram: *mut u8, xsize: isize, color: Color, x0: isize, y0: isize, x1: isize, y1: isize)
{
    for y in y0..=y1
    {
        for x in x0..=x1
        {
            let ptr = unsafe { &mut *(vram.offset(y * xsize + x)) };
            *ptr = color as u8;
        }
    }
}