mod data;
use crate::data::*;

const TICKS_SEC: u32 = 32768;
const MAX_LEDS: usize = 32; // max LEDs "at a time" (ie muxed) - faster to use a power of two


#[derive(Debug)]
struct Horloge {
    ticks: u32,  // 1/32KHz seconds. 0..TICKS_SEC * 60 
    minutes: u8, // TICKS_SEC*60 ticks: 0..4
    min5: u8,    // 5 minutes: 0..11
    hours: u8,   // 0..23
}

impl Horloge {
    const fn from_hm(hours: u8, min: u8) -> Horloge {
        Horloge {
            ticks: 0,
            minutes: min % 5,
            min5: min / 5,
            hours,
        }
    }

    // advance time ; hard RT @ TICKS_SEC Hz
    fn tick(&mut self, button: bool) {
        self.ticks += 1;
        if self.ticks == TICKS_SEC * (if button { 1 } else { 60 }) {
            self.ticks = 0;
            self.minutes += 1;
            if self.minutes == 5 {
                self.min5 += 1;
                self.minutes = 0;
                if self.min5 == 12 {
                    self.hours += 1; // change a la demie pour "moins le" ?
                    self.min5 = 0;
                }
            }
        }
    }

    // for a given tick, return the LED to illuminate - soft RT
    fn led(&self) -> Option<LED> {
        // moins le -> heure suivante, attention a minuit
        let hours = if self.min5 > 6 {
            if self.hours < 23 {
                self.hours + 1
            } else {
                0
            }
        } else {
            self.hours
        };

        let mux_tick = (self.ticks as usize) % MAX_LEDS;

        // not a match since we dont have exclusive ranges
        if mux_tick < NB_HOURS_LED {
            HOURS_LED[hours as usize][mux_tick]
        } else if mux_tick < NB_HOURS_LED+NB_MIN5_LED {
            MIN5_LED[self.min5 as usize][mux_tick - NB_HOURS_LED]
        } else if mux_tick < NB_HOURS_LED+NB_MIN5_LED+NB_MINUTES_LED {
            MINUTES_LED[self.minutes as usize][mux_tick - NB_HOURS_LED - NB_MIN5_LED]
        } else {
            None
        }
    }

    // debug code: show 2*MAX_LEDS ticks of leds
    fn show_leds(&mut self) {
        print!("{:?}:", self);
        for _ in 0..MAX_LEDS*2 {
            print!("{:?} ", self.led());
            self.tick(false);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn create_tick_zero() {
        let h = Horloge::from_hm(11,11);
        assert_eq!(h.ticks,0);
    }

    #[test]
    fn create_minute_ok() {
        let h = Horloge::from_hm(12,34);
        assert_eq!(h.minutes,4);
        assert_eq!(h.min5,6);
    }    

    #[test]
    fn create_hour() {
        let h = Horloge::from_hm(12,34);
        assert_eq!(h.hours,12);
    }

    #[test]
    fn test_tick() {
        let test_vec = [(2,0,0),(500,0,0),(32768,0,0),(32768*60-1,0,0),(32768*60,1,0),(32768*60*5,0,1)];
        for (ti,min,min5) in test_vec {
            let mut h = Horloge::from_hm(12,33);
            for _ in 0..ti {
                h.tick(false);
            }
            assert_eq!(h.minutes,3+min);
            assert_eq!(h.min5,6+min5);
        }
    }

    #[test]    
    fn test_tick_fast() {
        // when settings is set, time goes 60x faster
        let test_vec = [(2,0,0),(500,0,0),(32768-1,0,0),(32768,1,0),(32768*5,0,1),(32768*60,0,0)];
        for (ti,min,min5) in test_vec {
            let mut h = Horloge::from_hm(0,0);
            for _ in 0..ti {
                h.tick(true);
            }
            println!("{:?}",ti);
            assert_eq!(h.minutes,min);
            assert_eq!(h.min5,min5);
        }
    }

    #[test]
    fn test_max_leds_should_be_enough() {
        // should probably be a const assert
        assert!(MAX_LEDS >= NB_HOURS_LED + NB_MIN5_LED + NB_MINUTES_LED);
    }

    #[test]
    fn test_led_midnight() {
        let mut h = Horloge::from_hm(0,0);
        let leds = [Some(LED::Minuit),Some(LED::Minuit),Some(LED::Minuit),None,None,None];
        for l in leds.iter() {
            assert_eq!(h.led(), *l);
            h.tick(false);
        }
    }

    #[test]
    fn test_led_12_57() {
        let mut h = Horloge::from_hm(12,57);
        let leds = [
            Some(LED::Une),Some(LED::Heure),Some(LED::Heure),Some(LED::Heure),None,None,None,
            Some(LED::Moins),Some(LED::Moins),Some(LED::Moins),Some(LED::CinqMin),Some(LED::CinqMin),None
            ];
        for l in leds.iter() {
            assert_eq!(h.led(), *l);
            h.tick(false);
        }
    }
}

fn main() {
    Horloge::from_hm(0, 5).show_leds();
    Horloge::from_hm(23, 37).show_leds();
    Horloge::from_hm(11, 59).show_leds();
    Horloge::from_hm(23, 33).show_leds();
    Horloge::from_hm(12, 57).show_leds();
}
