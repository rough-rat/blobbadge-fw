
use py32_hal::gpio::Flex;

pub struct Charlie<'a> {
    pub pin_list: &'a[Flex<'a>],
    pub buf: [u8; 4],
}

impl <'a> Charlie<'a> {
    pub fn new(pins: &'a[Flex<'_>]) -> Charlie<'a> {
        let testbuf: [u8; 4] = [0, 0, 0, 0];
        // Charlie { pins, testbuf }
        Charlie { pin_list: &pins, buf: testbuf }
    }

    pub fn draw(&mut self) {
    
    }
}
        



// async fn set_flex_random(led_pins: &mut [Flex<'_>], rand: &mut i32) {
//     const M:i32 = 65537;
//     const A:i32 = 75;
//     const C:i32 = 74;
//     for i in led_pins.iter_mut() {
//         Timer::after_millis(10).await;
//         *rand = (*rand * A + C) % M;
//         match *rand % 3 {
//             0 => {
//                 i.set_as_output(Speed::Low);
//                 i.set_low();
//             }
//             1 => {
//                 i.set_as_output(Speed::Low);
//                 i.set_high();
//             }
//             _ => i.set_as_input(py32_hal::gpio::Pull::None),
//         }
//     }
// }



    // let mut rand = 0;
    // const M:i32 = 65537;
    // const A:i32 = 75;
    // const C:i32 = 74;
    
    // loop {
    //     set_flex_random(&mut led_pins, &mut rand).await;
    //     set_flex_random(&mut led_white, &mut rand).await;
    //     // Timer::after_millis(1000).await;
    // }