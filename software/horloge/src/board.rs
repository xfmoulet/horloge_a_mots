use horloge::data::*;

use atmega_hal::clock;
use atmega_hal::delay::Delay;
use atmega_hal::port::mode::Output;
use atmega_hal::port::*;
use panic_halt as _; // panic handler

use atmega_hal::pac::tc2::tccr2b::CS2_A;
use atmega_hal::pac::TC2;

use embedded_hal::blocking::delay::DelayUs;

type ClockSpeed = clock::MHz16;

// the timer is 8 bit, prescaler 1024 so max rollover is 255 / 32Hz = 8s :(
// 60s = 2*2*3*5 , let's use 1s ? could be 1,2,3,4,5,6 (10 and 12 are >8s)

const PRESCALER: u32 = 1024; // also check clock setup

const ROLLOVER_SECONDS: u32 = 1;
const ROLLOVER_TICKS: u32 = 32768 / PRESCALER * ROLLOVER_SECONDS + 1;

const _: () = assert!(ROLLOVER_TICKS < 255); // we compute but check at build time
const _: () = assert!(60 % ROLLOVER_SECONDS == 0);

// use a https://docs.rust-embedded.org/book/peripherals/singletons.html ?
pub struct BoardTimer {
    tick: u16, // past value of the timer, updated at 32Hz if presecaled by 1024

    second: u8,
    hour: u8,
    min5: u8,
    minute: u8,

    delay: Delay<ClockSpeed>,
}

impl BoardTimer {
    pub fn new(timer: TC2) -> BoardTimer {
        // Set up async timer tc2
        // https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-timer.rs

        timer.tccr2a.write(|w| w.wgm2().bits(0b00)); // TODO: understand
        timer.tccr2b.write(|w| {
            // TODO: understand
            w.cs2()
                //.prescale_256()
                .variant(CS2_A::PRESCALE_1024)
                .wgm22()
                .bit(true) // TODO: what is wgm22 ?
        });

        timer
            .ocr2a
            .write(|w| unsafe { w.bits(ROLLOVER_TICKS as u8) });
        // TODO also set up the timer src on EXT OSC - timer !

        // System clock: keep default internal RC@8MHz

        // for interrupt based ticks, see https://blog.rahix.de/005-avr-hal-millis/

        BoardTimer {
            tick: 0,

            second: 0,
            minute: 0,
            min5: 0,
            hour: 0,

            delay: Delay::new(),
        }
    }

    /// this function returns the current time in RTC as H:M5:M (NOT H:M:S !) where
    ///     H is the hour 0-23
    ///     M5 is the 5-minutes number 0-11
    ///     M is the remaining minutes 0-4
    pub fn time(&self) -> (u8, u8, u8) {
        (self.hour, self.min5, self.minute)
    }

    /// delay a bit
    pub fn delay_us(&mut self, us: u16) {
        self.delay.delay_us(us)
    }

    // to be called faster than ROLLOVER_SECONDS. main thread, not in an interrupt
    pub fn update_time(&mut self) {
        /*
        let counter_value = self.timer.tcnt2.read().bits();

        if self.tick < counter_value {
            // we looped, so rollover has passed, update N seconds
            self.second += (60 / ROLLOVER_SECONDS) as u8;
        }
        self.tick = counter_value;
        */
        self.tick += 1;
        if self.tick >= 5 {
            self.tick = 0;
            self.second += 1;
        }

        if self.second >= 60 {
            self.minute += 1;
            self.second = 0;
        }
        if self.minute >= 5 {
            self.minute -= 5;
            self.min5 += 1;
        }
        if self.min5 >= 12 {
            self.min5 -= 12;
            self.hour += 1;
        }
        if self.hour >= 24 {
            self.hour -= 24;
        }
    }
}

pub struct BoardLEDs {
    line1: atmega_hal::port::Pin<Output, PD0>,
    line2: atmega_hal::port::Pin<Output, PD1>,
    line3: atmega_hal::port::Pin<Output, PD2>,
    line4: atmega_hal::port::Pin<Output, PD3>,
    line5: atmega_hal::port::Pin<Output, PD4>,
    line6: atmega_hal::port::Pin<Output, PD5>,

    column1: atmega_hal::port::Pin<Output, PC0>,
    column2: atmega_hal::port::Pin<Output, PC1>,
    column3: atmega_hal::port::Pin<Output, PC2>,
    column4: atmega_hal::port::Pin<Output, PC3>,
    column5: atmega_hal::port::Pin<Output, PC4>,
    column6: atmega_hal::port::Pin<Output, PC5>,
}

impl BoardLEDs {
    pub fn new(pins: atmega_hal::Pins) -> BoardLEDs {
        BoardLEDs {
            line1: pins.pd0.into_output(),
            line2: pins.pd1.into_output(),
            line3: pins.pd2.into_output(),
            line4: pins.pd3.into_output(),
            line5: pins.pd4.into_output(),
            line6: pins.pd5.into_output(),

            column1: pins.pc0.into_output(),
            column2: pins.pc1.into_output(),
            column3: pins.pc2.into_output(),
            column4: pins.pc3.into_output(),
            column5: pins.pc4.into_output(),
            column6: pins.pc5.into_output(),
        }
    }

    // TODO is this board specific ?
    /// light a LED on the matrix
    pub fn light_led(&mut self, led: Option<LED>) {
        // light correct LED in matrix
        let (column, line) = if let Some(l) = led {
            LED_POSITIONS[l as usize]
        } else {
            (255, 255) // different from everything, will switch all LEDs off
        };

        // Lines = Anods ; on = high else low
        if line == 0 {
            self.line1.set_high()
        } else {
            self.line1.set_low()
        };
        if line == 1 {
            self.line2.set_high()
        } else {
            self.line2.set_low()
        };
        if line == 2 {
            self.line3.set_high()
        } else {
            self.line3.set_low()
        };
        if line == 3 {
            self.line4.set_high()
        } else {
            self.line4.set_low()
        };
        if line == 4 {
            self.line5.set_high()
        } else {
            self.line5.set_low()
        };
        if line == 5 {
            self.line6.set_high()
        } else {
            self.line6.set_low()
        };

        // Columns = Cathod ; on = low else high
        if column == 0 {
            self.column1.set_low()
        } else {
            self.column1.set_high()
        };
        if column == 1 {
            self.column2.set_low()
        } else {
            self.column2.set_high()
        };
        if column == 2 {
            self.column3.set_low()
        } else {
            self.column3.set_high()
        };
        if column == 3 {
            self.column4.set_low()
        } else {
            self.column4.set_high()
        };
        if column == 4 {
            self.column5.set_low()
        } else {
            self.column5.set_high()
        };
        if column == 5 {
            self.column6.set_low()
        } else {
            self.column6.set_high()
        };
    }
}

pub fn new_board() -> (BoardLEDs, BoardTimer) {
    let peripherals = atmega_hal::Peripherals::take().unwrap();
    (
        BoardLEDs::new(atmega_hal::pins!(peripherals)),
        BoardTimer::new(peripherals.TC2),
    )
}
