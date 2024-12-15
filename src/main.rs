#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use py32_hal::gpio::{Flex, Speed};

use {defmt_rtt as _, panic_halt as _};

fn set_flex_random(led_pins: &mut [Flex], rand: &mut i32) {
    const M:i32 = 65537;
    const A:i32 = 75;
    const C:i32 = 74;
    for i in led_pins.iter_mut() {
        *rand = (*rand * A + C) % M;
        match *rand % 3 {
            0 => {
                i.set_as_output(Speed::Low);
                i.set_low();
            }
            1 => {
                i.set_as_output(Speed::Low);
                i.set_high();
            }
            _ => i.set_as_input(py32_hal::gpio::Pull::None),
        }
    }
}

async fn charlieplex_test() {
    let p = py32_hal::init(Default::default());

    let mut led_pins = [
        Flex::new(p.PA0),
        Flex::new(p.PA1),
        Flex::new(p.PA2),
        Flex::new(p.PA3),
        Flex::new(p.PA4),
        Flex::new(p.PA5),
        Flex::new(p.PA6),
    ];

    let mut led_white = [
        Flex::new(p.PA7),
        Flex::new(p.PA8),
        Flex::new(p.PA12),
        Flex::new(p.PA13)
    ];

    let mut rand = 0;
    const M:i32 = 65537;
    const A:i32 = 75;
    const C:i32 = 74;
    
    loop {
        set_flex_random(&mut led_pins, &mut rand);
        set_flex_random(&mut led_white, &mut rand);
        Timer::after_millis(1000).await;
    }
}

// main is itself an async function.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");
    charlieplex_test().await;
}
