#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly

use cortex_m_rt::entry;
#[allow(unused_imports)]
use stm32h7xx_hal::interrupt;

#[entry]
fn main() -> ! {
    // initialization
    loop {
        // application logic
    }
}