#![no_std]
#![no_main]


mod board;

use horloge::MAX_LEDS;
use crate::board::Board;
use horloge::led_multiplex;

#[cortex_m_rt::entry]
fn main() -> ! {    
    let mut b  = Board::new();
    loop {
        let (hour, min5, minute) = b.time();
        for mux_tick in 0..MAX_LEDS {
            let led = led_multiplex(mux_tick, hour, min5, minute);
            b.light_led(led);
        }
    }
}
