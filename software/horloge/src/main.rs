#![no_std]
#![no_main]

mod board;

use crate::board::Board;
use horloge::led_multiplex;
use horloge::MAX_LEDS;

#[no_mangle]
fn main() -> ! {
    let mut board = Board::new();
    loop {
        board.update_time();
        let (hour, min5, minute) = board.time();
        for mux_tick in 0..MAX_LEDS {
            let led = led_multiplex(mux_tick, hour, min5, minute);
            board.light_led(led);
            board.delay_us(10_u16);
        }
    }
}
