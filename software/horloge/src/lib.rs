#![no_std]
/*
This lib is responsible for handling the logic of multiplexing the LEDs.
It uses the data generated in data.rs
*/

pub mod data;
pub const SECONDS_PER_MINUTE: u8 = 60; // set to 1 to go 60x faster

use crate::data::*;

pub const MAX_LEDS: usize = NB_HOURS_LED + NB_MIN5_LED + NB_MINUTES_LED; // max LEDs "at a time" (ie muxed)

// for a given mux tick / hour, return the LED to illuminate
pub fn led_multiplex(mux_tick: u8, smooth_tick: u8, mut hour: u8, mut min5: u8, mut minute: u8, second: u8, tick: u8) -> Option<LED> {    
    // smooth fade to next minute during last second
    // During last second of a minute, we consider the time to be next minute
    // Alternatively, according to the pattern tick.
    if second >= SECONDS_PER_MINUTE-1 && (INTERLEAVE_PATTERNS[tick as usize] & 1<<smooth_tick) != 0 {
        minute += 1;
        if minute>=5 {
            minute = 0;
            min5 += 1;
        }
        if min5 >= 12 {
            min5 = 0;
            hour += 1;
        }
        if hour >= 24 {
            hour = 0;
        }
    }

    // moins le -> heure effective = heure suivante, attention a minuit
    if min5 > 6 {
        hour = if hour < 23 { hour + 1 } else { 0 };
    }

    // Special case: midi/minuit et demi (no E)
    // not a match since we dont have exclusive ranges and ranges cannot be 0..X-1 in rust matches
    if mux_tick < NB_HOURS_LED as u8 {
        HOURS_LED[hour as usize][mux_tick as usize]
    } else if mux_tick < (NB_HOURS_LED + NB_MIN5_LED) as u8 {
        MIN5_LED[min5 as usize][mux_tick as usize - NB_HOURS_LED]
    } else if mux_tick < (NB_HOURS_LED + NB_MIN5_LED + NB_MINUTES_LED) as u8 {
        MINUTES_LED[minute as usize][mux_tick as usize - NB_HOURS_LED - NB_MIN5_LED]
    } else {
        None
    }
}

// TODO move to rustdoc tests since they dont support specific target ?
// if we want to test those, disable  .cargo/config settings
#[cfg(all(test, feature = "big_panel"))]
mod tests {
    use crate::*;

    #[test]
    fn test_led_midnight() {
        let leds = [
            Some(LED::Minuit),
            Some(LED::Minuit),
            Some(LED::Minuit),
            None,
            None,
            None,
        ];
        for (i, l) in leds.iter().enumerate() {
            assert_eq!(led_multiplex(i, 0, 0, 0), *l);
        }
    }

    #[test]
    fn test_led_12_57() {
        let leds = [
            Some(LED::Une),
            Some(LED::Heure),
            Some(LED::Heure),
            Some(LED::Heure),
            None,
            None,
            None,
            Some(LED::Moins),
            Some(LED::Moins),
            Some(LED::Moins),
            Some(LED::CinqMin),
            Some(LED::CinqMin),
            None,
        ];

        let min5 = 57 / 5;
        let minute = 57 % 5;

        for (i, l) in leds.iter().enumerate() {
            assert_eq!(led_multiplex(i, 12, min5, minute), *l);
        }
    }
}
