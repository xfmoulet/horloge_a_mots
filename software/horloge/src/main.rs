#![no_std]
#![no_main]

mod board;

use crate::board::{new_board, BoardLEDs, BoardTimer};
use horloge::led_multiplex;
use horloge::MAX_LEDS;


fn demo(board_leds: &BoardLEDs, board_timer: &BoardTimer) {
    // all columns
    for column in 0..6u8 {
        for _ in 0..1000 {
            for line in 0..6u8 {
                board_leds.light_led_xy(column,line);
                board_timer.delay_us(300_u16); // prevent "leaks" to same line LED
            }
        }
    }
    // all lines
    for line in 0..6u8 {
        for _ in 0..1000 {
            for column in 0..6u8 {
                board_leds.light_led_xy(column,line);
                board_timer.delay_us(300_u16); // prevent "leaks" to same line LED
            }
        }
    }
}

#[no_mangle]
fn main() -> ! {
    let (board_leds, mut board_timer) = new_board(13,37);
    
    demo(&board_leds, &board_timer);

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
