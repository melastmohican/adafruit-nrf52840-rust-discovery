# adafruit-nrf52840-rust-discovery

This repository contains examples for the [Adafruit Feather nRF52840 Express](https://www.adafruit.com/product/3857) board, written in Rust using the [Embassy](https://embassy.dev/) async framework.

## Hardware

![Adafruit Feather nRF52840 Express Pinout](https://cdn-learn.adafruit.com/assets/assets/000/114/673/large1024/circuitpython_Adafruit_Feather_nRF52840_Pinout.png?1662064111)

## Documentation

- [Primary Guide: Introducing the Adafruit nRF52840 Feather](https://learn.adafruit.com/introducing-the-adafruit-nrf52840-feather)
- [Adafruit Feather nRF52840 Express PrettyPins (PDF)](https://learn.adafruit.com/assets/57454)
- [Adafruit Feather nRF52840 Express PrettyPins (SVG)](https://learn.adafruit.com/assets/57455)

## Building

Everything builds natively with plain `cargo`: no container is needed.

```bash
rustup target add thumbv7em-none-eabihf
```

The plain examples (`blinky`, `rgBlinky`) need nothing else. The Matter examples also compile C code in their dependencies' build
scripts (OpenThread, mbedTLS, the Nordic radio libraries), which needs `cmake` and `clang` with `libclang`:

- **macOS:** `brew install cmake llvm`, then make Homebrew's LLVM visible to `cargo`, which is needed by `bindgen` and by one dependency that
  calls plain `clang`. Either of these is enough (both tested in a clean environment):
  - put it on `PATH`, in `~/.zshrc`: `export PATH="$(brew --prefix llvm)/bin:$PATH"`
  - or set `LIBCLANG_PATH`, in `~/.zshrc`: `export LIBCLANG_PATH="$(brew --prefix llvm)/lib"`

  If neither is set, the build fails in `nrf-mpsl-sys`, `nrf-sdc-sys` and `nrf-802154-sys` with `error: unknown type name 'uint32_t'` and
  `Unable to generate bindings`: `bindgen` found a `libclang` that cannot find the C headers for the bare-metal target (most likely
  Apple's). A `LIBCLANG_PATH` left over from another toolchain fails the same way; check with `echo $LIBCLANG_PATH`.
- **Linux (Debian/Ubuntu):** `sudo apt-get install clang libclang-dev cmake` (what CI installs). Set `LIBCLANG_PATH` if `bindgen`
  cannot find `libclang`.
- No ARM GCC is needed and `ninja` is optional. To flash you need `probe-rs`.

Tested with Rust 1.93.1 and 1.99.1, `cmake` 3.31 and 4.4, `clang` 19 and 23 (Linux and macOS arm64). CI builds on Ubuntu.

```bash
cargo run --release --example blinky                # the plain examples, in the repo root

cd matter-examples                                  # the Matter examples are a separate crate
cargo run --release --example matter_thread_coex_light
```

`cargo run` builds, flashes and runs through `probe-rs`: the runner is set in `.cargo/config.toml`, which `matter-examples/` inherits.
Use `cargo build` instead to only build, and `cargo run ... 2>&1 | tee run.log` to keep the RTT log.

## Flash layout

The default `memory.x` places the application at `0x00027000`, behind the MBR and the S140 v7.3.0 SoftDevice
(see "Getting latest S140 SoftDevice" below), with the DFU bootloader at `0x000F4000`:
- **MBR** at `0x00000000` (4 KB)
- **SoftDevice S140 v7.3.0** at `0x00001000` (~150 KB)
- **Application** from `0x00027000`
- **DFU Bootloader** at `0x000F4000` (48 KB), optional

The Matter examples (`matter-examples/memory.x`) end the application at `0x000ED000`, where the Adafruit layout keeps its user data, so
the linker refuses an app that would run into the user-data area or the bootloader. Nothing in this repo links over the MBR, the
SoftDevice or the bootloader, and you should not flash an image that does (for example one linked at `0x0`, as some upstream examples
are): if your board lacks the MBR, the SoftDevice or the bootloader, restore them with the steps below before running the examples.

## Restoring the bootloader (if erased)

If you accidentally erased the bootloader, flash the combined SD+bootloader hex:

1. Download `feather_nrf52840_express_bootloader-0.11.0_s140_6.1.1.hex` from
   https://github.com/adafruit/Adafruit_nRF52_Bootloader/releases

2. Flash it:
```bash
probe-rs erase --chip nRF52840_xxAA --allow-erase-all

probe-rs download --verify --binary-format hex feather_nrf52840_express_bootloader-0.11.0_s140_6.1.1.hex
      Erasing ✔ 100% [####################] 196.00 KiB @  26.58 KiB/s (took 7s)
  Programming ✔ 100% [####################] 196.00 KiB @  12.56 KiB/s (took 16s)
    Verifying ✔ 100% [####################] 196.00 KiB @  16.63 KiB/s (took 12s) 
```

## Getting latest S140 SoftDevice

https://www.nordicsemi.com/Products/Development-software/S140/Download

```bash
probe-rs download --verify --binary-format hex --chip nRF52840_xxAA s140_nrf52_7.3.0_softdevice.hex
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

### matter_thread_coex_light (Matter over Thread, upstream stack)

An On/Off light (red LED, P1.15) that is commissioned over BLE and runs on Thread, on the current upstream
`rs-matter-embassy` coex stack (dependencies pinned to exact upstream git revisions, no local patches). It lives in
[`matter-examples/`](matter-examples/), a separate crate like `softdevice-tests/`, because the Matter stack needs a different
dependency set from the plain examples (an allocator, and its own critical-section implementation):

```bash
cd matter-examples
cargo run --release --example matter_thread_coex_light    # builds, flashes and runs with probe-rs
```

The header comment of each Matter example lists the steps before `cargo run` (board layout, Home Assistant and Thread Border Router,
removing the old node) and what to do after it starts (pairing, "Add anyway", checking the light). The Matter discriminator is random on
every boot: read the QR code / pairing code from the RTT log. The
non-concurrent variant, `matter_thread_light`, is built the same way (`--example matter_thread_light`). It is
**experimental**: it builds, but commissioning it from a phone app does not work properly because of known issues in
`rs-matter`, `rs-matter-stack` and the software 802.15.4 radio driver (details in the header comment of
[`matter-examples/examples/matter_thread_light.rs`](matter-examples/examples/matter_thread_light.rs)). Use the coex example.

### softdevice_check

Demonstrates querying the SoftDevice version via raw API.

```bash
cargo run --example softdevice_check
```

### softdevice_mem_check

Reads and prints SoftDevice information block.

```bash
cargo run --example softdevice_mem_check
```
