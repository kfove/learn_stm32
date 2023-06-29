#![no_main]
#![no_std]
#![deny(warnings)]
#![deny(unsafe_code)]

use panic_halt as _;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true)]
mod app {
    use cortex_m_semihosting::hprintln;
    use stm32f1xx_hal::prelude::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init()]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut gpioc = cx.device.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        led.set_low();
        (Shared {}, Local {})
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        hprintln!("Hello, World!");
        loop {
            cortex_m::asm::nop();
        }
    }
}
