# Notes STM32f410 LED blink in rust 

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

## smooth pulse

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

## TODO

isolate program from lib
documentation / makefile


board - logic: hour, setup - display logic
board : LEDs, buttons, rtc (?)
