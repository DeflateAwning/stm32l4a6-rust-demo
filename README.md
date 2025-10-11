# STM32L4A6 Rust Demo

Working with the STM32L4A6ZG microcontroller via the Nucleo-L4A6ZG.

## Getting Started

1. Install the required dependencies.

```bash
rustup update
rustup target add thumbv7em-none-eabihf
rustup component add llvm-tools
cargo install cargo-binutils probe-rs-tools cargo-expand
```

2. Open this repo in VS Code.
3. Run `cargo embed` from the root of this repo, with the Nucleo-L4A6ZG plugged in.
4. Observe logs coming from the STM32. Observe the green onboard LED blinking.


## Guides/Notes

* Main guide: https://www.makerspace-online.com/make-your-microcontroller-apps-safe-and-secure-with-rust-2/
* User Manual: https://www.st.com/resource/en/user_manual/um2179-stm32-nucleo144-boards-mb1312-stmicroelectronics.pdf
* Maybe good: https://www.anyleaf.org/blog/writing-embedded-firmware-using-rust

## Alternative Ideas

* Could give this one a try: https://github.com/David-OConnor/stm32-hal

## Notable Dev Kit Pins

* LD1 = PC7 (Green LED)
* LD2 = PB7 (Blue LED)
* LD3 = PB14 (Red LED)
* USART2 TX = PA2 or PD5
* USART2 RX = PA3 or PD6

### Not Used
* PG7 = LPUART1 TX (via ST-LINK)
* PG8 = LPUART1 RX (via ST-LINK)

## Hints for Next Steps

1. Create two separate crates in a repo, and put as much testable logic as possible in the other platform-independent crate.
    * E.g., command parsing, the command list, etc. goes in that crate.
2. Consider adding defmt logging: https://github.com/knurling-rs/defmt
