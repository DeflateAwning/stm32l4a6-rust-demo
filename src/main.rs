#![no_std]
#![no_main]
extern crate cortex_m;
use cortex_m_rt::entry;

use panic_halt as _;

use rtt_target::{rprintln, rtt_init_print};


#[entry]
fn entry_point() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");
    let mut i = 0;
    loop {
        i += 1;
        rprintln!("i = {}", i);
        cortex_m::asm::delay(8_000_000);
    }
}
