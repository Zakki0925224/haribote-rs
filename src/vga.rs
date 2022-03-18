use core::fmt;

use crate::{asm::{load_eflags, out8}, font::FONTPACK};

const FONT_HEIGHT: usize = 16;
const FONT_WIDTH: usize = 8;
const CURSOR_HEIGHT: usize = 16;
const CURSOR_WIDTH: usize = 16;

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
    pub vram: &'static mut u8,
    pub mouse: [[Color; CURSOR_WIDTH]; CURSOR_HEIGHT]
}

impl Screen
{
    pub fn new() -> Screen
    {
        return Screen
        {
            scrnx: unsafe { *(0x0ff4 as *mut i16) },
            scrny: unsafe { *(0x0ff6 as *mut i16) },
            vram: unsafe { &mut *(*(0x0ff8 as *const i32) as *mut u8) },
            mouse: [[Color::DarkCyan; CURSOR_WIDTH]; CURSOR_HEIGHT]
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

        // カーソル
        self.init_cursor();
        self.putblock8(self.mouse, (self.scrnx / 2) as isize, (self.scrny / 2) as isize);
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

    pub fn putfont8(&mut self, x: isize, y: isize, color: Color, font: &[u8])
    {
        let color = color as u8;

        for i in 0..font.len()
        {
            let f = font[i];
            let offset = (y + i as isize) * self.scrnx as isize + x;

            if (f & 0x80) != 0 { *unsafe { &mut *((self.vram as *mut u8).offset(offset)) } = color; }
            if (f & 0x40) != 0 { *unsafe { &mut *((self.vram as *mut u8).offset(offset + 1)) } = color; }
            if (f & 0x20) != 0 { *unsafe { &mut *((self.vram as *mut u8).offset(offset + 2)) } = color; }
            if (f & 0x10) != 0 { *unsafe { &mut *((self.vram as *mut u8).offset(offset + 3)) } = color; }
            if (f & 0x08) != 0 { *unsafe { &mut *((self.vram as *mut u8).offset(offset + 4)) } = color; }
            if (f & 0x04) != 0 { *unsafe { &mut *((self.vram as *mut u8).offset(offset + 5)) } = color; }
            if (f & 0x02) != 0 { *unsafe { &mut *((self.vram as *mut u8).offset(offset + 6)) } = color; }
            if (f & 0x01) != 0 { *unsafe { &mut *((self.vram as *mut u8).offset(offset + 7)) } = color; }
        }
    }

    pub fn putfont8_asc(&mut self, x: isize, y: isize, color: Color, strs: &str)
    {
        let mut x = x;

        for i in 0..strs.len()
        {
            let c = strs.as_bytes()[i] as usize;
            self.putfont8(x, y, color, &FONTPACK[c]);
            x += 8;
        }
    }

    fn putblock8(&mut self, img: [[Color; 16]; 16], x: isize, y: isize)
    {
        for i in 0..16
        {
            for j in 0..16
            {
                let ptr = unsafe { &mut *((self.vram as *mut u8).offset((y + i) * self.scrnx as isize + (x + j))) };
                *ptr = img[i as usize][j as usize] as u8;
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

    fn init_cursor(&mut self)
    {
        let cursor: [[u8; CURSOR_WIDTH]; CURSOR_HEIGHT] =
        [
            *b"**************..",
            *b"*OOOOOOOOOOO*...",
            *b"*OOOOOOOOOO*....",
            *b"*OOOOOOOOO*.....",
            *b"*OOOOOOOO*......",
            *b"*OOOOOOO*.......",
            *b"*OOOOOOO*.......",
            *b"*OOOOOOOO*......",
            *b"*OOOO**OOO*.....",
            *b"*OOO*..*OOO*....",
            *b"*OO*....*OOO*...",
            *b"*O*......*OOO*..",
            *b"**........*OOO*.",
            *b"*..........*OOO*",
            *b"............*OO*",
            *b".............***"
        ];

        for y in 0..CURSOR_WIDTH
        {
            for x in 0..CURSOR_HEIGHT
            {
                match cursor[y][x]
                {
                    b'*' => self.mouse[y][x] = Color::Black,
                    b'O' => self.mouse[y][x] = Color::White,
                    _ => ()
                }
            }
        }
    }
}

pub struct ScreenWriter
{
    initial_x: usize,
    x: usize,
    y: usize,
    color: Color,
    screen: Screen,
}

impl ScreenWriter
{
    pub fn new(screen: Screen, color: Color, x: usize, y: usize) -> ScreenWriter
    {
        return ScreenWriter
        {
            initial_x: x,
            x,
            y,
            color,
            screen,
        };
    }

    fn newline(&mut self)
    {
        self.x = self.initial_x;
        self.y = self.y + FONT_HEIGHT;
    }
}

impl fmt::Write for ScreenWriter
{
    fn write_str(&mut self, s: &str) -> fmt::Result
    {
        let str_bytes = s.as_bytes();
        let height = self.screen.scrny as usize;
        let width = self.screen.scrnx as usize;

        for i in 0..str_bytes.len()
        {
            if str_bytes[i] == b'\n'
            {
                self.newline();
                continue;
            }
            if str_bytes[i] == b'\t'
            {
                self.write_str("    ");
                continue;
            }

            if self.x + FONT_WIDTH < width && self.y + FONT_HEIGHT < height
            {
                self.screen
                    .putfont8(self.x as isize, self.y as isize, self.color, &FONTPACK[str_bytes[i] as usize]);
            }
            else if self.y + FONT_HEIGHT * 2 < height
            {
                self.newline();
                self.screen
                .putfont8(self.x as isize, self.y as isize, self.color, &FONTPACK[str_bytes[i] as usize]);
            }
            if self.x + FONT_WIDTH < width
            {
                self.x = self.x + FONT_WIDTH;
            }
            else if self.y + FONT_HEIGHT < height
            {
                self.newline();
            }
            else
            {
                self.x = width;
                self.y = height;
            }
        }

        return Ok(());
    }
}