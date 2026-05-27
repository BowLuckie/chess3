#![allow(clippy::needless_return)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread, time::Duration};

mod board;
mod draw;
mod moves;

use board::Board;

fn main() {
    let (prow, pcol) = (6, 4);
    let board = Arc::new(Mutex::new(Board::new()));
    let ready = Arc::new(AtomicBool::new(false));

    let b = board.lock().unwrap();
    println!("{:?}", b.get_moves(prow, pcol));
    println!("{:?}", b.get_piece(prow, pcol));
    println!("{}", b);
    drop(b);
    
    #[allow(unused_variables)] 
    let logic_board = Arc::clone(&board);
    let window_ready = Arc::clone(&ready);
    thread::spawn(move || {
        while !window_ready.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(10));
        }

        logic(logic_board);
    });

    draw::draw(board, ready);
    
}

fn logic(board: Arc<Mutex<Board>>) {
    // example
    let (prow, pcol) = (6, 4);
    let e4 = moves::Move::from((prow, pcol), (prow - 2, pcol));

    thread::sleep(Duration::from_secs(3));
    board.lock().unwrap().raw_move(e4);

}