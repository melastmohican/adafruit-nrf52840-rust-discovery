# adafruit-nrf52840-rust-discovery

This repository contains examples for the [Adafruit Feather nRF52840 Express](https://www.adafruit.com/product/3857) board, written in Rust using the [Embassy](https://embassy.dev/) async framework.

## Hardware

![Adafruit Feather nRF52840 Express Pinout](https://cdn-learn.adafruit.com/assets/assets/000/114/673/large1024/circuitpython_Adafruit_Feather_nRF52840_Pinout.png?1662064111)

## Documentation

- [Primary Guide: Introducing the Adafruit nRF52840 Feather](https://learn.adafruit.com/introducing-the-adafruit-nrf52840-feather)
- [Adafruit Feather nRF52840 Express PrettyPins (PDF)](https://learn.adafruit.com/assets/57454)
- [Adafruit Feather nRF52840 Express PrettyPins (SVG)](https://learn.adafruit.com/assets/57455)

## Flash layout

The `memory.x` in this project correctly places the application at `0x00026000`,
preserving:
- **MBR** at `0x00000000` (4 KB)
- **SoftDevice S140 v6.x** at `0x00001000` (~148 KB)
- **DFU Bootloader** at `0x000F4000` (48 KB)

See `memory.x` for full region map.

## Restoring the bootloader (if erased)

If you accidentally erased the bootloader, flash the combined SD+bootloader hex:

1. Download `feather_nrf52840_express_bootloader-X.X.X_s140_6.1.1.hex` from
   https://github.com/adafruit/Adafruit_nRF52_Bootloader/releases

2. Flash it:
```bash
probe-rs erase --chip nRF52840_xxAA --allow-erase-all
probe-rs download --chip nRF52840_xxAA \
  feather_nrf52840_express_bootloader-X.X.X_s140_6.1.1.hex \
  --format hex
```

## Examples

The examples demonstrate the use of async drivers and core features of the Embassy framework on the nRF52840.

---

### main (Root Binary)

A template showcasing basic initialization of the Embassy executor on the nRF52840 and standard RTT logging output via `defmt`.

```bash
cargo run
```

### blinky

Alternates between the red LED on pin `P1.15` (D13) and the blue LED on pin `P1.10` (CONN) with a 500 ms period using asynchronous timers.

```bash
cargo run --example blinky
```

### rgBlinky

Cycles the onboard addressable RGB NeoPixel LED (P0.16) through rainbow colors using the HSV color space with gamma correction.

```bash
cargo run --example rgBlinky
```
