#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use py32_hal::gpio::{Flex, Speed};

use {defmt_rtt as _, panic_halt as _};

mod charlie;


async fn charlieplex_test() {
    let p = py32_hal::init(Default::default());

    let mut led_pins: [Flex<'_>; 7] = [
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

    let mut test_charlie = charlie::Charlie::new(&led_pins);

    // loop {
    test_charlie.draw();
    Timer::after_millis(1000).await;
    drop(test_charlie);
}

// main is itself an async function.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");


    charlieplex_test().await;
}
