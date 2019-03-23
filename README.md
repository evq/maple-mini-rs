# maple-mini-rs

A small rust wrapper crate around [stm32f1xx-hal](https://github.com/stm32-rs/stm32f1xx-hal).

Provides numbered pin aliases corresponding to the pins on the maple mini
board.

## Flashing blink example

This repo includes a small blink example using the RTFM framework.

It also contains linker scripts and build targets compatible with both the original maple mini
bootloader as well as the [stm32duino bootloader](https://github.com/rogerclarkmelbourne/STM32duino-bootloader).

### STM32duino bootloader
```
make flash-stm32duino
```

### Original bootloader
```
make flash-orig
```
