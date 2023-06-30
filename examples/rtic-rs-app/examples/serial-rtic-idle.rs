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
    };
    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        tx: Tx<USART3>,
        rx: Rx<USART3>,
        buf: [u8; 16],
    }
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut afio = cx.device.AFIO.constrain();
        let mut gpiob = cx.device.GPIOB.split();

        let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
        let rx = gpiob.pb11;

        let serial = Serial::new(
            cx.device.USART3,
            (tx, rx),
            &mut afio.mapr,
            Config::default().baudrate(9600.bps()),
            &clocks,
        );

        // 缓冲区
        let buf: [u8; 16] = [0; 16];
        let (mut tx, mut rx) = serial.split();
        // 开始监听发送的中断事件
        tx.listen();
        // 开始监听接受的中断事件和空闲中断事件,空闲中断的作用是在接受完数据后，如果没有新数据到来，那么就进入空闲中断
        rx.listen(); // 一次数据触发一次中断，多数据是频繁中断
                     // rx.listen_idle(); // 多次数据不会进入空闲中断，当没有数据时，中断，能够让单片机处理接受数据不抽搐（频繁出入中断）

        (Shared {}, Local { tx, rx, buf })
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = USART3, local = [tx, rx, index: usize = 0, buf])]
    fn usart3(cx: usart3::Context) {
        // 这里写的复杂，可以使用 let tx = cx.local.tx 简化
        if cx.local.rx.is_rx_not_empty() {
            // 读取数据
            if let Ok(rec) = block!(cx.local.rx.read()) {
                // 读取数据到buf中
                cx.local.buf[*cx.local.index] = rec;
                *cx.local.index += 1;

                // buf满就自动放出
                if *cx.local.index > 16 - 1 {
                    cx.local.buf[..].into_iter().for_each(|w| {
                        block!(cx.local.tx.write(*w)).unwrap();
                    });
                    *cx.local.index = 0;
                }
                // 读取到数据的时候, 打开空闲中断
                cx.local.rx.listen_idle();
            }
            // 没有数据，又是空闲的时候
        } else if cx.local.rx.is_idle() {
            cx.local.rx.unlisten_idle();
            // 这里[0..index]是必须的，迭代器知道长度, 否则迭代器会一直循环迭代
            cx.local.buf[0..*cx.local.index].into_iter().for_each(|w| {
                block!(cx.local.tx.write(*w)).unwrap();
            });
            *cx.local.index = 0;
        }
    }
}
