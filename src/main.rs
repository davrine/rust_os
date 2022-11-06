#![no_std] // Set rust-analyzer.checkOnSave.allTargets to false
#![no_main] // disbales rusts default entry point
mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";
// Entry point into OS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //      This will print hello world to the screen
    //      unsafe {
    //          *vga_buffer.offset(i as isize * 2) = byte;
    //          *vga_buffer.offset(i as isize * 2 + 1) = 0xb
    //      }
    // }

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
