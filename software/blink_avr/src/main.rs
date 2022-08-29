#![no_std]
#![no_main]

use panic_halt as _;
//use arduino_hal::prelude::*;
use arduino_hal::port::mode::Output;
use arduino_hal::hal::port::PB5;

fn stutter_blink(led: &mut arduino_hal::port::Pin<Output,PB5>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle();
        arduino_hal::delay_ms(i as u16);
    });
}

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();

    let pins = arduino_hal::pins!(peripherals);

    let mut led = pins.d13.into_output();

    loop {
        stutter_blink(&mut led, 25);
    }
}
