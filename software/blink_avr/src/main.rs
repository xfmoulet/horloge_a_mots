// makes C6 LED blink using very low-level code (no HAL so far)

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
        // directly configure bit 5 of PORTC as output
        b.dp.PORTC.ddrc.write(|w| w.pc5().set_bit());
        b
    }
    fn set(&self, b : bool) {
        self.dp.PORTC.portc.write(
            |w| if b { w.pc5().set_bit() } else { w.pc5().clear_bit() }
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
