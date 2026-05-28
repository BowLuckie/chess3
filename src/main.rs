#![allow(clippy::needless_return)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread, time::Duration};
use board::Board;
use crate::{input::InputState, moves::Piece};

mod board;
mod window;
mod moves;
mod input;


fn main() {
    let board: Arc<Mutex<Board>> = Arc::new(Mutex::new(Board::new()));
    let ready: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let input: Arc<Mutex<InputState>> = Arc::new(Mutex::new(InputState::new()));
    let logic_input = Arc::clone(&input);

    let logic_board = Arc::clone(&board);
    let window_ready = Arc::clone(&ready);

    
    thread::spawn(move || {
        while !window_ready.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(10));
        }

        logic(logic_board, logic_input);
    });

    window::create_window(board, ready, input);
}

/// unlocks the board and computes a closure on it
fn with_board<T>(board: &Arc<Mutex<Board>>, f: impl FnOnce(&mut Board) -> T) -> T {
    f(&mut board.lock().unwrap())
}

fn logic(board: Arc<Mutex<Board>>, input: Arc<Mutex<InputState>>) {

    with_board(&board, |board_| println!("{}", board_));

    loop {
        let mv = input.lock().unwrap().pending_move.take();


        if let Some(mv) = mv {
            let piece: &Option<Piece> = with_board(&board, |b| -> &Option<Piece> {b.get_piece(mv.from.0, mv.from.1)});
            with_board(&board, |b| b.checked_move(mv))
        }

        thread::sleep(Duration::from_millis(16));
    }
}