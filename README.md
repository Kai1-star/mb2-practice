# mb2-practice

My exercises for the [Discovery MB2](https://docs.rust-embedded.org/discovery-mb2/) book: embedded Rust on the BBC micro:bit v2.

## About

I'm learning bare-metal Rust on Cortex-M as groundwork for my main project,
[sdv-mini](https://github.com/Kai1-star/sdv-mini), a small software-defined vehicle
built from two micro:bit v2 ECUs talking over a real CAN bus.

Each chapter lives in its own folder. I type the code myself rather than copying it,
then break it on purpose and add a small variation of my own.
Commit messages record **what I learned**, not just what changed.

## Hardware & Tools

- **Board:** BBC micro:bit v2 (nRF52833, Arm Cortex-M4F)
- **Target:** `thumbv7em-none-eabihf`
- **Flash & debug:** [probe-rs](https://probe.rs) (`cargo embed`, `probe-rs run`), GDB
- **Host:** macOS (Apple Silicon)

## Chapters

| Chapter | Topic | Folder | Status |
|---------|-------|--------|--------|
| 5 | Meet your software | done in the book repo | ✅ |
| 6 | Hello, world (blinking) | done in the book repo | ✅ |
| 7 | LED roulette | [`led_roulette/`](led_roulette/) | ✅ |
| 8 | Inputs and outputs | — | ⬜ |
| 9 | Registers | — | ⬜ |
| 10 | Serial port | — | ⬜ |
| 11 | UART | — | ⬜ |
| 12 | I2C | — | ⬜ |
| 13 | LED compass | — | ⬜ |
| 14 | Punch-o-meter (accelerometer) | — | ⬜ |
| 15 | Interrupts | — | ⬜ |
| 16 | Snake game | — | ⬜ |

## How to run

You need the `thumbv7em-none-eabihf` target and probe-rs installed (see chapter 3 of the book).
With the micro:bit plugged in via USB:

```bash
cd led_roulette
cargo run
```

`cargo run` flashes and starts the program through `probe-rs run`. The build target,
runner and linker flags come from `.cargo/config.toml`.
`cargo embed` uses `Embed.toml` instead, which halts the chip after flashing
and opens a GDB server for debugging.

## Credits

Based on [the micro:bit v2 Embedded Discovery Book](https://github.com/rust-embedded/discovery-mb2)
by the Rust Embedded Working Group. The book's code is licensed under MIT or Apache-2.0,
and its text under CC-BY-4.0.
