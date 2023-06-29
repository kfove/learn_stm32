#![no_main]
#![no_std]
#![deny(warnings)]
#![deny(unsafe_code)]
#![feature(type_alias_impl_trait)]
use panic_halt as _;

use rtic::app;

#[app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [EXTI0])]
mod app {
    use core::fmt::Write;// 为Ssd1306实现write
    use cortex_m_semihosting::hprintln;
    use ssd1306::{mode::*, prelude::*, size::DisplaySize128x64, I2CDisplayInterface, Ssd1306};
    use stm32f1xx_hal::{
        gpio::{Alternate, OpenDrain, Pin},
        i2c::{BlockingI2c, Mode},
        pac::I2C1,
        prelude::*,
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        display: Ssd1306<
            I2CInterface<
                BlockingI2c<
                    I2C1,
                    (
                        Pin<'B', 8, Alternate<OpenDrain>>,
                        Pin<'B', 9, Alternate<OpenDrain>>,
                    ),
                >,
            >,
            DisplaySize128x64,
            TerminalMode,
        >,
    }
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        hprintln!("start init");
        let mut gpiob = cx.device.GPIOB.split();
        let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
        let sck = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

        hprintln!("io init");
        let mut afio = cx.device.AFIO.constrain();
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        hprintln!("i2c init");
        let i2c = BlockingI2c::i2c1(
            cx.device.I2C1,
            (scl, sck),
            &mut afio.mapr,
            Mode::Fast {
                frequency: 400_000.Hz(),
                duty_cycle: stm32f1xx_hal::i2c::DutyCycle::Ratio2to1,
            },
            clocks,
            1000,
            10,
            1000,
            1000,
        );

        hprintln!("interface init");
        let interface = I2CDisplayInterface::new(i2c);

        let mut display = ssd1306::Ssd1306::new(
            interface,
            DisplaySize128x64,
            ssd1306::rotation::DisplayRotation::Rotate0,
        )
        .into_terminal_mode();
        match display.init() {
            Ok(_) => hprintln!("display init"),
            Err(_) => {hprintln!("error to init display");panic!("error")}
        }
        display.clear().unwrap();
        hprintln!("Hello, World!");
        (Shared {}, Local { display })
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        hprintln!("Hello, idle!");
        foo::spawn().unwrap();
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(local = [display], priority = 1)]
    async fn foo(cx: foo::Context) {
        cx.local.display.write_str("Hello, World!").unwrap();
        hprintln!("Hello, foo!");
    }
}
