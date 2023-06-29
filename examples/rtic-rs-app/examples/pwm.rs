#![no_std]
#![no_main]
#![deny(unsafe_code)]

use panic_halt as _;

use cortex_m_rt::entry;

use stm32f1xx_hal::{
    pac,
    prelude::*,
    time::ms,
    timer::{Channel, Tim2NoRemap},
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();

    let mut gpioa = dp.GPIOA.split();

    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let c3 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let c4 = gpioa.pa3.into_alternate_push_pull(&mut gpioa.crl);

    let mut pwm =
        dp.TIM2
            .pwm_hz::<Tim2NoRemap, _, _>((c1, c2, c3, c4), &mut afio.mapr, 1.kHz(), &clocks);

    pwm.enable(Channel::C1);
    pwm.enable(Channel::C2);
    pwm.enable(Channel::C3);
    pwm.enable(Channel::C4);

    pwm.set_period(ms(1).into_rate());

    let max_duty = pwm.get_max_duty();
    // 将duty分512份,pwm的分辨率为1/512,duty不能无限分，最大的分数为usize,那分辨率最大1/2^32
    let duty = max_duty / 512;

    pwm.set_duty(Channel::C1, duty * 256);
    pwm.set_duty(Channel::C2, duty * 128);
    pwm.set_duty(Channel::C3, duty * 64);
    loop {
        for i in 0..512 {
            pwm.set_duty(Channel::C4, duty * i);
        }
    }
}
