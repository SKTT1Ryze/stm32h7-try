#![no_main]
#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use alloc::{vec::Vec, vec};

use cortex_m_rt::entry;
use alloc_cortex_m::CortexMHeap;
use panic_halt as _;
use spi::NoMiso;
use stm32h7xx_hal::hal::digital::v2::OutputPin;
use stm32h7xx_hal::{pac, prelude::*, spi};
use embedded_graphics::{mono_font::{MonoTextStyle, ascii::Font10x20}, pixelcolor::BinaryColor, prelude::*, text::Text};
use embedded_graphics::primitives::{Circle, Rectangle, Triangle, PrimitiveStyle};
use epd_waveshare::{
    epd2in13_v2::{Display2in13, EPD2in13},
    graphics::{Display, DisplayRotation},
    prelude::*,
};

// use epd_waveshare::{
//     color::*,
//     epd2in9::{Display2in9, EPD2in9},
//     graphics::Display,
//     prelude::*,
// };

#[allow(unused_imports)]
use nb::block;

// RST Pin: PB11
// BUSY Pin: PB10
// DC Pin: PE15
// SPI2_SCK Pin: PB13
// SPI2_MISO Pin: PB14
// SPI2_MOSI Pin: PB15
// SPI2_CS1 Pin: PB12
// SPI3_SCK Pin: PC10
// SPI3_MISO Pin: PC11
// SPI3_MOSI Pin: PC12

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
    let ccdr = rcc
        .sys_ck(100.mhz())
        .pll1_q_ck(48.mhz())
        .freeze(pwrcfg, &dp.SYSCFG);

    // let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // Configure PC0, PC1, PC2 as output.
    let mut led_0 = gpioc.pc0.into_push_pull_output();
    let mut led_1 = gpioc.pc1.into_push_pull_output();
    let mut led_2 = gpioc.pc2.into_push_pull_output();
    
    let rst = gpiob.pb11.into_push_pull_output();
    let busy = gpiob.pb10.into_pull_down_input();
    let dc = gpioe.pe15.into_push_pull_output();
    let cs = gpiob.pb12.into_push_pull_output();

    let sck = gpiob.pb13.into_alternate_af5();
    // let miso = gpiob.pb14.into_alternate_af5();
    let mosi = gpiob.pb15.into_alternate_af5();

    // Initialise the SPI2 peripheral.
    let mut spi: spi::Spi<_, _, u8> = dp.SPI2.spi(
        (sck, NoMiso, mosi),
        spi::MODE_0,
        3.mhz(),
        ccdr.peripheral.SPI2,
        &ccdr.clocks,
    );

    // let sck = gpioc.pc10.into_alternate_af6();
    // let miso = gpioc.pc11.into_alternate_af6();
    // let mosi = gpioc.pc12.into_alternate_af6();


    // Initialise the SPI peripheral.
    // let mut spi = dp.SPI3.spi(
    //     (sck, miso, mosi),
    //     spi::MODE_0,
    //     3.mhz(),
    //     ccdr.peripheral.SPI3,
    //     &ccdr.clocks,
    // );
    
    // Initialise the e-paper
    // dc.set_low().unwrap();
    // cs.set_low().unwrap();
    // rst.set_high().unwrap();

    // cs.set_high().unwrap();

    // rst.set_high().unwrap();
    // delay.delay_ms(100_u16);
    // rst.set_low().unwrap();
    // delay.delay_ms(2_u16);
    // rst.set_high().unwrap();
    // delay.delay_ms(10_u16);

    // dc.set_low().unwrap();
    // cs.set_low().unwrap();
    // spi.write(&[0x04]).unwrap();
    // cs.set_high().unwrap();
    
    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    // Setup the device
    let mut epd = EPD2in13::new(
        &mut spi, cs, busy, dc, rst, &mut delay).unwrap();
    // Setup the graphics
    let mut display = Display2in13::default();
    
    display.set_rotation(DisplayRotation::Rotate90);
    
    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    let text_style = MonoTextStyle::new(Font10x20, BinaryColor::On);
    let display_size = display.size() - Size::new(1, 1);
    let yoffset = 10;
    
    // Draw a 3px wide outline around the display.
    Rectangle::new(Point::zero(), display_size)
        .into_styled(thick_stroke)
        .draw(&mut display).unwrap();
    
    // Draw a triangle.
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, 16 + yoffset),
    )
    .into_styled(thin_stroke)
    .draw(&mut display).unwrap();

    // Draw a filled square
    Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
        .into_styled(fill)
        .draw(&mut display).unwrap();
    
    // Draw a circle with a 3px wide stroke.
    Circle::new(Point::new(88, yoffset), 17)
        .into_styled(thick_stroke)
        .draw(&mut display).unwrap();
    
    let text = "Hello World from Rust";
    let width = text.len() as i32 * 6;
    // Draw some text
    Text::new(text, Point::new(64 - width / 2, 40))
        .into_styled(text_style)
        .draw(&mut display).unwrap();
        
    // Transfer the frame data to the epd and display it
    epd.update_frame(&mut spi, &display.buffer()).unwrap();
    epd.display_frame(&mut spi).unwrap();
    epd.sleep(&mut spi).unwrap();
    
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

    // Echo what is received on the serial link.
    // let received = block!(rx.read()).unwrap();
    // block!(tx.write(received)).ok();
}