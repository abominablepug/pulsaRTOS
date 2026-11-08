# PulsaRTOS

[![Rust](https://img.shields.io/badge/Rust-Nightly-orange?logo=rust)](https://www.rust-lang.org/)
[![Target Architecture](https://img.shields.io/badge/Architecture-x86__64--bare--metal-blue)](x86_64-pulsaRTOS.json)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-green)](#license)

A freestanding, bare-metal x86_64 operating system kernel written in Rust. It notably includes fault resilience, physical & virtual memory management, and deterministic execution.

add image here

---

add video here

## Overview

PulsaRTOS is a 64-bit operating system kernel made for `x86_64` architecture without requiring an underlying operating system or standard C runtime (`#![no_std]`, `#![no_main]`).

I built PulsaRTOS to explore and learn more about systems engineering, kernel-level development, and real-time computing standards. The kernel implements CPU descriptor tables, hardware interrupt handling, 4-level paging, multiple dynamic memory allocators (including a fixed-size block allocator currently used for reliability), and a QEMU test automation harness using serial UART.

### Key Features
- Hardware-Isolated Fault Handling: Interrupt Stack Table (IST) inside the Task State Segment (TSS) which prevents any kernel stack overflows from snowballing into triple faults.
- Hierarchical Virtual Memory: 4-level paging using recursive address translation, physical memory mapping, and a custom frame allocator.
- Tri-Tier Heap Allocator: Interchangeable memory allocators—Bump Allocator, Linked List Free Allocator, and a Fixed-Size Block Allocator (Power-of-2 slabs using linked-list as a fallback).
- Integration Testing: Test framework communicating across a 16550 UART serial bus with QEMU exit codes via the `isa-debug-exit` device.

---

more images

## Technical Architecture

```
                                  +---------------------------------------+
                                  |         PulsaRTOS Kernel Core         |
                                  |    (Dynamic Alloc, Panics, Strings)   |
                                  +-------------------+-------------------+
                                                      |
                   +----------------------------------+----------------------------------+
                   |                                  |                                  |
                   v                                  v                                  v
+------------------------------------+  +----------------------------+  +---------------------------------+
|      Memory Management Unit        |  |     Interrupt Subsystem    |  |          I/O Subsystem          |
+------------------------------------+  +----------------------------+  +---------------------------------+
| * 4-Level Offset Page Tables       |  | * 8259 Dual PIC (Cascade)  |  | * 80x25 VGA Text Buffer         |
| * Usable PhysFrame Allocator       |  | * Timer (PIT) Interrupts   |  | * Thread-Safe Spinlock Writer   |
| * Custom Allocators:               |  | * PS/2 Scancode Parser     |  | * 16550 UART Serial Controller  |
|   - Bump Allocator                 |  | * IDT Exception Handlers   |  | * isa-debug-exit QEMU Device    |
|   - Linked List Free-List          |  | * TSS & IST Stack Guards   |  +---------------------------------+
|   - Fixed-Size Block (Power of 2)  |  +----------------------------+
+------------------------------------+
                   |                                  |                                  |
                   +----------------------------------+----------------------------------+
                                                      |
                                                      v
                                  +---------------------------------------+
                                  |         x86_64 Bare Metal / QEMU      |
                                  +---------------------------------------+
```

---

## Core Subsystems

### 1. Memory Management & Paging
- **4-Level Page Tables**: Implements x86_64 paging using `OffsetPageTable` to map complete physical memory to a virtual address offset.
- **Frame Allocation**: The `BootInfoFrameAllocator` translates raw BIOS memory map regions, creating non-overlapping, page-aligned `PhysFrame<Size4KiB>` chunks.
- **Virtual Memory Translations**: Converts virtual addresses to physical frames and facilitates dynamic page mapping with `PageTableFlags::PRESENT | PageTableFlags::WRITABLE`.

### 2. Custom Heap Allocators
PulsaRTOS integrates with the Rust `alloc` collection (`Box`, `Vec`, `Rc`, `Arc`) through a `#[global_allocator]` interface and comes with three different implementations that offer varying pros and cons:
- **Bump Allocator (`src/allocator/bump.rs`)**: Linear allocation counter; ultra-fast $O(1)$ allocation without individual deallocation.
- **Linked-List Allocator (`src/allocator/linked_list.rs`)**: Enables memory reclamation across variable-sized allocations by traversing with alignment adjustments.
- **Fixed-Size Block Allocator (`src/allocator/fixed_size_block.rs`)**: High-performance slab-like allocator managing power-of-two block sizes (16, 32, 64, 128, 256, 512, 1024, 2048 bytes). Minimizes memory fragmentation and provides near $O(1)$ allocation (Linked-List Allocator fallback to ensure data security).

### 3. CPU Tables & Fault Resilience
- **Global Descriptor Table (GDT)**: Configures data segment selectors.
- **Task State Segment (TSS) & IST**: Configures a 20 KiB Interrupt Stack Table (IST) for double faults. When stack overflow occurs, the CPU switches to a known-clean stack frame, preventing a fatal triple fault crash.
- **Interrupt Descriptor Table (IDT)**: Captures CPU exceptions including Breakpoint, Page Faults, and Double Faults.

### 4. Interrupt Handling & Hardware Drivers
- **PIT Timer**: Captures periodic clock ticks.
- **PS/2 Keyboard Driver**: Interrupt-driven scancode reader decoded into ASCII keystrokes.
- **VGA Text Driver**: Directly manages memory-mapped I/O at port `0xb8000` supporting an 80×25 character display, color codes, carriage returns, newlines, and scrolling.

---

## Automated Testing

Testing bare-metal kernels requires bridging the virtual machine with the host operating system which PulsaRTOS does through the following:

1. **Serial Port Driver (`src/serial.rs`)**: Initializes a UART 16550 standard serial interface.
2. **Headless Execution**: Kernel test outputs are piped directly from guest serial to host.
3. **QEMU Auto-Exit**: Interacts with the `isa-debug-exit` device:
   - `0x10` : **Success**
   - `0x11` : **Failure**

### Integration Test Suite (`tests/`)
- `basic_boot.rs`: Verifies successful boot and VGA buffer output.
- `heap_allocation.rs`: Exhaustive test of `Box`, dynamic `Vec` sizing, and allocation saturation over 100 KiB heap bounds.
- `stack_overflow.rs`: Deliberately induces infinite recursion to verify TSS / IST stack switching.
- `should_panic.rs`: Validates panic handler assertion trapping and error reporting.

---

## Build and Run

Ensure you have the following installed:

**Rust Nightly**:
  ```bash
  rustup default nightly
  rustup component add rust-src llvm-tools-preview
  ```
**QEMU**:
  ```bash
  # Debian / Ubuntu
  sudo apt install qemu-system-x86

  # Arch Linux
  sudo pacman -S qemu-system-x86

  # macOS (Homebrew)
  brew install qemu
  ```
**Bootimage**:
  ```bash
  cargo install bootimage
  ```
---

Once installed do the following:

1. Clone the repository:
   ```bash
   git clone https://github.com/EliNahoum/pulsaRTOS.git
   cd pulsaRTOS
   ```

2. Run in QEMU:
   ```bash
   cargo run # escape to exit
   ```
3. Run Bare-Metal Automated Test Suite:
   ```bash
   cargo test
   ```

4. Build a Bootable Disk Image:
   ```bash
   cargo bootimage
   ```
   The compiled binary will be generated in `target/x86_64-pulsaRTOS/debug/bootimage-pulsaRTOS.bin`.

---

## Roadmap & Active To-Do List

### Current Focus
- [ ] Multi-Core CPU Parallel Processing
- [ ] Linked List Allocator Merging
- [ ] Dedicated Standalone Bootloader
- [ ] UEFI Transition

### Future Milestones
- [ ] Preemptive Multitasking
- [ ] Async / Await Executor
- [ ] User Mode & System Calls
- [ ] Filesystem
- [ ] Space-Themed Shell / Terminal Interface

---

## License

This project is licensed under the [Apache 2.0 License](LICENSE-APACHE) at your option.
