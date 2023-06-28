#![no_main]
#![no_std]
#![deny(warnings)]
#![deny(unsafe_code)]
#![feature(type_alias_impl_trait)]
use panic_halt as _;

use rtic::app;

#[app(device = stm32f1xx_hal::pac, peripherals = true)]
mod app {
    use cortex_m_semihosting::hprintln;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}
    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        hprintln!("Hello, World!");
        foo::spawn().unwrap();
        (Shared {}, Local {})
    }

    #[task]
    async fn foo(_: foo::Context) {
        hprintln!("Hello, foo!");
    }
}
