#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Welcome to PulsaRTOS!\nA simple real-time operating system written in Rust.\n");
    panic!("NOW ABLE TO PANIC!!!");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
