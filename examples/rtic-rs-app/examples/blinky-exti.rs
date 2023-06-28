#![no_std]
#![no_main]

use panic_halt as _;

// use cortex_m_semihosting::hprintln;

use cortex_m::interrupt::Mutex;

use cortex_m_rt::entry;

use core::cell::RefCell;
use stm32f1xx_hal::{
    gpio::{gpioc::PC13, ExtiPin, Input, Output, PullDown, PA0},
    pac,
    pac::interrupt,
    prelude::*,
};

static LED: Mutex<RefCell<Option<PC13<Output>>>> = Mutex::new(RefCell::new(None));
static EXIT_KEY: Mutex<RefCell<Option<PA0<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));
#[entry]
fn init() -> ! {
    let mut dp = match pac::Peripherals::take() {
        Some(dp) => dp,
        None => panic!("failed to get Device Peripherals!"),
    };
    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();
    let mut afio = dp.AFIO.constrain();

    cortex_m::interrupt::free(|cs| {
        LED.borrow(cs)
            .replace(Some(gpioc.pc13.into_push_pull_output(&mut gpioc.crh)));
        EXIT_KEY
            .borrow(cs)
            .replace(Some(gpioa.pa0.into_pull_down_input(&mut gpioa.crl)));
        if let Some(exit_key) = EXIT_KEY.borrow(cs).borrow_mut().as_mut() {
            exit_key.make_interrupt_source(&mut afio);
            exit_key.trigger_on_edge(&mut dp.EXTI, stm32f1xx_hal::gpio::Edge::Rising);
            exit_key.enable_interrupt(&mut dp.EXTI);
        };
    });

    // 配置中断

    // 打开中断
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI0);
    }

    loop {}
}

#[interrupt]
fn EXTI0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(led) = LED.borrow(cs).borrow_mut().as_mut() {
            led.toggle();
        }
        if let Some(exit_key) = EXIT_KEY.borrow(cs).borrow_mut().as_mut() {
            exit_key.clear_interrupt_pending_bit();
        }
    });
}
