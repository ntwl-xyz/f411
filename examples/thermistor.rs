//! Reads an NTC thermistor on PC1 (ADC1_IN11) and prints temperature over defmt.
//!
//! Wiring — voltage divider powered from the board's 3V rail (NOT 5V; the ADC
//! reference is VDDA ~= 3.0 V and 5 V would exceed the pin's input range):
//!
//!     3V ── NTC ──┬── PC1 (P1 pin 7, ADC1_IN11)
//!                 ├── R_FIXED ── GND
//!                 └── 100nF ──── GND   (optional noise filter, in parallel)
//!
//! With the NTC on top and the fixed resistor on the bottom, the reading is
//! ratiometric (independent of the exact supply voltage):
//!
//!     R_ntc = R_FIXED * (ADC_MAX - raw) / raw
//!
//! then the Beta equation converts resistance to temperature.

#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_std]
#![no_main]

// defmt global logger (RTT transport) + panic handler
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use f411::hal::{
    adc::{
        config::{AdcConfig, SampleTime},
        Adc,
    },
    pac,
    prelude::*,
    rcc::Config,
};

// ---- NTC parameters: edit to match your thermistor ----
const R_FIXED: f32 = 10_000.0; // fixed divider resistor (ohms)
const R0: f32 = 10_000.0; // NTC nominal resistance at T0 (ohms)
const BETA: f32 = 3950.0; // NTC beta coefficient (kelvin)
const T0: f32 = 298.15; // reference temperature for R0 (kelvin, = 25 C)
// 12-bit ADC full scale (AdcConfig::default() is Resolution::Twelve)
const ADC_MAX: f32 = 4095.0;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    // 64 MHz sysclk -> PCLK2 64 MHz -> ADC clock 32 MHz (Pclk2_div_2), within
    // the F411's 36 MHz ADC limit.
    let mut rcc = p.RCC.freeze(Config::hsi().sysclk(64.MHz()));

    let gpioc = p.GPIOC.split(&mut rcc);
    let therm = gpioc.pc1.into_analog();

    let mut adc = Adc::new(p.ADC1, true, AdcConfig::default(), &mut rcc);

    let mut delay = cp.SYST.delay(&rcc.clocks);

    defmt::println!("thermistor: reading PC1 (ADC1_IN11)");

    loop {
        let raw = adc.convert(&therm, SampleTime::Cycles_480);

        // raw == 0 means the node is at GND: an open/disconnected NTC, or wiring
        // fault. Guard the division and report it instead of computing garbage.
        if raw == 0 {
            defmt::warn!("raw=0 — thermistor disconnected or miswired?");
            delay.delay_ms(1000u32);
            continue;
        }

        let r_ntc = R_FIXED * (ADC_MAX - raw as f32) / raw as f32;

        // Beta equation: 1/T = 1/T0 + (1/BETA) * ln(R_ntc / R0)
        let inv_t = 1.0 / T0 + (1.0 / BETA) * libm::logf(r_ntc / R0);
        let temp_c = 1.0 / inv_t - 273.15;

        defmt::println!("raw={} R={} ohm  T={} C", raw, r_ntc, temp_c);

        delay.delay_ms(1000u32);
    }
}
