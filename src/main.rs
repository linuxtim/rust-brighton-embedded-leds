#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m;
use cortex_m_rt::entry;

use cortex_m_semihosting::{debug, hprintln};
use crate::hal::{prelude::*, stm32};
use stm32f4xx_hal as hal;

// For the real hardware use device = stm32f4xx_hal
// For Qemu, use lm3s6965 (doesn't work at the moment)

#[rtfm::app(device = stm32f4xx_hal)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {
        // Access the device peripherals (dp) and cortex peripherals (cp):
        if let (Some(dp), Some(cp)) = (
            stm32::Peripherals::take(),
            cortex_m::peripheral::Peripherals::take(),
        ) {
            // Set up the LED: it's connected to pin PA5 on the microcontroler
            let gpioa = dp.GPIOA.split();
            let mut led = gpioa.pa5.into_push_pull_output();
            
            // The external LED, on the next pin down:
            let mut xled = gpioa.pa6.into_push_pull_output();
            
            // Set up the system clock. We want to run at 48MHz for this one.
            let rcc = dp.RCC.constrain();
            let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
            
            // Create a delay abstraction based on SysTick
            let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
            
            // Let's mutate this from debugger...
            let mut ms = 50_u32;    
            
            loop {
                // On for / off for: 0.5s
                // https://doc.rust-lang.org/std/convert/enum.Infallible.html
                hprintln!("Blink!").unwrap();
                led.set_high().unwrap();
                xled.set_low().unwrap();
                delay.delay_ms(ms);
                led.set_low().unwrap();
                xled.set_high().unwrap();
                delay.delay_ms(ms);
            }
        } else {
            panic!("failed to access peripherals");
        }
    }

};
