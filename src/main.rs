#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    loop {}
}

// Define the entry point for the program
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This is the entry point for the program
    loop {}
}

fn main() {
    // This is the main function
}