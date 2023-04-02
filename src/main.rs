#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    #[cfg(feature = "embassy-stm32")]
    embassy_stm32_run();
    #[cfg(feature = "stm32f4xx-hal")]
    stm32f4xx_hal_run();
}

#[inline(always)]
#[cfg(feature = "stm32f4xx-hal")]
fn stm32f4xx_hal_run() -> ! {
    use stm32f4xx_hal::prelude::*;
    let p = stm32f4xx_hal::pac::Peripherals::take().unwrap();

    let bank = p.GPIOB.split();
    let pin = bank.pb0.into_push_pull_output();

    stm32f4xx_pin_loop(pin);
}

#[inline(never)]
#[cfg(feature = "stm32f4xx-hal")]
fn stm32f4xx_pin_loop(
    mut pin: stm32f4xx_hal::gpio::PB0<stm32f4xx_hal::gpio::Output<stm32f4xx_hal::gpio::PushPull>>,
) -> ! {
    loop {
        pin.set_high();
        pin.set_low();
    }
}

#[inline(always)]
#[cfg(feature = "embassy-stm32")]
fn embassy_stm32_run() -> ! {
    let p = embassy_stm32::init(Default::default());

    let pin = p.PB0;
    use embassy_stm32::gpio::{Level, Output, Speed};
    let output = Output::new(pin, Level::Low, Speed::High);

    embassy_stm32_pin_loop(output);
}

#[cfg(feature = "embassy-stm32")]
#[inline(never)]
fn embassy_stm32_pin_loop(
    mut output: embassy_stm32::gpio::Output<embassy_stm32::peripherals::PB0>,
) -> ! {
    loop {
        output.set_high();
        output.set_low();
    }
}
