#![no_std]
#![no_main]

/// 入口
use cortex_m_rt::entry;

/// 忽略panic
use panic_halt as _;

/// 引入stm32f1xx_hal,pac是外设接口Peripherals, prelude中有许多常用的trait,
/// 比如将usize转化成Rate的.Hz()方法
use stm32f1xx_hal::{pac, prelude::*};
/// 引入semihosting,方便调试。意味半主机模式
use cortex_m_semihosting::hprintln;
/// 入口函数,名字随意, 返回值是 !，在rust中这是一个never类型,panic!和loop返回值就是never
#[entry]
fn main() -> ! {
    /// 获取外设接口
    let dp = pac::Peripherals::take().unwrap();
    /// 获取GPIOC, PC13脚是我们需要的, ,split()方法在prelude中, 具体的trait位置为
    /// stm32f1xx_hal::gpio::ExtiPin
    let mut gpioc = dp.GPIOC.split();
    /// 获取PC13,并设置为推挽输出, 由于led和gpioc状态需要改变，所以声明为可变
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    /// 设置低电平
    led.set_low();
    /// 这段代码会在openocd打印
    hprintln!("Hello, World!");
    loop {}
}
