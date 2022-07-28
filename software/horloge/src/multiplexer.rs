pub mod data;
use crate::data::*;

const MAX_LEDS: usize = 32; // max LEDs "at a time" (ie muxed) - faster to use a power of two

// for a given mux tick / hour, return the LED to illuminate
pub fn led_multiplex(mux_tick: usize, hour: u8, min5: u8, minute: u8) -> Option<LED> {
    // moins le -> heure suivante, attention a minuit
    let ehour = if min5 > 6 {
        if hour < 23 {
            hour + 1
        } else {
            0
        }
    } else {
        hour
    };

    // not a match since we dont have exclusive ranges yet in rust matches
    if mux_tick < NB_HOURS_LED {
        HOURS_LED[ehour as usize][mux_tick]
    } else if mux_tick < NB_HOURS_LED + NB_MIN5_LED {
        MIN5_LED[min5 as usize][mux_tick - NB_HOURS_LED]
    } else if mux_tick < NB_HOURS_LED + NB_MIN5_LED + NB_MINUTES_LED {
        MINUTES_LED[minute as usize][mux_tick - NB_HOURS_LED - NB_MIN5_LED]
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_max_leds_should_be_enough() {
        // should probably be a const assert
        assert!(MAX_LEDS >= NB_HOURS_LED + NB_MIN5_LED + NB_MINUTES_LED);
    }

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
