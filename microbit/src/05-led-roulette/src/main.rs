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

    let mut u = [0, 0];
    let mut v = [1, 0];
    let mut light_seq = vec![u];

    stroke(&mut light_seq, &mut u, v, 4);

    let mut grid = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    loop {
        for seq in light_seq {
            (x, y) = seq;
            grid[x][y] = 1;
            display.show(&mut timer, grid, 2000);
        }
    }
}

//fn stroke(
//    light_seq: &mut Vec<[isize; 2]>,
//    u: &mut [isize; 2],
//    v: [isize; 2],
//    n: isize,
//) {
//    for _ in 1..=n {
//        // Note: x and y are swapped in the matrix -> nested vector mental model
//        u[0] += v[1];
//        u[1] += v[0];
//        light_seq.push(*u);
//    }
//}
