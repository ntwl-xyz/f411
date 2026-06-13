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
    led::{Leds},
    hal::{delay::Delay, prelude::*, stm32},
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();

    // stm32 in this case internally refers to the stm32f411 lib inside the HAL
    let peripherals = stm32::Peripherals::take().unwrap();

    let rcc = peripherals.RCC.constrain();
    let gpiod = peripherals.GPIOD.split();

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze();
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut leds = Leds::new(gpiod);
    let mut i = 0;

    loop {
        for j in 0..leds.len() {
            leds[j].off();
        }

        leds[i].on();
        delay.delay_ms(100_u16);
        i = (i + 1) % 4;
    }
}
