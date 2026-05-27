#![allow(clippy::needless_return)]

mod board;
mod draw;
mod moves;

use std::{thread, time::Duration};

use board::Board;

fn main() {
    let (prow, pcol) = (0, 1);
    let mut board = Board::new();
    println!("{}", board);
    let e4 = moves::Move::from((prow, pcol), (prow + 2, pcol));

    board.raw_move(e4);
    draw::draw(&board);

}
