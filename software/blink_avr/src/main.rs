// makes L6 LED blink

#![no_std]
#![no_main]

use panic_halt as _;

use atmega_hal::delay::Delay;
use embedded_hal::blocking::delay::DelayMs;

type ClockSpeed = atmega_hal::clock::MHz8;

#[no_mangle]
fn main() -> ! {
    let peripherals = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(peripherals);

    let mut delay = Delay::<ClockSpeed>::new();

    let mut led = pins.pd5.into_output();

    loop {
        led.set_high();
        delay.delay_ms(300_u16);
        led.set_low();
        delay.delay_ms(300_u16);
    }
}
