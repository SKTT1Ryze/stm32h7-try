#![no_main]
#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use alloc::{vec::Vec, vec};
use core::fmt::Write;
#[allow(unused_imports)]
use core::cell::{Cell, RefCell};

use cortex_m_rt::entry;
use alloc_cortex_m::CortexMHeap;
use panic_halt as _;
use cortex_m::interrupt::{free, Mutex};
use cortex_m::peripheral::NVIC;
use stm32h7xx_hal::hal::digital::v2::OutputPin;
use stm32h7xx_hal::{pac, prelude::*, interrupt};
use stm32h7xx_hal::gpio::{Edge, ExtiPin, Input, Output, PullUp, PushPull};
// Key pin
use stm32h7xx_hal::gpio::gpioc::{PC0, PC13};

#[allow(unused_imports)]
use nb::block;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

// Semaphore for synchronization
// static SEMAPHORE: Mutex<Cell<bool>> = Mutex::new(Cell::new(true));

static KEY_PIN: Mutex<RefCell<Option<PC13<Input<PullUp>>>>> =
    Mutex::new(RefCell::new(None));
static LED: Mutex<RefCell<Option<PC0<Output<PushPull>>>>> =
    Mutex::new(RefCell::new(None));
static mut COUNTER: usize = 0;

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

    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.mhz()).freeze(pwrcfg, &dp.SYSCFG);

    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpioc  = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    
    // Configure PC0, PC1, PC2 as output.
    let mut led_0 = gpioc.pc0.into_push_pull_output();
    let mut led_1 = gpioc.pc1.into_push_pull_output();
    let mut led_2 = gpioc.pc2.into_push_pull_output();
    let mut led_3 = gpioc.pc3.into_push_pull_output();
    
    // Set the led down first
    led_0.set_low().unwrap();
    led_1.set_low().unwrap();
    led_2.set_low().unwrap();
    led_3.set_low().unwrap();

    let tx = gpioa.pa9.into_alternate_af7();
    let rx = gpioa.pa10.into_alternate_af7();

    let serial = dp.USART1.serial(
        (tx, rx),
        115_200.bps(),
        ccdr.peripheral.USART1,
        &ccdr.clocks
    ).unwrap();

    let (mut tx, mut _rx) = serial.split();
    
    // Get true random number generator
    let mut rng = dp.RNG.constrain(ccdr.peripheral.RNG, &ccdr.clocks);
    let mut random_bytes = [0u16; 3];

    rng.fill(&mut random_bytes).unwrap();

    // Get the delay provider.
    #[allow(unused_variables)]
    let mut delay = cp.SYST.delay(ccdr.clocks);

    // Push button configuration
    let mut syscfg = dp.SYSCFG;
    let mut exti = dp.EXTI;

    let mut key = gpioc.pc13.into_pull_up_input();
    key.make_interrupt_source(&mut syscfg);
    key.trigger_on_edge(&mut exti, Edge::RISING);
    key.enable_interrupt(&mut exti);

    // Save information needed by the interrupt handlers to the global variable
    free(|cs| {
        KEY_PIN.borrow(cs).replace(Some(key));
        LED.borrow(cs).replace(Some(led_0));
    });

    // Enable the button interrupts
    unsafe {
        cp.NVIC.set_priority(interrupt::EXTI15_10, 1);
        NVIC::unmask::<interrupt>(interrupt::EXTI15_10);
    }

    // core::fmt::Write is implemented for tx.
    writeln!(tx, "Hello, world!").unwrap();

    loop {
        let random_element: Result<u32, _> = rng.gen();
        match random_element {
            Ok(random) => {
                match random % 3 {
                    0 => {
                        led_1.set_high().unwrap();
                        delay.delay_ms(500_u16);
                        
                        led_1.set_low().unwrap();
                        delay.delay_ms(500_u16);
                    },
                    1 => {
                        led_2.set_high().unwrap();
                        delay.delay_ms(500_u16);

                        led_2.set_low().unwrap();
                        delay.delay_ms(500_u16);
                    },
                    2 => {
                        led_3.set_high().unwrap();
                        delay.delay_ms(500_u16);
                        
                        led_3.set_low().unwrap();
                        delay.delay_ms(500_u16);
                    },
                    _ => unreachable!()
                }
            },
            Err(_) => panic!()
        }

        // Echo what is received on the serial link.
        // let received = block!(rx.read()).unwrap();
        // block!(tx.write(received)).ok();
    }
}

fn toggle_led() {
    free(|cs| {
        if let Some(b) = LED.borrow(cs).borrow_mut().as_mut() {
            if unsafe { COUNTER % 2 } == 0 {
                b.set_high().unwrap();
            } else {
                b.set_low().unwrap();
            }
        }
    });
    unsafe { COUNTER += 1; }
}

#[interrupt]
fn EXTI15_10() {
    toggle_led();
    free(|cs| {
        if let Some(b) = KEY_PIN.borrow(cs).borrow_mut().as_mut() {
            b.clear_interrupt_pending_bit()
        }
    });
}