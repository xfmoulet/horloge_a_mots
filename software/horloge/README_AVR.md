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

	avrdude -patmega328p -carduino -P /dev/ttyUSB0 -b57600 -D -Uflash:w:target/avr-atmega328p/release/blink_avr.elf:e


(see also https://github.com/Rahix/avr-hal/tree/main/ravedude ?)

### Bootloader

to flash a bootloader using an arduino, check this page: https://docs.arduino.cc/built-in-examples/arduino-isp/ArduinoISP
en gros: on flashe ArduinoISP sur l'arduino, on utilise ce dernier comme programmateur, et on envoie avec arduinoISP le bon bootloader ou directement le programme sans bootloader (testé).

Pour le bootloader lui-même on peut utiliser minicore (en cours de test): https://github.com/MCUdude/MiniCore
Choisir le core Atmega8, programmeur Arduino as Programmer

> ATTENTION: bien choisir **internal** clock 4/8 MHz, PAS external clock au risque de "briquer" le système (je parle d'expérience)
