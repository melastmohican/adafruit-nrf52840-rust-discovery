//! Example that reads and prints SoftDevice information block.
//!
//! It dumps raw memory from the SoftDevice info region, parses fields such as
//! size, firmware ID, and device ID, and prints them using `defmt`. The example
//! runs on an Adafruit Feather nRF52840 board with the S140 SoftDevice.
//!
//! ```bash
//! cargo run --example softdevice_mem_check
//! ```
//! Example output (from running on Adafruit Feather nRF52840):
//! [INFO]   [0x3000] = 0xffffff2c
//! [INFO]   [0x3004] = 0x51b1e5db
//! [INFO]   [0x3008] = 0x00026000
//! [INFO]   [0x300c] = 0xffff00b6
//! [INFO]   [0x3010] = 0x0000008c
//! [INFO]   [0x3014] = 0x005b9169
//! [INFO]   [0x3018] = 0xedfe5f93
//! [INFO]   [0x301c] = 0x733c84a0
//! [INFO]   [0x3020] = 0x146274f8
//! [INFO]   [0x3024] = 0xcbc0065e
//! [INFO]   [0x3028] = 0x6013f272
//! [INFO]   [0x302c] = 0x4df8b530
//! [INFO]   [0x3030] = 0x2c064604
//! [INFO]   [0x3034] = 0xd20e78a9
//! [INFO]   [0x3038] = 0xf004e8df
//! [INFO]   [0x303c] = 0x0e0e0e03
//! [INFO] SoftDevice ID (unreliable, see DRGN-8363): 0x008c
//! [INFO] SoftDevice FWID: 0x00b6 (S140 v6.1.1 ✅)
//! [INFO] SoftDevice size: 155648 bytes (152 KB)
//!
//! Example out for s140_nrf52_7.3.0_softdevice.hex
//! [INFO ]   [0x3000] = 0xffffff2c
//! [INFO ]   [0x3004] = 0x51b1e5db
//! [INFO ]   [0x3008] = 0x00027000
//! [INFO ]   [0x300c] = 0xffff0123
//! [INFO ]   [0x3010] = 0x0000008c
//! [INFO ]   [0x3014] = 0x006adb78
//! [INFO ]   [0x3018] = 0xc69a2e7a
//! [INFO ]   [0x301c] = 0xfa6cb67d
//! [INFO ]   [0x3020] = 0xcc2157f3
//! [INFO ]   [0x3024] = 0xe5d510c3
//! [INFO ]   [0x3028] = 0x3cfb7114
//! [INFO ]   [0x302c] = 0x4dfcb530
//! [INFO ]   [0x3030] = 0x2c064604
//! [INFO ]   [0x3034] = 0xd20e78a9
//! [INFO ]   [0x3038] = 0xf004e8df
//! [INFO ]   [0x303c] = 0x0e0e0e03
//! [INFO ] SoftDevice ID (unreliable, see DRGN-8363): 0x008c
//! [INFO ] SoftDevice FWID: 0x0123 (S140 v7.3.0 ✅)
//! [INFO ] SoftDevice size: 159744 bytes (156 KB)

#![no_std]
#![no_main]

use defmt::info;
// Defmt Logging
use defmt_rtt as _;
// Panic Handler
use panic_probe as _;

use embassy_executor::Spawner;

/// Nordic SoftDevice info block starts at 0x3000 on all nRF52 series devices.
/// Layout (each field is 4 bytes):
///   +0x00  info block size / magic
///   +0x04  CRC/hash of the SoftDevice binary
///   +0x08  SoftDevice size in bytes
///   +0x0C  upper u16 = reserved (0xFFFF), lower u16 = FWID  ← use this to identify SD
///   +0x10  upper u16 = reserved (0x0000), lower u16 = SD ID ← unreliable, S140 reports as S132 (Nordic bug DRGN-8363)
///   +0x14  hash/signature bytes (not human-readable)
const SD_INFO_BASE: u32 = 0x0000_3000;

extern crate nrf_softdevice;
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_nrf::init(Default::default());
    // Raw dump omitted to avoid RTT buffer overflow; uncomment if needed.
    for offset in (0x00..0x40_u32).step_by(4) {
        let val = unsafe { core::ptr::read_volatile((SD_INFO_BASE + offset) as *const u32) };
        info!("  [0x{:04x}] = 0x{:08x}", SD_INFO_BASE + offset, val);
    }

    // --- Parsed fields ---
    let sd_size = unsafe { core::ptr::read_volatile((SD_INFO_BASE + 0x08) as *const u32) };
    let fwid = unsafe { core::ptr::read_volatile((SD_INFO_BASE + 0x0C) as *const u16) };
    let sd_id = unsafe { core::ptr::read_volatile((SD_INFO_BASE + 0x10) as *const u16) };

    // SD ID at 0x3010 is unreliable — S140 early versions report 0x008C (S132) due to
    // Nordic bug DRGN-8363. Use FWID at 0x300C as the authoritative identifier instead.
    info!("SoftDevice ID (unreliable, see DRGN-8363): 0x{:04x}", sd_id);

    info!(
        "SoftDevice FWID: 0x{:04x} ({})",
        fwid,
        match fwid {
            0x00B6 => "S140 v6.1.1 ✅",
            0x00A9 => "S140 v6.0.0",
            0x00AF => "S140 v6.1.0",
            0x0100 => "S140 v7.0.1",
            0x0101 => "S140 v7.2.0",
            0x0123 => "S140 v7.3.0 ✅",
            0x00A8 => "S132 v6.0.0",
            0x00A5 => "S132 v5.1.0",
            0x00A0 => "S132 v5.0.1",
            0x009D => "S132 v5.0.0",
            0x0098 => "S132 v4.0.2",
            0x008C => "S132 v3.0.0",
            0x0088 => "S132 v2.0.1",
            0x0081 => "S132 v2.0.0",
            _ => "Unknown",
        }
    );
    info!("SoftDevice size: {} bytes ({} KB)", sd_size, sd_size / 1024);
    // Ensure all pending RTT packets are sent before we idle
    defmt::flush();

    // Ensure all RTT packets are flushed
    defmt::flush();
    loop {}
}
