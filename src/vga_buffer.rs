use core::fmt;
use core::ptr::Unique;
use spin::Mutex;
use volatile::Volatile;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Color {
    //Each of the possible VGA Colors
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

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        //A function to create a new Color Code object by passing in
        ColorCode((background as u8) << 4 | (foreground as u8)) //Foreground and background colors
    }
    pub const fn from_num(color: &u8) -> ColorCode {
        ColorCode(*color)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25; //VGA Width
const BUFFER_WIDTH: usize = 80; //VGA Height

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), //If newline char, call newline function
            _ => {
                //otherwise
                if self.column_position >= BUFFER_WIDTH {
                    //if position is greater than 80, newline
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1; //Set row to 79
                let col = self.column_position; //And col to current column

                let color_code = self.color_code; //let colorcode = same colorcode
                self.buffer().chars[row][col].write(ScreenChar {
                    //Set current position to char
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1; //Make position move up by one
            }
        }
    }
    pub fn set_color(&mut self, color: ColorCode) {
        self.color_code = color;
    }

    fn buffer(&mut self) -> &mut Buffer {
        //Returns mutable buffer
        unsafe { self.buffer.as_mut() }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let buffer = self.buffer();
                let character = buffer.chars[row][col].read();
                buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn set_pos(&mut self, xpos: usize, ypos: usize, character: char) {
        let color = self.color_code;
        if xpos < BUFFER_WIDTH && ypos < BUFFER_HEIGHT {
            let buffer = self.buffer();
            buffer.chars[ypos][xpos].write(ScreenChar {
                //Set current position to char
                ascii_character: character as u8,
                color_code: color,
            });
        } else {
            for byte in "Sorry, you can't write to a location outside the framebuffer".bytes() {
                self.write_byte(byte)
            }
            self.write_byte('\n' as u8) //New line
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer().chars[row][col].write(blank);
        }
    }
}

/*IMPLEMENTING MACROS*/
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}

pub static VGAWRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::White, Color::Black),
    buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
});

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga_buffer::print(format_args!($($arg)*));
    });
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    VGAWRITER.lock().write_fmt(args).unwrap();
}

macro_rules! setcolor {
    ($col:expr) => {
        set_color(ColorCode::from_num($col))
    };
    ($col1:expr, $col2:expr) => {
        set_color(ColorCode::new($col1, $col2))
    };
}
pub fn set_color(color: ColorCode) {
    VGAWRITER.lock().set_color(color);
}

macro_rules! setposition {
    ($xpos:expr, $ypos:expr) => {
        set_pos($xpos, $ypos, ' ')
    };
    ($xpos:expr, $ypos:expr, $chara:expr) => {
        set_pos($xpos, $ypos, $chara)
    };
}
pub fn set_pos(xpos: usize, ypos: usize, character: char) {
    VGAWRITER.lock().set_pos(xpos, ypos, character);
}

pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
    }
}
/*END OF IMPLEMENTING MACROS*/
