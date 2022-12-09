// makes L6 LED blink using very low-level code (no HAL so far)

#![no_std]
#![no_main]


use panic_halt as _;
use avr_device::atmega8::Peripherals;

// absolutely inaccurate but eh
#[inline(always)]
fn delay(ms: u32) {
    for _ in 0..ms * 1000 {
        core::hint::black_box({}) // empty loop
    }
}

struct BoardLED {
    dp: Peripherals,
}

impl BoardLED {
    fn new() -> BoardLED {
        let b = BoardLED{ dp: Peripherals::take().unwrap() };
        // directly configure bit 5 of PORTD as output
        b.dp.PORTD.ddrd.write(|w| w.pd5().set_bit());
        b
    }
    fn set(&self, b : bool) {
        self.dp.PORTD.portd.write(
            |w| if b { w.pd5().set_bit() } else { w.pd5().clear_bit() }
        );
    }
}

#[avr_device::entry]
fn main() -> ! {
    let led = BoardLED::new();

    loop {
      delay(100);
      led.set(false);
      delay(100);
      led.set(true);
      delay(100);
      led.set(false);
      delay(500);
      led.set(true);
    }
}
