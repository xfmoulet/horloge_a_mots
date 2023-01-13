BINARY=target/avr-atmega8/release/horloge.elf
build:
	cargo build --release
flash: build 
#	avrdude -patmega8    -carduino -P /dev/ttyUSB0 -b19200 -e -D -Uflash:w:$(BINARY):e
	avrdude -patmega8 -cstk500v1 -P /dev/ttyACM0 -b19200 -e -D -Uflash:w:$(BINARY):e
doc:
	cargo doc --release --open