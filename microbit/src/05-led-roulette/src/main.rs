#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{Timer},
};

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    const LIGHT_SEQ: [(usize, usize); 17] = [
        (0, 0), (0, 1), (0, 2), (0, 3), (0, 4),
        (1, 4), (2, 4), (3, 4), (4, 4),
        (4, 3), (4, 2), (4, 1), (4, 0),
        (4, 0), (3, 0), (2, 0), (1, 0),
    ];

    let mut grid = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut u_old = (0, 0);
    loop {
        for u_new in LIGHT_SEQ {
            grid[u_old.0][u_old.1] = 0;
            grid[u_new.0][u_new.1] = 1;
            display.show(&mut timer, grid, 40);
            u_old = u_new;
        }
    }
}
