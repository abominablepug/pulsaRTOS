#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pulsaRTOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use pulsaRTOS::println;
use pulsaRTOS::task::{Task, executor::Executor, keyboard};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use pulsaRTOS::allocator;
    use pulsaRTOS::memory::{self, BootInfoFrameAllocator};

    println!("Welcome to PulsaRTOS!\nA simple real-time operating system written in Rust.\n");

    pulsaRTOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();

    println!("Crash Prevented");
    pulsaRTOS::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    pulsaRTOS::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pulsaRTOS::test_panic_handler(info)
}

async fn async_number() -> u32 {
    43
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
