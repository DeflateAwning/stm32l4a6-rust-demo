#![no_std]
#![no_main]
extern crate cortex_m;
use cortex_m_rt::entry;

use panic_halt as _;

use stm32l4xx_hal::{self as hal};
use stm32l4xx_hal::prelude::*;

use rtt_target::{rprintln, rtt_init_print};


#[entry]
fn entry_point() -> ! {
    rtt_init_print!();
    rprintln!("rtt_init_print() done.");

    let peripheral = hal::stm32::Peripherals::take().unwrap();
    let mut rcc = peripheral.RCC.constrain();
    // let mut gpioa = peripheral.GPIOA.split(&mut rcc.ahb2);
    let mut gpioc = peripheral.GPIOC.split(&mut rcc.ahb2);

    let mut led = gpioc.pc7.into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    // TODO: Read button


    rprintln!("Hello, world!");
    let mut i = 0;
    loop {
        // Toggle PC7.
        led.toggle();

        i += 1;
        rprintln!("i = {}", i);
        cortex_m::asm::delay(4_000_000);
    }
}
