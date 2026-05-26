#![allow(clippy::needless_return)]

mod board;
mod draw;
mod moves;

use board::Board;

fn main() {
    let board = Board::new();
    println!("{}", board);
    println!("{:?}", board.get_piece(0, 1));
    println!("{:?}", board.get_moves(0, 1));
    draw::draw(&board);
}
