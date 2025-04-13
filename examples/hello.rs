//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use adafruit_nrf52840_express::Board;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!");
    let _board = Board::new().unwrap();
    loop {}
}
