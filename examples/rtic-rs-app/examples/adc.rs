#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f1xx_hal::{adc, pac, prelude::*};
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.adcclk(2.MHz()).freeze(&mut flash.acr);
    hprintln!("adc freq: {}", clocks.adcclk());

    // 两adc都是可以用的，看来Clocks实现了Copy
    let mut adc1 = adc::Adc::adc1(dp.ADC1, clocks);
    let mut adc2 = adc::Adc::adc2(dp.ADC2, clocks);

    let mut gpiob = dp.GPIOB.split();

    let mut ch0 = gpiob.pb0.into_analog(&mut gpiob.crl);
    let mut ch1 = gpiob.pb1.into_analog(&mut gpiob.crl);

    loop {
        let data: u16 = adc1.read(&mut ch0).unwrap();
        hprintln!("adc1: {}", data);
        let data: u16 = adc2.read(&mut ch1).unwrap();
        hprintln!("adc2: {}", data);
    }
}
