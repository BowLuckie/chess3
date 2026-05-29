#![allow(clippy::needless_return)]

use crate::{
    input::InputState,
    moves::{Move, PieceKind},
};
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
    let board: Arc<Mutex<Board>> = Arc::new(Mutex::new(Board::test_board()));
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

        make_move(mv, board.clone());

        thread::sleep(Duration::from_millis(16));
    }
}

pub fn make_move(mv: Option<Move>, board: Arc<Mutex<Board>>) {
    if let Some(mv) = mv {
        with_board(&board, |b| {
            if b.check_move(mv) {
                b.raw_move(mv);
                println!("{:?} {:?}", mv, b.get_piece(mv.to.0, mv.to.1));
                if (mv.to.1 - mv.from.1).abs() > 1
                    && b.get_piece(mv.to.0, mv.to.1)
                        .is_some_and(|p| p.kind == PieceKind::King)
                {
                    let rank = mv.to.0;
                    let (rook_from, rook_to) = if mv.to.1 == 6 {
                        ((rank, 7), (rank, 5))
                    } else {
                        ((rank, 0), (rank, 3))
                    };
                    b.raw_move(Move::new(rook_from, rook_to));
                }
                b.switch_turn();
                b.gamestate = b.get_gamestate(b.to_move);
            }
        })
    }
}
