#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED (C13)
        let gpio_c = dp.GPIOC.split();
        let mut led = gpio_c.pc13.into_push_pull_output();

        // set up inputs
        let gpio_b = dp.GPIOB.split();
        let b0 = gpio_b.pb0.into_pull_up_input();
        // let b1 = gpio_b.pb1.into_pull_up_input();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {

            if b0.is_low().unwrap() {
                led.set_low().unwrap();
            } else {
                led.set_high().unwrap();
            }

            // led.set_high().unwrap();
            delay.delay_ms(1_u32);
            // led.set_low().unwrap();
            // delay.delay_ms(1000_u32);
        }
    }

    loop {}
}
