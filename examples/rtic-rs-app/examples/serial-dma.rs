#![no_std]
#![no_main]

use cortex_m::singleton;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{
    device::can1::tx,
    pac,
    prelude::*,
    serial::{Config, Serial},
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();
    let channels = dp.DMA1.split();

    let mut gpiob = dp.GPIOB.split();

    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    let serial = Serial::new(
        dp.USART3,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(9600.bps()),
        &clocks,
    );

    let tx = serial.tx.with_dma(channels.2);
    let rx = serial.rx.with_dma(channels.3);
    let buf = singleton!(: [u8; 8] = [0; 8]).unwrap();

    // 这里做演示, rx,tx不能放入循环里面,read和write都会移走所有权，这很麻烦，具体后面会完善
    let (buf, _rx) = rx.read(&mut *buf).wait();
    let (_, _) = tx.write(buf.get(0).unwrap()).wait();
    loop {}
}
