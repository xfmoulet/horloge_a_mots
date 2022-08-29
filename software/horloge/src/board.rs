use horloge::data::*;

use panic_halt as _; // panic handler

use atmega_hal::clock;
use atmega_hal::delay::Delay;
use atmega_hal::port::mode::Output;
use atmega_hal::port::*;

use embedded_hal::blocking::delay::DelayUs;

const TICKS_PER_MINUTE: u32 = 32768 * 60;
type ClockSpeed = clock::MHz16;

pub struct Board {
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

    tick: u32,
    hour: u8,
    min5: u8,
    minute: u8,

    delay: Delay<ClockSpeed>,
}

impl Board {
    pub fn new() -> Board {
        let peripherals = atmega_hal::Peripherals::take().unwrap();

        // Set up the LEDs
        let pins = atmega_hal::pins!(peripherals);

        // Set up timer / interrupts

        // Set up the system clock.
        /*
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
        */
        /* also set up the timer on ext clock + interrupt */

        Board {
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

            tick: 0,
            minute: 0,
            min5: 0,
            hour: 0,

            delay: Delay::new(),
        }
    }

    // light a LED
    pub fn light_led(&mut self, led: Option<LED>) {
        // light correct LED in matrix
        let (line, column) = if let Some(l) = led {
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

    #[allow(non_snake_case)]
    pub fn tick32kHz(&mut self) {
        self.tick += 1;
        while self.tick >= TICKS_PER_MINUTE {
            self.tick -= TICKS_PER_MINUTE;
            self.minute += 1;
        }
        while self.minute >= 5 {
            self.minute -= 5;
            self.min5 += 1;
        }
        while self.min5 >= 12 {
            self.min5 -= 12;
            self.hour += 1;
        }
        while self.hour >= 24 {
            self.hour -= 24;
        }
    }
}
