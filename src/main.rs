#![no_std]
#![no_main]
extern crate cortex_m;
use cortex_m_rt::entry;

use core::panic::PanicInfo;

use hal::prelude::*;
use hal::serial::{Config, Serial};
use stm32l4xx_hal as hal;

use nb::block;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn entry_point() -> ! {
    rtt_init_print!();
    rprintln!("rtt_init_print() done.");

    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let peripheral = hal::stm32::Peripherals::take().unwrap();

    // Clocks / power
    let mut rcc = peripheral.RCC.constrain();
    let mut flash = peripheral.FLASH.constrain();
    let mut pwr = peripheral.PWR.constrain(&mut rcc.apb1r1);
    let clocks = rcc.cfgr.sysclk(64.MHz()).freeze(&mut flash.acr, &mut pwr);
    rprintln!("Clocks are configured.");

    // Systick-based delay
    let mut timer = hal::delay::Delay::new(cortex_peripherals.SYST, clocks);

    // GPIO
    let mut _gpioa = peripheral.GPIOA.split(&mut rcc.ahb2);
    let mut gpioc = peripheral.GPIOC.split(&mut rcc.ahb2);
    let mut gpiod = peripheral.GPIOD.split(&mut rcc.ahb2);
    let mut led = gpioc
        .pc7
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    // -------------------------
    // USART2 @ 115_200 8N1
    // Pins: PD5 (TX), PD6 (RX), AF7 (alternate function 7)
    // -------------------------
    let tx = gpiod
        .pd5
        .into_alternate(&mut gpiod.moder, &mut gpiod.otyper, &mut gpiod.afrl);
    let rx = gpiod
        .pd6
        .into_alternate(&mut gpiod.moder, &mut gpiod.otyper, &mut gpiod.afrl);

    // Configure and enable the peripheral
    let serial_cfg = Config::default().baudrate(115_200.bps());
    let mut serial = Serial::usart2(
        peripheral.USART2,
        (tx, rx),
        serial_cfg,
        clocks,
        &mut rcc.apb1r1,
    );

    rprintln!("USART2 up at 115200 8N1. Echo task running…");

    // Simple heartbeat + echo
    let mut i: u32 = 0;
    loop {
        // Try to read a byte; if one arrives, echo it right back.
        while let Ok(b) = serial.read() {
            // Echo the received byte.
            block!(serial.write(b)).ok();

            // Also mirror to RTT for visibility.
            // (Printable ASCII guard to keep RTT tidy).
            let ch = core::char::from_u32(b as u32).unwrap_or('.');
            rprintln!(
                "RX: 0x{:02X} '{}'",
                b,
                if ch.is_ascii_graphic() { ch } else { '.' }
            );
        }

        // Blink LED at ~2 Hz
        led.toggle();
        i = i.wrapping_add(1);
        if (i & 0x01) == 0 {
            rprintln!("i = {}", i);
        }
        timer.delay_ms(500_u16);
    }
}



#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {}
}
