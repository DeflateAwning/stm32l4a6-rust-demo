#![no_std]
#![no_main]
extern crate cortex_m;
use cortex_m_rt::entry;

use panic_halt as _;



#[entry]
fn entry_point() -> ! {
    loop {}
}
