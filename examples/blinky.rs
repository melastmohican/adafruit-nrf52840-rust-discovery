//! # Onboard LED Blinky Example
//!
//! Alternates between the red LED on pin P1_15 and the blue LED on pin P1_10 with a 500 ms period.
//!
//! ## Hardware
//!
//! - **Board:** Adafruit Feather nRF52840 Express
//!
//! ## Run
//!
//! ```bash
//! cargo run --example blinky
//! ```

#![no_std]
#![no_main]

use defmt::info;
// Defmt Logging
use defmt_rtt as _;
// Panic Handler
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;

// Adafruit Feather nRF52840 Express pin mapping:
//   D13 / red LED  → P1_15
//   Blue LED       → P1_10
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    info!("Hello from Embassy!");

    let mut led_red = Output::new(p.P1_15, Level::Low, OutputDrive::Standard);
    let mut led_blue = Output::new(p.P1_10, Level::High, OutputDrive::Standard);
    // Blue LED starts high (on) to match the original behaviour.

    loop {
        led_red.set_high();
        led_blue.set_low();
        Timer::after_millis(500).await;

        led_red.set_low();
        led_blue.set_high();
        Timer::after_millis(500).await;
    }
}
