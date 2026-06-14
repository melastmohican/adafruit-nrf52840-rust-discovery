//! "Hello, world!" over RTT — Embassy port of examples/hello.rs

#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;

extern crate nrf_softdevice;
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_nrf::init(Default::default());

    info!("Hello, world!");

    loop {
        // Nothing to do — message already printed.
        // In a real application you would spawn tasks here.
        core::hint::spin_loop();
    }
}
