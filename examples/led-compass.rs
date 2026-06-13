//! Blinks an LED
#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_std]
#![no_main]

// defmt global logger (RTT transport) + panic handler
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use f411::{
    hal::{prelude::*, stm32},
    led::{LedCompass},
};

#[entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();

    // stm32 in this case internally refers to the stm32f411 lib inside the HAL
    let peripherals = stm32::Peripherals::take().unwrap();
    let gpiod = peripherals.GPIOD.split();

    let mut compass = LedCompass::new(gpiod);

    compass.n.on();
    compass.e.on();
    compass.s.on();
    compass.w.on();

    loop {}
}
