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
    hal::{pac, prelude::*},
    led::Leds,
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let gpiod = dp.GPIOD.split(&mut rcc);

    let mut delay = cp.SYST.delay(&rcc.clocks);

    let mut leds = Leds::new(gpiod);
    let mut i = 0;

    loop {
        for j in 0..leds.len() {
            leds[j].off();
        }

        leds[i].on();
        delay.delay_ms(100u32);
        i = (i + 1) % 4;
    }
}
