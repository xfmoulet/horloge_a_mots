Notes STM32f410 LED blink in rust 

## add stm32 core target

	rustup target add thumbv7em-none-eabihf

## load example project

	git clone https://github.com/stm32-rs/stm32f4xx-hal.git

add line to Cargo.toml

	[features]
	default = ["rt", "stm32f410"]

## cargo flash

> see https://github.com/probe-rs/cargo-flash 

	# add libusb dependency
	apt install -y pkg-config libusb-1.0-0-dev 
	apt purge libusb-dev

	# install cargo flash
	cargo install cargo-flash                  

## build example

	cargo build --example delay-syst-blinky --features stm32f410 --release
    // will not be needed, cargo flash does it
## probe setup

needs permissions to access usb probe: https://probe.rs/docs/getting-started/probe-setup/

	sudo cp 69-probe-rs.rules /etc/udev/rules.d/
	udevadm control --reload
	udevadm trigger

## transfer data
	
	cargo flash --chip stm32f410rbtX --release --example delay-syst-blinky

	Finished release [optimized + debuginfo] target(s) in 0.08s
	    Flashing /home/xavier/Documents/Xav/dev/horloge_a_mots/stm32f4xx-hal/target/thumbv7em-none-eabihf/release/examples/delay-syst-blinky
	     Erasing sectors ✔ [00:00:00] [##################################################################] 16.00KiB/16.00KiB @ 41.75KiB/s (eta 0s )
	 Programming pages   ✔ [00:00:00] [##################################################################]  1.00KiB/ 1.00KiB @  2.33KiB/s (eta 0s )
	    Finished in 0.459s

--> IT BLINKS !

## modification

smooth : 

    for n in (0..32u32).chain((0..32u32).rev()) {
        for _ in 0..777 {                    
            led.set_high();
            delay.delay_us(n);
            led.set_low();
            delay.delay_us(32-n);
        }
    }

## links

- https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/


## Separate project from lib

- create a projet with `cargo init`
- copy led blinker
- add dependencies
```toml
	[dependencies]
	embedded-hal = "0.2"
	nb = "1"
	cortex-m = "0.7"
	cortex-m-rt = "0.7"
	# Panic behaviour, see https://crates.io/keywords/panic-impl for alternatives
	panic-halt = "0.2"

	[dependencies.stm32f4xx-hal]
	version = "0.13.2"
	features = ["stm32f407"] # replace the model of your microcontroller here
```
- also optimize for size, use LTO (divides size by 10!)

```toml
[profile.release]
lto = true
opt-level = "s"
```

- copy `.cargo/config` and `memory.x` from the `stm32f4xx-hal` repository to your project and make sure the sizes match up with the datasheet
- build with 
```
cargo flash --chip stm32f410rbtX --release
```

## Next steps

documentation / makefile
separate create 

use IRQ based blinker (with 32kHz crystal interrupts)

board - logic: hour, setup - display logic
board : LEDs, buttons, 32kHz interrupt
