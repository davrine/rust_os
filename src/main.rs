#![no_std] // Set rust-analyzer.checkOnSave.allTargets to false
#![no_main] // disbales rusts default entry point

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
