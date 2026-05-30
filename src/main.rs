#![allow(clippy::identity_op)]
#![allow(clippy::needless_return)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::cast_lossless)]

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
    let board: Arc<Mutex<Board>> = Arc::new(Mutex::new(Board::new()));
    let ready_flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let input: Arc<Mutex<InputState>> = Arc::new(Mutex::new(InputState::new()));

    let logic_input: Arc<Mutex<InputState>> = Arc::clone(&input);

    let logic_board: Arc<Mutex<Board>> = Arc::clone(&board);
    let window_pointer: Arc<AtomicBool> = Arc::clone(&ready_flag);

    thread::spawn(move || {
        while !window_pointer.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(10));
        }

        logic(&logic_board, &logic_input);
    });

    window::chess_window(&board, &ready_flag, &input);
}

/// unlocks the board and computes a closure on it
fn with_board<T>(board: &Arc<Mutex<Board>>, f: impl FnOnce(&mut Board) -> T) -> T {
    f(&mut board.lock().unwrap())
}

fn logic(board: &Arc<Mutex<Board>>, input: &Arc<Mutex<InputState>>) {
    println!();
    with_board(board, |b| println!("{b}"));

    loop {
        if let Some(mv) = input.lock().unwrap().take_pending() {
            with_board(board, |b| {
                make_move(mv, b);
            });
        }

        thread::sleep(Duration::from_millis(16));
    }
}

pub fn make_move(mv: Move, b: &mut Board) {
    if !b.check_move(mv) {
        return;
    }

    let target = b.get_piece(mv.to.0, mv.to.1).is_some();
    b.raw_move(mv);
    let piece = b.get_piece(mv.to.0, mv.to.1);

    if (mv.to.1 - mv.from.1).abs() > 1 && piece.is_some_and(|p| p.kind == PieceKind::King) {
        let rank = mv.to.0;
        let (rook_from, rook_to) = if mv.to.1 == 6 {
            ((rank, 7), (rank, 5))
        } else {
            ((rank, 0), (rank, 3))
        };

        b.raw_move(Move::new(rook_from, rook_to));
    } else if target || piece.is_some_and(|p| p.kind == PieceKind::Pawn) {
        b.halfmove_clock = 0;
    }

    b.switch_turn();
    b.halfmove_clock += 1;
    b.gamestate = b.get_gamestate(b.to_move);
}
