#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use maple_mini::{
    gpio::{Output, PushPull},
    pins::*,
    prelude::*,
    timer,
};
use rtfm::app;

#[app(device = maple_mini::device)]
const APP: () = {
    static mut LED: LedPin<Output<PushPull>> = ();

    #[init]
    fn init() {
        let mut rcc = device.RCC.constrain();

        let mut flash = device.FLASH.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut gpiob = device.GPIOB.split(&mut rcc.apb2);

        timer::Timer::syst(core.SYST, 1.hz(), clocks).listen(timer::Event::Update);

        LED = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
    }

    #[exception(resources = [LED], binds=SysTick)]
    fn toggle_led() {
        if resources.LED.is_set_high() {
            resources.LED.set_low();
        } else {
            resources.LED.set_high();
        }
    }
};
