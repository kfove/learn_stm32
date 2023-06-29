#![no_std]
#![no_main]

// 入口
use cortex_m_rt::entry;

// 忽略panic
use panic_halt as _;

// 引入stm32f1xx_hal,pac是外设接口Peripherals, prelude中有许多常用的trait,
// 比如将usize转化成Rate的.Hz()方法
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};
// 引入semihosting,方便调试。意味半主机模式
use cortex_m_semihosting::hprintln;
// 入口函数,名字随意, 返回值是 !，在rust中这是一个never类型,panic!和loop返回值就是never
#[entry]
fn main() -> ! {
    // 获取外设接口
    let dp = pac::Peripherals::take().unwrap();
    // 这是cortex-m的基本设备接口
    let cp = cortex_m::Peripherals::take().unwrap();
    // 获取GPIOC, PC13脚是我们需要的, ,split()方法在prelude中, 具体的trait位置为
    // stm32f1xx_hal::gpio::ExtiPin
    // 获取flash和rcc, rcc为时钟控制寄存器的抽象
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // 配置时钟,由于flash的读写依赖系统的时钟,时钟的频率，所以这两是一起配置初始化
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split();
    // 获取PC13,并设置为推挽输出, 由于led和gpioc状态需要改变，所以声明为可变
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // 设置低电平
    led.set_low();
    // 这段代码会在openocd打印
    hprintln!("Hello, World!");
    // 获取定时器，配置为计数模式
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    // 计数1Hz,
    // 意味这一秒内，信号只跳变一次，及只计数1次。,如果是20.Hz(),速度会更快。因为1s内计数两次
      timer.start(1.Hz()).unwrap();
    loop {
        // block！会在timer触发后停止，他会阻塞线程，直到计数器计数完一次
        nb::block!(timer.wait()).unwrap();
        led.toggle();
        nb::block!(timer.wait()).unwrap();
        led.toggle();
    }
}
