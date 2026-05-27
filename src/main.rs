#![allow(clippy::needless_return)]

use std::{io::Write, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread, time::Duration};
use board::Board;
use crate::input::InputState;

mod board;
mod draw;
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

    draw::draw(board, ready, input);
}

/// unlocks the board and computes a closure on it
fn with_board<T>(board: &Arc<Mutex<Board>>, f: impl FnOnce(&mut Board) -> T) -> T {
    f(&mut board.lock().unwrap())
}

fn logic(board: Arc<Mutex<Board>>, input: Arc<Mutex<InputState>>) {
    loop {
        let mv = input.lock().unwrap().pending_move.take();

        if let Some(mv) = mv {
            with_board(&board, |board_| board_.raw_move(mv))
        }

        thread::sleep(Duration::from_millis(16));
    }
}