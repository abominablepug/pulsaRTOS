#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pulsaRTOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use pulsaRTOS::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Welcome to PulsaRTOS!\nA simple real-time operating system written in Rust.\n");

    pulsaRTOS::init();

    unsafe {
        *(0xbadadd as *mut u32) = 42;
    }

    #[cfg(test)]
    test_main();

    println!("Crash Prevented");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pulsaRTOS::test_panic_handler(info)
}
