use panic_halt as _;
use avr_device::atmega8::{Peripherals, TC2, PORTB, PORTC, PORTD};

use horloge::data::*;
use horloge::SECONDS_PER_MINUTE;

// the timer is 8 bit, prescaler 1024 so max rollover is 255 / 32Hz = 8s :(
// 60s = 2*2*3*5 , let's use 1s ? could be 1,2,3,4,5,6 (10 and 12 are >8s)

const PRESCALER: u32 = 1024; // also check clock setup
const ROLLOVER_SECONDS: u32 = 1;
const ROLLOVER_TICKS: u32 = 32768 / PRESCALER * ROLLOVER_SECONDS;

// some checks
const _: () = assert!(ROLLOVER_TICKS < 255); // we compute but check at build time
const _: () = assert!(60 % ROLLOVER_SECONDS == 0);


// use a https://docs.rust-embedded.org/book/peripherals/singletons.html ?
pub struct BoardTimer {
    tick: u8, // past value of the timer, updated at 32Hz if presecaled by 1024

    second: u8,
    hour: u8,
    min5: u8,
    minute: u8,

    timer: TC2,
    extra_port: PORTB, // GPIO port to read time setting pin
}

impl BoardTimer {
    pub fn new(timer: TC2, extra_port: PORTB, hour: u8, minute: u8) -> BoardTimer {

        // Set up async timer tc2
        // https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-timer.rs

        // Datasheet p114: control register2
        timer.tccr2.write(|w| w
        	// waveform generation : 10 - CTC: Clear Timer on Compare match
        	.wgm20().clear_bit()
        	.wgm21().set_bit()
            .cs2().val_0x07() // prescale 1024 - Datasheet p. 116
        ); 

        // Output compare register
        timer.ocr2.write(|w| unsafe { w.bits(ROLLOVER_TICKS as u8 - 1) });

        // Asynchronous mode, from 32KHz crystal
        timer.assr.write(|w| w.as2().set_bit());

        // System clock: keep default internal RC@8MHz

        // set GPIO 4 (PB1, see doc/dtd2022/images/schema_avr.png for schema) to input, pull-up
        extra_port.ddrb.write(|w| w.pb1().clear_bit());
        extra_port.portb.write(|w| w.pb1().set_bit() );

        // for interrupt based ticks, see https://blog.rahix.de/005-avr-hal-millis/
        BoardTimer {
            tick: 0, // 32Hz

            second: 0,
            minute: minute%5,
            min5: minute/5,
            hour,

            timer,
            extra_port,
        }
    }

    /// this function returns the current time in RTC as H:M5:M:S:T (NOT H:M:S !) where
    ///     H is the hour 0-23
    ///     M5 is the 5-minutes number 0-11
    ///     M is the remaining minutes 0-4
    ///     S is the seconds
    ///     T 1/32s ticks 
    pub fn time(&self) -> (u8, u8, u8, u8, u8) {
        (self.hour, self.min5, self.minute, self.second, self.tick)
    }

    /// delay a bit - blocking
	// absolutely inaccurate but eh
	#[inline(always)]
    pub fn delay_us(&self, us: u16) {
	    for _ in 0..us  {
	        core::hint::black_box({}) // empty loop
	    }
	}    

    // are we setting time ?
    fn time_set(&self) -> bool {
        self.extra_port.pinb.read().pb1().bit_is_clear()
    }

    // to be called faster than ROLLOVER_SECONDS. main thread, not in an interrupt
    pub fn update_time(&mut self) {
        let counter_value = self.timer.tcnt2.read().bits();

        if self.tick > counter_value {
            if self.time_set() {
                self.min5 += 1;
                self.minute=0;
                self.tick=0;
            } else {
                self.second += ROLLOVER_SECONDS as u8;
            }
        }

        self.tick = counter_value;
        // TODO make a time + normalize(time) ? 
        if self.second >= SECONDS_PER_MINUTE {
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
    lines: PORTD,
    columns: PORTC,
}

impl BoardLEDs {
    pub fn new(line_port : PORTD, column_port:PORTC) -> BoardLEDs {
        let b = BoardLEDs {
            lines: line_port,
            columns: column_port,
        };

        // configure as output
        b.lines.ddrd.write(|w| 
        	w.pd0().set_bit()
        	.pd1().set_bit()
        	.pd2().set_bit()
        	.pd3().set_bit()
        	.pd4().set_bit()
        	.pd5().set_bit()
        );
        // configure as output
        b.columns.ddrc.write(|w| 
        	w.pc0().set_bit()
        	.pc1().set_bit()
        	.pc2().set_bit()
        	.pc3().set_bit()
        	.pc4().set_bit()
        	.pc5().set_bit()
        );
        b
    }

    /// light a LED on the matrix
    pub fn light_led(&self, led: Option<LED>) {
        // light correct LED in matrix
        let (column, line) = if let Some(l) = led {
            LED_POSITIONS[l as usize]
        } else {
            (255, 255) // different from everything, will switch all LEDs off
        };
        self.light_led_xy(column,line);
    }

    pub fn light_led_xy(&self, column: u8, line: u8) {
        // Lines = Anods ; on = high else low
        self.lines.portd.write(
            |w| w
            .pd0().bit(line != 0)
            .pd1().bit(line != 1) 
            .pd2().bit(line != 2) 
            .pd3().bit(line != 3) 
            .pd4().bit(line != 4) 
            .pd5().bit(line != 5) 
        );

        // Columns = Cathod ; on = low else high
        self.columns.portc.write(
            |w| w
            .pc0().bit(column == 0)
            .pc1().bit(column == 1) 
            .pc2().bit(column == 2) 
            .pc3().bit(column == 3) 
            .pc4().bit(column == 4) 
            .pc5().bit(column == 5) 
        );
    }
}

pub fn new_board(hour:u8, minute:u8) -> (BoardLEDs, BoardTimer) {
    let peripherals = Peripherals::take().unwrap();

    (
        BoardLEDs::new(peripherals.PORTD, peripherals.PORTC),
        BoardTimer::new(peripherals.TC2, peripherals.PORTB, hour, minute),
    )
}
