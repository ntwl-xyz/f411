//! Lights the user LED pointing closest to magnetic north
#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_std]
#![no_main]

// defmt global logger (RTT transport) + panic handler
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use f411::{
    hal::{i2c::I2c, pac, prelude::*, rcc::Config},
    led::LedCompass,
    Lsm303dlhc,
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.freeze(Config::hsi().sysclk(64.MHz()).pclk1(32.MHz()));

    // user LEDs on GPIOD
    let gpiod = p.GPIOD.split(&mut rcc);
    let mut compass = LedCompass::new(gpiod);

    // on-board LSM303DLHC magnetometer on I2C1 (PB6 = SCL, PB9 = SDA)
    let gpiob = p.GPIOB.split(&mut rcc);
    let scl = gpiob.pb6;
    let sda = gpiob.pb9;
    let i2c = I2c::new(p.I2C1, (scl, sda), 400.kHz(), &mut rcc);
    let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

    let mut delay = cp.SYST.delay(&rcc.clocks);

    loop {
        let mag = lsm303dlhc.mag().unwrap();

        // Horizontal magnetic-field vector in the board frame. With only four
        // LEDs, the LED closest to north is simply whichever axis dominates and
        // its sign — the same result atan2 would round to, without the float
        // math. Widen to i32 first so `.abs()` can't overflow on i16::MIN.
        //

        let x = mag.x as i32;
        let y = mag.y as i32;

        compass.n.off();
        compass.e.off();
        compass.s.off();
        compass.w.off();

        // Axis -> LED mapping: +X points North and +Y points West on this board.
        if x.abs() >= y.abs() {
            if x >= 0 {
                compass.n.on();
            } else {
                compass.s.on();
            }
        } else if y >= 0 {
            compass.w.on();
        } else {
            compass.e.on();
        }

        defmt::println!("mag x={} y={}", x, y);

        delay.delay_ms(100u32);
    }
}
