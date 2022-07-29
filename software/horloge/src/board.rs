use horloge::data::*;

use stm32f4xx_hal as hal;

use hal::{pac, prelude::*};
use panic_halt as _; // panic handler

// See https://stackoverflow.com/questions/71653128/use-of-generics-for-embedded-hal-structs if we want to be generic
pub struct Board {
    delay: hal::timer::SysDelay,
    rtc: hal::rtc::Rtc,

    line1: hal::gpio::Pin<'A', 0, hal::gpio::Output>,
    line2: hal::gpio::Pin<'A', 1, hal::gpio::Output>,
    line3: hal::gpio::Pin<'A', 2, hal::gpio::Output>,
    line4: hal::gpio::Pin<'A', 3, hal::gpio::Output>,
    line5: hal::gpio::Pin<'A', 4, hal::gpio::Output>,
    line6: hal::gpio::Pin<'B', 7, hal::gpio::Output>,

    column1: hal::gpio::Pin<'B', 3, hal::gpio::Output>,
    column2: hal::gpio::Pin<'A', 11, hal::gpio::Output>,
    column3: hal::gpio::Pin<'B', 0, hal::gpio::Output>,
    column4: hal::gpio::Pin<'A', 7, hal::gpio::Output>,
    column5: hal::gpio::Pin<'A', 6, hal::gpio::Output>,
    column6: hal::gpio::Pin<'B', 5, hal::gpio::Output>,
}

impl Board {
    pub fn new() -> Board {
        let mut dp = pac::Peripherals::take().unwrap();
        let cp = cortex_m::peripheral::Peripherals::take().unwrap();

        // Set up the LEDs
        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        Board {
            line1: gpioa.pa0.into_push_pull_output(),
            line2: gpioa.pa1.into_push_pull_output(),
            line3: gpioa.pa2.into_push_pull_output(),
            line4: gpioa.pa3.into_push_pull_output(),
            line5: gpioa.pa4.into_push_pull_output(),
            line6: gpiob.pb7.into_push_pull_output(),

            column1: gpiob.pb3.into_push_pull_output(),
            column2: gpioa.pa11.into_push_pull_output(),
            column3: gpiob.pb0.into_push_pull_output(),
            column4: gpioa.pa7.into_push_pull_output(),
            column5: gpioa.pa6.into_push_pull_output(),
            column6: gpiob.pb5.into_push_pull_output(),

            delay: cp.SYST.delay(&clocks),
            rtc: hal::rtc::Rtc::<hal::rtc::Lse>::new(dp.RTC, &mut dp.PWR), // lots of ':'
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

        if line == 0 { self.line1.set_high() } else { self.line1.set_low() };
        if line == 1 { self.line2.set_high() } else { self.line2.set_low() };
        if line == 2 { self.line3.set_high() } else { self.line3.set_low() };
        if line == 3 { self.line4.set_high() } else { self.line4.set_low() };
        if line == 4 { self.line5.set_high() } else { self.line5.set_low() };
        if line == 5 { self.line6.set_high() } else { self.line6.set_low() };

        if column == 0 { self.column1.set_high() } else {self.column1.set_low() };
        if column == 1 { self.column2.set_high() } else {self.column2.set_low() };
        if column == 2 { self.column3.set_high() } else {self.column3.set_low() };
        if column == 3 { self.column4.set_high() } else {self.column4.set_low() };
        if column == 4 { self.column5.set_high() } else {self.column5.set_low() };
        if column == 5 { self.column6.set_high() } else {self.column6.set_low() };
    }

    /// this function returns the current time in RTC as H:M5:M (NOT H:M:S !) where
    ///     H is the hour 0-23
    ///     M5 is the 5-minutes number 0-11
    ///     M is the minutes 0-4
    pub fn time(&self) -> (u8, u8, u8) {
        // directly read register since we dont need date, sync
        let tr = self.rtc.regs.tr.read();

        let hour = tr.ht().bits() * 10 + tr.hu().bits();
        let min5 = tr.mnt().bits() * 2 + (if tr.mnu().bits() >= 5 { 1 } else { 0 });
        let units = tr.mnu().bits();
        let minute = if units >= 5 { units - 5 } else { units }; // min % 5

        (hour, min5, minute)
    }

    // delay a bit
    pub fn delay_us(&mut self, nb: u32) {
        self.delay.delay_us(nb);
    }
}
