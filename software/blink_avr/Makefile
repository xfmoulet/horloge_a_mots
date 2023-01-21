build:
	cargo build --release
flash_xfm: build
	avrdude -patmega8 -cstk500v1 -P /dev/ttyUSB0 -b19200 -e -D  -Uflash:w:target/avr-atmega8/release/blink_avr.elf:e

flash_xfm_cpp:
	avrdude -patmega8 -cstk500v1 -P /dev/ttyUSB0 -b19200 -e -D  -Uflash:w:cpp/blink.ino_atmega8_8000000L.hex:i
