#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER
        .lock()
        .write_str("Welcome to PulsaRTOS!\nA simple real-time operating system written in Rust.\n")
        .unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        "Insertions like {} and {} have now been added!",
        7 * 6,
        1.337
    )
    .unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
