#![no_std] // Set rust-analyzer.checkOnSave.allTargets to false
#![no_main] // disbales rusts default entry point

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
