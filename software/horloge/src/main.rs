#![no_std]
#![no_main]

mod board;

use crate::board::new_board;
use horloge::led_multiplex;
use horloge::MAX_LEDS;

#[no_mangle]
fn main() -> ! {
    let (mut board_leds, mut board_timer) = new_board();

    loop {
        board_timer.update_time();
        let (hour, min5, minute, seconds, tick) = board_timer.time();
        for smooth_tick in 0..32_u8 {
            for mux_tick in 0..MAX_LEDS as u8 {
                let led = led_multiplex(mux_tick, smooth_tick, hour, min5, minute, seconds, tick);
                board_leds.light_led(led);
                board_timer.delay_us(300_u16); // prevent "leaks" to same line LED
            }
        }
    }
}
