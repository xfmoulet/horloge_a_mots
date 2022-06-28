# Reflexions about Software 

## Clocks

- We have one 32768 Hz clock (ticks)
- for each tick :
    - If we clock the main clock at 8MHz (default) => 244 cycles per clock (not much)
        - on a cortexM0, there is NO division instruction (code will be generated, can be slow) -> try to avoid them so dont divide ticks to get minutes each time, rather having a (tick, minute, hour) structure updated each tick
    - If we have 32 LED ticks (see hereafter) and we output a LED every tick, one screen will be refreshed around 1kHz, seems OK
    - We need for each tick:
       - update the time struct itself
       - choose which LEDs to light given one time
       - decode line/ columns those LEDs
       - multiplex them in time 
       - (maybe later) do smooth transitions between states ? 

  
## LED ticks

- LEDs will be time-multiplexed
- some LED signals need to be on for several ticks to appear as bright as the ones that are split between several LEDs
- by example we will not light "one", "two" , "three" simultaneously. but any combinations of hours will be output with any compunations of minutes.
- computations show that we need 32 leds ticks max.
- computation is time -> LEDs to map -> LED to turn on for this tick (mux) 
- needs to be done quickly (need to optimize the worst case) 
- we can pregenerate some `hour:minute -> (line / column) of LEDs to turn on` tables in flash (in rust, can be done using a `build.rs` script)

