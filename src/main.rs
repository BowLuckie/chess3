mod board;
mod draw;
mod moves;

use board::Board;

fn main() {
    let board = Board::new();
    println!("{}", board);
    println!("{:?}", board.get_piece(6, 0));
    println!("{:?}", board.get_moves(6, 0));
    draw::draw(&board);
}
