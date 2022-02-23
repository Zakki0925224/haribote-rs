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

pub struct Screen
{
    pub scrnx: i16,
    pub scrny: i16,
    pub vram: &'static mut u8
}

impl Screen
{
    pub fn new() -> Screen
    {
        return Screen
        {
            scrnx: unsafe { *(0x0ff4 as *mut i16) },
            scrny: unsafe { *(0x0ff6 as *mut i16) },
            vram: unsafe { &mut *(*(0x0ff8 as *const i32) as *mut u8) }
        };
    }

    pub fn init(&mut self)
    {
        let xsize = self.scrnx as isize;
        let ysize = self.scrny as isize;

        self.set_palette();
        // 背景
        self.boxfill8(Color::DarkCyan, 0, 0, xsize - 1, ysize - 29);

        // タスクバー
        self.boxfill8(Color::LightGray, 0, ysize - 28, xsize - 1, ysize - 28);
        self.boxfill8(Color::White,     0, ysize - 27, xsize - 1, ysize - 27);
        self.boxfill8(Color::LightGray, 0, ysize - 26, xsize - 1, ysize - 1);

        // スタートボタン？
        self.boxfill8(Color::White,    3, ysize - 24, 59, ysize - 24);
        self.boxfill8(Color::White,    2, ysize - 24, 2, ysize - 4);
        self.boxfill8(Color::DarkGray, 3, ysize - 4, 59, ysize - 4);
        self.boxfill8(Color::DarkGray, 59, ysize - 23, 59, ysize - 5);
        self.boxfill8(Color::Black,    2, ysize - 3, 59, ysize - 3);
        self.boxfill8(Color::Black,    60, ysize - 24, 60, ysize - 3);

        // 通知
        self.boxfill8(Color::DarkGray, xsize - 47, ysize - 24, xsize - 4, ysize - 24);
        self.boxfill8(Color::DarkGray, xsize - 47, ysize - 23, xsize - 47, ysize - 4);
        self.boxfill8(Color::White,    xsize - 47, ysize - 3, xsize - 4, ysize - 3);
        self.boxfill8(Color::White,    xsize - 3, ysize - 24, xsize - 3, ysize - 3);
    }

    pub fn boxfill8(&mut self, color: Color, x0: isize, y0: isize, x1: isize, y1: isize)
    {
        for y in y0..=y1
        {
            for x in x0..=x1
            {
                let ptr = unsafe { &mut *((self.vram as *mut u8).offset(y * self.scrnx as isize + x)) };
                *ptr = color as u8;
            }
        }
    }

    fn set_palette(&self)
    {
        let rgb = &RGB_TABLE;
        let eflags = load_eflags();
        out8(0x03c8, 0);

        for i in 0..16
        {
            out8(0x03c9, rgb[i as usize][0] / 4);
            out8(0x03c9, rgb[i as usize][1] / 4);
            out8(0x03c9, rgb[i as usize][2] / 4);
        }
    }
}