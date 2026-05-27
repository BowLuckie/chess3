#![allow(clippy::needless_return)]

mod board;
mod draw;
mod moves;

use board::Board;

fn main() {
    let (prow, pcol) = (7, 3);
    let board = Board::new();
    println!("{}", board);
    println!("{:?}", board.get_piece(prow, pcol));
    println!("{:?}", board.get_moves(prow, pcol));
    draw::draw(&board);
}
