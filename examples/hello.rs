#![no_main]
#![no_std]

// defmt global logger (RTT transport) + panic handler
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
// pull in the device crate so its interrupt vector table gets linked
use f411::hal as _;

#[entry]
fn main() -> ! {
    defmt::println!("hello world");
    loop {}
}
