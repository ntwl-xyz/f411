#![no_std]
#![no_main]

// defmt global logger (RTT transport) + panic handler
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use f411::{
    hal::{i2c::I2c, pac, prelude::*, rcc::Config},
    Lsm303dlhc,
};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.freeze(Config::hsi().sysclk(64.MHz()).pclk1(32.MHz()));

    let gpiob = p.GPIOB.split(&mut rcc);
    let scl = gpiob.pb6;
    let sda = gpiob.pb9;

    let i2c = I2c::new(p.I2C1, (scl, sda), 400.kHz(), &mut rcc);

    let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

    defmt::println!("initialised lsm303dlhc");

    loop {
        let accel = lsm303dlhc.accel().unwrap();
        let mag = lsm303dlhc.mag().unwrap();
        let temp = lsm303dlhc.temp().unwrap();

        defmt::println!("accel={}", defmt::Debug2Format(&accel));
        defmt::println!("mag={}", defmt::Debug2Format(&mag));
        defmt::println!("temp={}", defmt::Debug2Format(&temp));
    }
}
