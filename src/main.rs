#![no_std] // Set rust-analyzer.checkOnSave.allTargets to false
#![no_main] // disbales rusts default entry point
mod vga_buffer;

use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello World!"; *** this was always outside _start

//Below code was inside _start it is the most basic way to print to the screen

// vga_buffer::print_something();
// let vga_buffer = 0xb8000 as *mut u8;

// for (i, &byte) in HELLO.iter().enumerate() {
//      This will print hello world to the screen
//      unsafe {
//          *vga_buffer.offset(i as isize * 2) = byte;
//          *vga_buffer.offset(i as isize * 2 + 1) = 0xb
//      }
// }

// Entry point into OS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Booted").unwrap();
    write!(vga_buffer::WRITER.lock(), "numbers {} and {}", 1, 2).unwrap();
    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
