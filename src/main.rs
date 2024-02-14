#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use cortex_m_rt::entry;

use stm32l4xx_hal::delay::Delay;
use stm32l4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut led = gpioa
        .pa12
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let mut timer = Delay::new(cp.SYST, clocks);

    loop {
        led.toggle();

        timer.delay_ms(1_000_u16);
    }
}
