//! Interfacing the on-board L3GD20 (gyroscope)
#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

// defmt global logger (RTT transport) + panic handler
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use f411::{
    hal::{
        pac,
        prelude::*,
        rcc::Config,
        spi::{Mode, Phase, Polarity, Spi},
    },
    L3gd20,
};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.freeze(Config::hsi().sysclk(64.MHz()).pclk1(32.MHz()));

    let gpioa = p.GPIOA.split(&mut rcc);
    let gpioe = p.GPIOE.split(&mut rcc);

    let mut nss = gpioe.pe3.into_push_pull_output();
    nss.set_high();

    let sck = gpioa.pa5;
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7;

    // L3GD20 is accessed over SPI mode 3 (CPOL = 1, CPHA = 1)
    let mode = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnSecondTransition,
    };

    let spi = Spi::new(
        p.SPI1,
        (Some(sck), Some(miso), Some(mosi)),
        mode,
        1.MHz(),
        &mut rcc,
    );

    let mut l3gd20 = L3gd20::new(spi, nss).unwrap();

    // sanity check: the WHO_AM_I register always contains this value
    assert_eq!(l3gd20.who_am_i().unwrap(), 0xD4);

    loop {
        let m = l3gd20.all().unwrap();
        defmt::println!(
            "gyro x={} y={} z={} temp={}",
            m.gyro.x,
            m.gyro.y,
            m.gyro.z,
            m.temp
        );
    }
}
