`2026-04-23`
* Progress: Testing suite including unit tests, implementation tests, and panic tests have all been implemented and tested (pun intended)
* Issue: Printing testing suite output to terminal requires a way for the OS to communicate with the host system
* Solution: The `serial_print!` and `serial_println!` macros were redefined to read from the first SERIAL port of the OS to the host computer allowing for proper test output without the need for a display.

---

`2026-04-22`
* Progress: Created new `vga_buffer` file with logic to reconfigure and simplify the `print!` and `println!` macros to print to the buffer rather than to the terminal.
* Issue: Syntax regarding reconfiguring the macros was difficult to follow along with and debug
* Solution: Testing different configurations and further investigating the nuances of Rust's syntax helped root out the issue

---

`2026-04-21`
* Progress: Successfully printed first characters to the VGA buffer
* Issue: Printing to multiple lines within the VGA buffer proved to be difficult requiring additional logic
* Solution: Changing the location of the cursor in special scenarios and when the next byte was a `\n` character

---

`2026-04-20`
* Progress: Created the initial project and got it to compile
* Issue: Using the .json file for the OS target was continuously failing
* Solution: I installed the nightly version of rustc
