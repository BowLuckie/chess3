#![allow(clippy::needless_return)]

use crate::input::InputState;
use board::Board;
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

mod board;
mod input;
mod moves;
mod window;

fn main() {
    let board: Arc<Mutex<Board>> = Arc::new(Mutex::new(Board::checkmate_test()));
    let ready_flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let input: Arc<Mutex<InputState>> = Arc::new(Mutex::new(InputState::new()));

    let logic_input: Arc<Mutex<InputState>> = Arc::clone(&input);

    let logic_board: Arc<Mutex<Board>> = Arc::clone(&board);
    let window_pointer: Arc<AtomicBool> = Arc::clone(&ready_flag);

    thread::spawn(move || {
        while !window_pointer.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(10));
        }

        logic(logic_board, logic_input);
    });

    window::chess_window(board, ready_flag, input);
}

/// unlocks the board and computes a closure on it
fn with_board<T>(board: &Arc<Mutex<Board>>, f: impl FnOnce(&mut Board) -> T) -> T {
    f(&mut board.lock().unwrap())
}

fn logic(board: Arc<Mutex<Board>>, input: Arc<Mutex<InputState>>) {
    println!("");
    with_board(&board, |b| println!("{}", b));

    loop {
        let mv = input.lock().unwrap().take_pending();

        if let Some(mv) = mv {
            with_board(&board, |b| {
                if b.check_move(mv) {
                    b.raw_move(mv, false);
                    b.switch_turn();
                    b.gamestate = b.get_gamestate(b.to_move);
                }
            })
        }

        thread::sleep(Duration::from_millis(16));
    }
}
