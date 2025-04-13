#![no_std]
#![no_main]

use adafruit_nrf52840_express::Board;
use adafruit_nrf52840_express::hal::gpio;
use adafruit_nrf52840_express::prelude::{OutputPin, _embedded_hal_blocking_delay_DelayMs};
use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!");
    let board = Board::new().unwrap();

    let mut delay = board.delay;

    let mut led1 = board.d13.into_push_pull_output(gpio::Level::Low);
    let mut led2 = board.blue_led;

    led2.set_high().unwrap();

    loop {
        led1.set_high().unwrap();
        led2.set_low().unwrap();

        delay.delay_ms(500u16);

        led1.set_low().unwrap();
        led2.set_high().unwrap();

        delay.delay_ms(500u16);
    }
}