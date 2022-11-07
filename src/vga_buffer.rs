use spin::Mutex;
use volatile::Volatile; // The reason why Volitile is required is because the copiler sees that we only write to ir and never read from it
                        // the rust compiler doesnt know that were not accessing ram but instead accessing the vga memorybuffer so its possible that newer versions of the
                        // rust compiler might erronously optimze this by thinking its usless and removing it

use lazy_static::lazy_static;
lazy_static! { // for clarifcation as to why this is usfull look at https://os.phil-opp.com/vga-text-mode/ search for the keyword static
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {// ref keyword specifies that WRITER will be a reference
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // repr(u8) attribute, each enum variant is stored as a u8
            // Actually 4 bits would be sufficient, but Rust doesn’t have a u4 type
            // it doesnt know that this is a vga memory

pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
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
#[repr(transparent)] // To ensure that the ColorCode has the exact same data layout as a u8, we use the repr(transparent) attribute.
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        // The << operatir shifts all the bits in background into foreground
        ColorCode((background as u8) << 4 | (foreground as u8)) // << 4 shifting all bits in background to the left bits so 00000001 = 00010000
    } // the | is a bitwise or it compares every bit and ors it https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_OR
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // Since the field ordering in default structs is undefined in Rust, we need the repr(C) attribute. It guarantees that the struct’s fields are laid out exactly like in a C struct and thus guarantees the correct field ordering.
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)] // repr(transparent) again to ensure that it has the same memory layout as its single field.
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), // If new line character write new line
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line(); // if the position of the column is greater than the width (max number of columns) then new line
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte), // This means every ascii character after 20 and before delete is printed inclusing newline look up ascii chart for clarification
                _ => self.write_byte(0xfe), // this means anythoer invlaid ascii character will print square
            }
        }
    }
    // *** 0xb8000 this memory adress is reserved for the vga buffer thats how we are able to print because the cpu reads from here specificly for vga ***

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(char);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}
use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Below code was test function for printing ascii chars to screen

// pub fn print_something() {
//     use core::fmt::Write;
//     let mut writer = Writer {
//         column_position: 0,
//         color_code: ColorCode::new(Color::Yellow, Color::Black),
//         buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }, // It first creates a new Writer that points to the VGA buffer at 0xb8000. The syntax for this might seem a bit strange: First, we cast the integer 0xb8000 as a mutable raw pointer. Then
//                                                            //we convert it to a mutable reference by dereferencing it (through *) and immediately borrowing it again (through &mut)
//     };
//     writer.write_byte(b'H');
//     writer.write_string("ello ");
//     writer.write_string("Wörld!");
//     write!(writer, "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
// }



