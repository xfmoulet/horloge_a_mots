# Reflexions about Software 

## Clocks

- We have one 32768 Hz clock (ticks)
- for each tick :
    - If we clock the main clock at 8MHz (default) 
        - 8MHz/32kHz = 244 cycles per clock (not much): computations needs to be done quickly 
        - need to optimize the worst case
    - compute time from ticks
        - on a cortexM0, there is NO division instruction (code will be generated, can be slow) 
        - try to avoid divisions so dont divide ticks to get minutes each time, 
        - rather have a (tick, minute, hour) structure updated each tick
    - If we have 32 LED ticks (see hereafter) and we output a LED every tick, one screen will be refreshed around 1kHz, seems OK
    - We need for each tick:
       - update the time struct itself
       - choose which LED to light now
       - (maybe later) do smooth transitions between states ? 
  
## LED ticks

- LEDs will be time-multiplexed
- some LED signals need to be on for several ticks to appear as bright as the ones that are split between several LEDs
- by example we will not light "one", "two" , "three" simultaneously. but any combinations of hours will be output with any compunations of minutes.
- computations show that we need 32 leds ticks max.
- computation is: 
    - time(hours, min) -> LEDs to output
    - LEDs to output + current tick -> LED to turn on for this tick (mux) 
    - LED to turn on -> line/column to turn on
- To be fast, we can pregenerate some `hour:minute -> array (line / column) of LEDs to turn on` tables in flash (in rust, can be done using a `build.rs` script) and then choose which to turn on with ticks%32 (fast)
    - since those (hours, minutes) tables are independent we can generate two smaller tables in flash/ROM

