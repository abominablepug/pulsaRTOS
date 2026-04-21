#![no_std]
#![no_main]

use core::panic::PanicInfo;

static PULSARTOS: &[u8] =
    b"Welcome to PulsaRTOS!\nA simple real-time operating system written in Rust.";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    let mut col = 0;
    let mut row = 0;

    for &byte in PULSARTOS.iter() {
        if byte == b'\n' {
            col = 0;
            row += 1;
        } else {
            let offset = (row * 80 + col) * 2;
            unsafe {
                *vga_buffer.offset(offset) = byte;
                *vga_buffer.offset(offset + 1) = 0xb;
            }

            col += 1;

            if col >= 80 {
                col = 0;
                row += 1;
            }
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
