#![no_std]
#![no_main]

// defmt global logger (RTT transport) + panic handler
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;

use f411::{
    hal::{prelude::*, stm32, i2c::I2c},
    Lsm303dlhc,
};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze();

    let gpiob = p.GPIOB.split();
    let scl = gpiob.pb6.into_alternate_af4();
    let sda = gpiob.pb9.into_alternate_af4();

    let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks);

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
