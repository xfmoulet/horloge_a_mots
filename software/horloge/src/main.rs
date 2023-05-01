#![no_std]
#![no_main]

mod board;

use crate::board::{delay_us, new_board};
use horloge::led_multiplex;
use horloge::MAX_LEDS;

#[cfg(feature = "first_test_animation")]
use board::BoardLEDs;

#[cfg(feature = "first_test_animation")]
fn first_test_animation(board_leds: &BoardLEDs) {
    // all columns
    for column in 0..6u8 {
        for _ in 0..1000 {
            for line in 0..6u8 {
                board_leds.light_led_xy(column, line);
                delay_us(300_u16); // prevent "leaks" to same line LED
            }
        }
    }
    // all lines
    for line in 0..6u8 {
        for _ in 0..1000 {
            for column in 0..6u8 {
                board_leds.light_led_xy(column, line);
                delay_us(300_u16); // prevent "leaks" to same line LED
            }
        }
    }
}

#[no_mangle]
fn main() -> ! {
    let (board_leds, mut board_timer) = new_board();

    #[cfg(feature = "first_test_animation")]
    first_test_animation(&board_leds);

    loop {
        board_timer.update_time();
        let (hour, min5, minute, seconds, tick) = board_timer.time();
        for smooth_tick in 0..32_u8 {
            for mux_tick in 0..MAX_LEDS as u8 {
                let led = led_multiplex(mux_tick, smooth_tick, hour, min5, minute, seconds, tick);
                board_leds.light_led(led);
                delay_us(300_u16); // prevent "leaks" to same line LED
            }
        }
    }
}
