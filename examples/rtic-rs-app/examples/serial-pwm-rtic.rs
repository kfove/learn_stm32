#![no_main]
#![no_std]
#![deny(warnings)]
#![deny(unsafe_code)]
#![feature(type_alias_impl_trait)]
use panic_halt as _;

use rtic::app;

#[app(device = stm32f1xx_hal::pac, peripherals = true)]
mod app {
    // use cortex_m_semihosting::hprintln;
    use nb::block;
    use stm32f1xx_hal::{
        device::USART3,
        prelude::*,
        serial::{Config, Rx, Serial, Tx},
        time::ms,
        timer::{Channel, PwmChannel, Tim2NoRemap},
    };
    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        tx: Tx<USART3>,
        rx: Rx<USART3>,
        duty: u16,
        c1: PwmChannel<stm32f1xx_hal::pac::TIM2, 0>,
    }
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut afio = cx.device.AFIO.constrain();
        let mut gpiob = cx.device.GPIOB.split();
        let mut gpioa = cx.device.GPIOA.split();

        let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
        let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);

        let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
        let rx = gpiob.pb11;

        let mut pwm =
            cx.device
                .TIM2
                .pwm_hz::<Tim2NoRemap, _, _>((c1, c2), &mut afio.mapr, 1.kHz(), &clocks);

        pwm.enable(Channel::C1);
        pwm.set_period(ms(20).into_rate());

        // 设置占空比梯度为0.1ms
        let duty = pwm.get_max_duty() / 200;

        let c1 = pwm.split().0;

        let serial = Serial::new(
            cx.device.USART3,
            (tx, rx),
            &mut afio.mapr,
            Config::default().baudrate(9600.bps()),
            &clocks,
        );
        let (mut tx, mut rx) = serial.split();
        // 开始监听发送的中断事件
        tx.listen();
        // 开始监听接受的中断事件和空闲中断事件,空闲中断的作用是在接受完数据后，如果没有新数据到来，那么就进入空闲中断
        rx.listen(); // 一次数据触发一次中断，多数据是频繁中断
        rx.listen_idle(); // 多次数据不会进入空闲中断，当没有数据时，中断，能够让单片机处理接受数据不抽搐（频繁出入中断）

        (Shared {}, Local { tx, rx, duty, c1 })
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = USART3, local = [tx, rx, duty, c1])]
    fn usart3(cx: usart3::Context) {
        if cx.local.rx.is_rx_not_empty() {
            if let Ok(rec) = block!(cx.local.rx.read()) {
                match rec {
                    48_u8 => cx.local.c1.set_duty(*cx.local.duty * 5),
                    49_u8 => cx.local.c1.set_duty(*cx.local.duty * 7),
                    50_u8 => cx.local.c1.set_duty(*cx.local.duty * 9),
                    51_u8 => cx.local.c1.set_duty(*cx.local.duty * 11),
                    52_u8 => cx.local.c1.set_duty(*cx.local.duty * 13),
                    53_u8 => cx.local.c1.set_duty(*cx.local.duty * 15),
                    54_u8 => cx.local.c1.set_duty(*cx.local.duty * 17),
                    55_u8 => cx.local.c1.set_duty(*cx.local.duty * 19),
                    56_u8 => cx.local.c1.set_duty(*cx.local.duty * 21),
                    57_u8 => cx.local.c1.set_duty(*cx.local.duty * 23),
                    _ => block!(cx.local.tx.write(b'X')).unwrap(),
                }
            }
        }
    }
}
