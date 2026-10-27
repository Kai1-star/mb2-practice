#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::hal::timer::Timer;
use microbit::{hal::gpio};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use microbit::{board::Board, display :: blocking ::Display, pac::usbd::tasks_startepin::TASKS_STARTEPIN_SPEC};





#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // Configure buttons
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;
  

// create the Display
let mut display = Display::new(board.display_pins);
// and light up some LEDs
let link_arrow = [
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 1, 1, 1, 1],
    [0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0],
];


let right_arrow = [
    [0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0],
    [1, 1, 1, 1, 1],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
];


let mittle = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

let blank = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];





    loop {
        let on_pressed = button_a.is_low().unwrap();
        let off_pressed = button_b.is_low().unwrap();
        match (on_pressed, off_pressed) {
            // Stay in current state until something is pressed.
            (false, false) => {
                display.show(&mut timer, mittle, 1000);
      
            },
            // Change to on state.
            (true, false) => {
                display.show(&mut timer, link_arrow, 1000);
                // display.clear();
                 timer.delay_ms(2500);
                 display.show(&mut timer, blank, 100);

            },
            // Change to off state.
            (false, true) => {
                display.show(&mut timer, right_arrow, 1000);
                // display.clear();
                // timer.delay_ms(250);
                timer.delay_ms(2500);
                 display.show(&mut timer, blank, 100);
            },
            // Stay in current state until something is released.
            (true, true) => (),
        }
        timer.delay_ms(10_u32);
    }
}