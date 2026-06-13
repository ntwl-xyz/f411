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
    hal::{prelude::*, stm32, spi::Spi},
    l3gd20,L3gd20,
};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze();

    let gpioa = p.GPIOA.split();
    let gpioe = p.GPIOE.split();

    let mut nss = gpioe.pe3.into_push_pull_output();
    nss.set_high().unwrap();

    let sck = gpioa.pa5.into_alternate_af5();
    let miso = gpioa.pa6.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();

    let spi = Spi::spi1(
        p.SPI1,
        (sck, miso, mosi),
        l3gd20::MODE,
        1.mhz().into(),
        clocks,
    );

    let mut l3gd20 = L3gd20::new(spi, nss.into()).unwrap();

    // // sanity check: the WHO_AM_I register always contains this value
    assert_eq!(l3gd20.who_am_i().unwrap(), 0xD4);

    loop {
        let m = l3gd20.all().unwrap();
        defmt::println!("m={}", defmt::Debug2Format(&m));
    }
}
