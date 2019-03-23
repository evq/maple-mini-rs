flash-stm32duino:
	cp memory.stm32duino.x memory.x
	cargo build --examples
	arm-none-eabi-objcopy target/thumbv7m-none-eabi/debug/examples/blink target/thumbv7m-none-eabi/debug/examples/blink.bin -O binary
	dfu-util -D target/thumbv7m-none-eabi/debug/examples/blink.bin -a 2

flash-orig:
	cp memory.orig.x memory.x
	cargo build --examples
	arm-none-eabi-objcopy target/thumbv7m-none-eabi/debug/examples/blink target/thumbv7m-none-eabi/debug/examples/blink.bin -O binary
	dfu-util -D target/thumbv7m-none-eabi/debug/examples/blink.bin -a 1
