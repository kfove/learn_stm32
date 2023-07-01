#![no_std]
#![no_main]

use cortex_m::singleton;
use cortex_m_rt::entry;
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
    // cortex-m::singleton会创建一个静态对象，并返回Option<&'static mut T>
    // 所以这东西不应该在loop中使用，这导致多个变量拥有可变所有权,在rtic中应该可以
    let buf = singleton!(: [u8; 8] = [0; 8]).unwrap();

    // .read() 和 .write() 都会丢弃获取所有权返回.如果在循环中,当第一次循环结束后,buf和rx或tx已经drop了，第二次是没有的
    // 同一作用域下可以，请不要把他两放在循环中
    // 这里问题是read和write接受的是mut T的参数,不是引用，而且是可变所有权,Rc行不通,Rc<RefCell>具有内部可变性，但是返回是Ref或RefMut
    // 虽然实现了deref,但是还是不行啊
    // 最终问题是，我怎么做资源分配，这两方法会把所有权转移，确实难搞，技术现在还不够
    let (buf, rx) = rx.read(buf).wait();
    let (buf, _rx) = rx.read(buf).wait();

    let (_buf, _tx) = tx.write(buf).wait();
    loop {}
}
