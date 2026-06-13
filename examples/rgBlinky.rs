//! # WS2812 RGB LED Example for Adafruit Feather nRF52840 Express
//!
//! Cycles through rainbow colors on the built-in NeoPixel LED (P0.16)
//! using SPI with the `smart-leds` and `ws2812-spi` crates.
//!
//! ## Hardware
//!
//! - **Board:** Adafruit Feather nRF52840 Express
//!
//! ## Run
//!
//! ```bash
//! cargo run --example rgBlinky
//! ```

#![no_std]
#![no_main]

use defmt::info;
// Defmt Logging
use defmt_rtt as _;
// Panic Handler
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_nrf::{bind_interrupts, peripherals, spim};
use embassy_nrf::spim::Spim;
use embassy_time::Timer;
use smart_leds::{
    RGB8, SmartLedsWrite, brightness, gamma,
    hsv::{Hsv, hsv2rgb},
};
use ws2812_spi::Ws2812;

// Bind the interrupt for SPI3
bind_interrupts!(struct Irqs {
    SPIM3 => spim::InterruptHandler<peripherals::SPI3>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    info!("Initializing WS2812 NeoPixel on P0.16...");

    // Configure SPI config. WS2812 expects SPI frequency around 2.4MHz - 3.8MHz.
    // Since nRF52840 SPIM supports discrete 2MHz or 4MHz, we set it to 2MHz (M2)
    // as it is in the range for ws2812-spi default 3-bit encoding.
    let mut config = spim::Config::default();
    config.frequency = spim::Frequency::M2;
    config.mode = spim::MODE_0;

    // We only need MOSI (P0.16) for driving the NeoPixel.
    // We use new_txonly_nosck to initialize it with just the MOSI pin.
    let spi = Spim::new_txonly_nosck(
        p.SPI3,
        Irqs,
        p.P0_16, // MOSI
        config,
    );

    // Create the Ws2812 SPI wrapper
    let mut led = Ws2812::new(spi);

    // Initial HSV color (hue, saturation, value/brightness)
    let mut color = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };

    // Brightness level (0-255, setting to 10 to avoid too bright output)
    let brightness_level = 10;

    info!("Starting rainbow animation...");

    loop {
        // Iterate through all hues to create rainbow effect
        for hue in 0..=255 {
            color.hue = hue;

            // Convert from HSV to RGB color space
            let data: RGB8 = hsv2rgb(color);

            // Apply gamma correction and brightness limiting
            let leds = [data];
            led.write(brightness(gamma(leds.iter().cloned()), brightness_level))
                .unwrap();

            Timer::after_millis(20).await;
        }
    }
}
