#![no_std]
#![no_main]
extern crate cortex_m;
use cortex_m_rt::entry;

use panic_halt as _;

use stm32l4::stm32l4x6 as pac;

use rtt_target::{rprintln, rtt_init_print};


#[entry]
fn entry_point() -> ! {
    rtt_init_print!();

    let peripheral = pac::Peripherals::take().unwrap();
    let _gpioa = &peripheral.GPIOA;
    let gpioc = &peripheral.GPIOC;
    let rcc = &peripheral.RCC;

    // Enable GPIO-C clock.
    rcc.ahb2enr.modify(|_, w| w.gpiocen().set_bit());
    // Set PC7 as output.
    gpioc.moder.modify(|_, w| w.moder7().output());

    rprintln!("Hello, world!");
    let mut i = 0;
    loop {
        // Toggle PC7.
        gpioc.odr.modify(|_, w| w.odr7().bit(i % 2 == 0));

        i += 1;
        rprintln!("i = {}", i);
        cortex_m::asm::delay(4_000_000);
    }
}
