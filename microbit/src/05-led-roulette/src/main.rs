#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
//use rtt_target::{rtt_init_print, rprintln};
//use panic_rtt_target as _;
use panic_halt as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    //rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut grid = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    fn update_grid(leds: &mut [[u8; 5]; 5]) {
        if leds[0][0] == 0 {
            leds[0][0] = 1;
        } else if leds[0][0] == 1 {
            leds[0][0] = 0;
            leds[0][1] = 1;
        } else if leds[0][1] == 1 {
            leds[0][1] = 0;
            leds[0][0] = 1;
        }
    }

    loop {
        display.show(&mut timer, grid, 2000);
        update_grid(&mut grid);
        //timer.delay_ms(2000_u32);
    }
}
