#![no_std]
#![no_main]
//! Example demonstrating how to query the SoftDevice version via raw API.
//!
//! This example initializes the SoftDevice with appropriate interrupt priorities,
//! queries the BLE version, prints it via defmt, and then loops forever.
//!
//! Run on an Adafruit Feather nRF52840 board with the S140 SoftDevice.
// ```bash
// cargo run --example softdevice_check
// ```
//! Example output (from running on Adafruit Feather nRF52840):
//!
//! [INFO ] SoftDevice version: BLE=9 company=0x0059 sub=182
//!
//! Example output for S140 v7.3.0
//! [INFO ] softdevice RAM: 13112 bytes
//! [INFO ] SoftDevice version: BLE=10 company=0x0059 sub=291
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
extern crate nrf_softdevice;
use nrf_softdevice::{raw, Softdevice};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // SoftDevice requires P0/P1 reserved for itself
    // embassy peripherals must use P2 or lower
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = embassy_nrf::interrupt::Priority::P2;
    config.time_interrupt_priority = embassy_nrf::interrupt::Priority::P2;
    let _p = embassy_nrf::init(config);

    let config = nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_RC as u8,
            rc_ctiv: 16,
            rc_temp_ctiv: 2,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_500_PPM as u8,
        }),
        ..Default::default()
    };

    let _sd = Softdevice::enable(&config);

    // Read version via raw SVC call
    let mut version = raw::ble_version_t {
        version_number: 0,
        company_id: 0,
        subversion_number: 0,
    };

    let ret = unsafe { raw::sd_ble_version_get(&mut version) };
    if ret == 0 {
        info!(
            "SoftDevice version: BLE={} company=0x{:04x} sub={}",
            version.version_number, version.company_id, version.subversion_number
        );
    } else {
        info!("sd_ble_version_get failed: {}", ret);
    }

    loop {
        embassy_time::Timer::after_secs(5).await;
    }
}
