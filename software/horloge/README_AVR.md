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

# flasher le bootloader arduino avec un arduino

## mettre un arduino en mode ArduinoISP
- ouvrir l'IDE Arduino (v1.8 chez moi)
- ouvrir `Fichier/Exemples/11-ArduinoISP`, les instructions sont au début du fichier
- Connecter un arduino, uploader l' ISP arduino (upload ArduinoISP)
- connecter l'Arduino vers la board horloge (5v, GND, Reset, ...) vers pins 5v, 10,11,12,...

## flasher le bootloader Arduino vers la board
