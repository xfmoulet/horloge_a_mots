BINARY=target/avr-atmega8/release/horloge.elf
.PHONY: build
build:
	cargo build --release
flash: build 
	avrdude -patmega8    -cstk500v1 -P /dev/ttyUSB0 -b19200 -e -D -Uflash:w:$(BINARY):e -U lfuse:w:0xe4:m -U hfuse:w:0xd9:m 
doc:
	cargo doc --release --open
