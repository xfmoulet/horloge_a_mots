# Word Clock Software 

## Firmware usage

- A demo lights up all columns then all lines
- Clock starts at 13:37
- Connect GPIO pin 4 to GND to set time (1s -> +5 minutes)

## General 

- Main principle is a compute loop : 

    - read the time as hour, 5 minute step (00,05,10,...), remaining minutes 0-4
    - current time + current multiplex tick -> LED to output this tick (or none)
    - LED to turn on -> line/column to turn on
    - Turn it on on the hardware 
    - Wait a small delay 

## Clocks

- See clock tree in reference manual, page 124
- We have one 32768 Hz crystal. We cannot use this crystal for main clock : if we want to use it as main clock the whole microcontroller (MCU) will be driven at 32kHz and NOT a multiple of it using the PLL. This is very slow.
- As a consequence we will use the internal (not precise, fast) 8MHz clock for running the program and display the LED but not update the clock
- Update of the clock will be done using a timer driven by the Crystal (precise), see `board.rs` file, method `update_time`


## LED display

- LEDs will be time-multiplexed
    - some LED signals need to be "on" for several ticks to appear as bright as the ones that are split between several LEDs
    - computations show that we need 32 leds ticks max. By example we will not light "one", "two" , "three" simultaneously. but any combinations of hours will be output with any computations of minutes.
    - If we have 32 LED ticks (see hereafter) and we output a LED every tick, we need to be fast to not flicker.
    - In order to light a LED, we set the line (anod) as high, columns (cathods) to LOW. Reverse voltage of 3.3v is within spec.

- To be fast, we pre-generate some `hour:minute -> array (line / column) of LEDs to turn on` tables in flash
    - In rust, this is done using a `build.rs` script
    - since those (hours, 5 minute, minutes) tables are independent we can generate several smaller tables in flash/ROM

> mini board : to test, a 4x2 board with 8 LEDs can be generated. Use the mini_board feature in Cargo.toml defaults

## Files

- `build.rs`     : will generate at build time the `data.rs` file
- `src/lib.rs`   : multiplexing and decoding logic for the LEDs : will use the data tables in data.rs
- `src/data.rs`  : this is a generated file. It will contain the static correspondence tables for led multiplexing.
- `src/board.rs` : handling all hardware-related functions. It is currently not a trait bout could be for testing (useful?), implements driving LEDs, reading time and waiting 
- `src/main.rs`      : main file with high-level logic
