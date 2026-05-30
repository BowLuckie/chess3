use std::sync::{Arc, Mutex};

use raylib::RaylibHandle;

use crate::{
    board::Board,
    moves::{Coordinate, Move},
    window::TILE_SIZE,
};

#[derive(Debug, Clone)]
pub struct InputState {
    pub selected: Option<Coordinate>,
    pending_move: Option<Move>,
    pub legal_moves: Vec<Move>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            selected: None,
            pending_move: None,
            legal_moves: vec![],
        }
    }

    pub fn take_pending(&mut self) -> Option<Move> {
        self.pending_move.take()
    }

    pub fn push_pending(&mut self, mv: Option<Move>) {
        self.pending_move = mv;
    }
}

pub fn handle_click(board: &Arc<Mutex<Board>>, input: &Arc<Mutex<InputState>>, rl: &RaylibHandle) {
    let col = (rl.get_mouse_x() / TILE_SIZE) as i8;
    let row = (rl.get_mouse_y() / TILE_SIZE) as i8;
    let new = (row, col);

    let (selected_old, _legal_moves, _pending) = {
        let input_state = input.lock().unwrap();
        (
            input_state.selected,
            input_state.legal_moves.clone(),
            None::<Move>,
        )
    };

    let (legal_moves, pending) = {
        let board_guard = board.lock().unwrap();
        match selected_old {
            Some(old) if old == new => (vec![], None),
            Some(old) => {
                let moves = board_guard.get_moves(row, col);
                (moves, Some(Move::new(old, new)))
            }
            None => (board_guard.get_moves(row, col), None),
        }
    };

    let mut input_state = input.lock().unwrap();
    match selected_old {
        Some(old) if old == new => {
            input_state.selected = None;
            input_state.legal_moves.clear();
        }
        _ => {
            input_state.selected = Some(new);
            input_state.legal_moves = legal_moves;
            if let Some(mv) = pending {
                input_state.push_pending(Some(mv));
            }
        }
    }
}
