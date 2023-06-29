#![no_main]
#![no_std]
#![deny(warnings)]
#![deny(unsafe_code)]

use panic_halt as _;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true)]
mod app {
    use cortex_m_semihosting::hprintln;
    use stm32f1xx_hal::{
        gpio::{ExtiPin, Input, Output, PullDown, PushPull, PA0, PC13},
        prelude::*,
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PC13<Output<PushPull>>,
        button: PA0<Input<PullDown>>,
    }

    #[init()]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        let mut afio = cx.device.AFIO.constrain();
        let mut gpioc = cx.device.GPIOC.split();
        let mut gpioa = cx.device.GPIOA.split();
        let mut button = gpioa.pa0.into_pull_down_input(&mut gpioa.crl);
        let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        {
            button.make_interrupt_source(&mut afio);
            button.trigger_on_edge(&mut cx.device.EXTI, stm32f1xx_hal::gpio::Edge::Rising);
            button.enable_interrupt(&mut cx.device.EXTI);
        }
        (Shared {}, Local { led, button })
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        hprintln!("Hello, World!");
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = EXTI0, local = [led, button])]
    fn button_click(cx: button_click::Context) {
        cx.local.led.toggle();
        cx.local.button.clear_interrupt_pending_bit();
    }
}
