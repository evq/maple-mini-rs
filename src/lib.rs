#![no_std]

pub use stm32f1xx_hal::*;

pub mod pins {
    // Right
    pub type D0<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB11<MODE>;
    pub type D1<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB10<MODE>;
    pub type D2<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB2<MODE>;
    pub type D3<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB0<MODE>;
    pub type D4<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA7<MODE>;
    pub type D5<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA6<MODE>;
    pub type D6<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA5<MODE>;
    pub type D7<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA4<MODE>;
    pub type D8<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA3<MODE>;
    pub type D9<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA2<MODE>;
    pub type D10<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA1<MODE>;
    pub type D11<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA0<MODE>;
    pub type D12<MODE> = ::stm32f1xx_hal::gpio::gpioc::PC15<MODE>;
    pub type D13<MODE> = ::stm32f1xx_hal::gpio::gpioc::PC14<MODE>;
    pub type D14<MODE> = ::stm32f1xx_hal::gpio::gpioc::PC13<MODE>;
    // Left
    pub type D15<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB7<MODE>;
    pub type D16<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB6<MODE>;
    pub type D17<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB5<MODE>;
    pub type D18<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB4<MODE>;
    pub type D19<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB3<MODE>;
    pub type D20<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA15<MODE>;
    pub type D21<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA14<MODE>;
    pub type D22<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA13<MODE>;
    pub type D23<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA12<MODE>;
    pub type D24<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA11<MODE>;
    pub type D25<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA10<MODE>;
    pub type D26<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA9<MODE>;
    pub type D27<MODE> = ::stm32f1xx_hal::gpio::gpioa::PA8<MODE>;
    pub type D28<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB15<MODE>;
    pub type D29<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB14<MODE>;
    pub type D30<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB13<MODE>;
    pub type D31<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB12<MODE>;
    pub type D32<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB8<MODE>;
    pub type D33<MODE> = ::stm32f1xx_hal::gpio::gpiob::PB1<MODE>;
    // Named
    pub type ButPin<MODE> = D32<MODE>;
    pub type LedPin<MODE> = D33<MODE>;
}
