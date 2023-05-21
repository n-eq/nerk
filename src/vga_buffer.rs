use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

static MEM_VIDEO_ADDRESS: u32 = 0xB8000;

pub mod colors {
    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Color {
        Black = 0,
        Blue = 1,
        Green = 2,
        Cyan = 3,
        Red = 4,
        Magenta = 5,
        Brown = 6,
        LightGray = 7,
        DarkGray = 8,
        LightBlue = 9,
        LightGreen = 10,
        LightCyan = 11,
        LightRed = 12,
        Pink = 13,
        Yellow = 14,
        White = 15,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ColorCode(u8);

    impl ColorCode {
        pub fn new(foreground: Color, background: Color) -> ColorCode {
            ColorCode((background as u8) << 4 | (foreground as u8))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: colors::ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: colors::ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn clear_line(&mut self, line: usize) {
        if line >= BUFFER_HEIGHT {
            return;
        }

        for c in self.buffer.chars[line].iter_mut() {
            c.ascii_character = ' ' as u8;
        }
    }

    #[must_use]
    pub fn new(fg: colors::Color, bg: colors::Color) -> Self {
        let mut w = Writer {
            column_position: 0,
            color_code: colors::ColorCode::new(fg, bg),
            buffer: unsafe { &mut *(MEM_VIDEO_ADDRESS as *mut Buffer) },
        };
        for line in 0..BUFFER_HEIGHT {
            w.clear_line(line);
        }
        w
    }

    pub fn new_line(&mut self) {
        for line in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[line - 1][col] = self.buffer.chars[line][col];
            }
        }
        self.column_position = 0;
        self.clear_line(BUFFER_HEIGHT - 1);
    }

    pub fn write(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };

                self.column_position += 1;
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> =
        Mutex::new(Writer::new(colors::Color::White, colors::Color::Black),);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
