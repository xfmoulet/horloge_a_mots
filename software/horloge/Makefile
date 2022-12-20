BINARY=target/avr-atmega328p/release/horloge.elf
build:
	cargo build --release
flash: build 
	avrdude -patmega328p -carduino -P /dev/ttyUSB0 -b57600 -D -Uflash:w:$BINARY:e
doc:
	cargo doc --release --open