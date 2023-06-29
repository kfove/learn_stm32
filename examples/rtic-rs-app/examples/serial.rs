#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nb::block;
use panic_halt as _;
use stm32f1xx_hal::{
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

    let mut gpiob = dp.GPIOB.split();

    // USART1
    // let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    // let rx = gpioa.pa10;

    // USART1
    // let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    // let rx = gpiob.pb7;

    // USART2
    // let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // let rx = gpioa.pa3;

    // USART3
    // pa10为tx口,所以设置推挽输出，由于异步使用内部去控制电平，所以alternate
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    // 默认就是浮空输入
    let rx = gpiob.pb11;

    // 比特率9600
    let mut serial = Serial::new(
        dp.USART3,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(9600.bps()),
        &clocks,
    );

    block!(serial.tx.write(b'X')).unwrap();

    let rec = block!(serial.rx.read()).unwrap();
    assert_eq!(rec, b'X');

    // 我们可以将tx和rx单独分出来,这方便我们在不同任务中使用
    let (mut tx, mut rx) = serial.split();
    block!(tx.write(b'Y')).unwrap();
    let rec = block!(rx.read()).unwrap();
    assert_eq!(rec, b'Y');

    loop {}
}
