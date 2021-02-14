#![deny(warnings)]
#![no_main]
#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use alloc::{vec::Vec, vec};

use cortex_m_rt::entry;
use alloc_cortex_m::CortexMHeap;
use panic_halt as _;
use stm32h7xx_hal::hal::digital::v2::OutputPin;
use stm32h7xx_hal::{pac, prelude::*};

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[entry]
fn main() -> ! {
    // Init the allocator
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, 1024) }

    // Test the heap allocator
    let mut data = Vec::new();
    for i in 0..5 {
        data.push(i);
    }
    assert_eq!(data, vec![0, 1, 2, 3, 4]);

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.mhz()).freeze(pwrcfg, &dp.SYSCFG);

    let gpioc  = dp.GPIOC.split(ccdr.peripheral.GPIOC);

    // Configure PC1, PC2, PC3 as output.
    let mut led_0 = gpioc.pc0.into_push_pull_output();
    let mut led_1 = gpioc.pc1.into_push_pull_output();
    let mut led_2 = gpioc.pc2.into_push_pull_output();
    
    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    
    loop {
        led_0.set_high().unwrap();
        delay.delay_ms(500_u16);
        
        led_0.set_low().unwrap();
        delay.delay_ms(500_u16);

        led_1.set_high().unwrap();
        delay.delay_ms(500_u16);

        led_1.set_low().unwrap();
        delay.delay_ms(500_u16);

        led_2.set_high().unwrap();
        delay.delay_ms(500_u16);
        
        led_2.set_low().unwrap();
        delay.delay_ms(500_u16);
    }
}
