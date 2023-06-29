#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{
    gpio::{Output, PushPull, PC13},
    pac::{self, interrupt, TIM1},
    prelude::*,
    timer::{CounterMs, Event, TimerExt},
};

static LED: Mutex<RefCell<Option<PC13<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
static TIMER: Mutex<RefCell<Option<CounterMs<TIM1>>>> = Mutex::new(RefCell::new(None));
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split();
    let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut timer = dp.TIM1.counter_ms(&clocks);
    timer.start(1.secs()).unwrap();
    timer.listen(Event::Update);
    cortex_m::interrupt::free(|cs| {
        LED.borrow(cs).replace(Some(led));
        TIMER.borrow(cs).replace(Some(timer));
    });
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::TIM1_UP);
    }
    loop {}
}

#[interrupt]
fn TIM1_UP() {
    cortex_m::interrupt::free(|cs| {
        if let Some(led) = LED.borrow(cs).borrow_mut().as_mut() {
            led.toggle();
        }
        if let Some(timer) = TIMER.borrow(cs).borrow_mut().as_mut() {
            timer.clear_interrupt(Event::Update);
        }
    })
}
