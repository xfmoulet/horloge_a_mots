# README for AVR / Arduino with Rust 

See: 
- https://book.avr-rust.com
- https://github.com/avr-rust/blink
- https://dev.to/creativcoder/how-to-run-rust-on-arduino-uno-40c0

## Setup

### installing

```bash
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
sudo apt-get install binutils gcc-avr avr-libc avrdude # ubuntu, see https://book.avr-rust.com/002.1-installing-required-third-party-tools.html
cd myproject
rustup override set nightly
```

### flashing 

	avrdude -q -C/etc/avrdude.conf -patmega328p -carduino -P/dev/ttyACM0 -D "-Uflash:w:$1:e"
(see also https://github.com/Rahix/avr-hal/tree/main/ravedude ?)

