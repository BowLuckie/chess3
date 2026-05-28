use std::sync::{Arc, Mutex};

use raylib::RaylibHandle;

use crate::{board::Board, moves::{Coordinate, Move}, window::TILE_SIZE};

#[derive(Debug, Clone)]
pub struct InputState { 
    pub selected: Option<Coordinate>,
    pending_move: Option<Move>,
    pub legal_moves: Vec<Move>,
}

impl InputState {
    pub fn new() -> Self {
        Self { selected: None, pending_move: None, legal_moves: vec![] }
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

    let mut input_state = input.lock().unwrap();
    let new = (row, col);
    let selected_old = input_state.selected;
    let board_guard = board.lock().unwrap();
    match selected_old {
        Some(old) => {
            if old == new {
                input_state.selected = None;
                input_state.legal_moves.clear();
            } else {
                let old_piece = board_guard.get_piece_by_cord(old);
                match old_piece {
                    Some(_) => {
                        input_state.selected = Some(new);
                        input_state.legal_moves = board_guard.get_moves(row, col);
                        input_state.push_pending(Some(Move::new(old, new))); 
                    },
                    None => {
                        input_state.selected = Some(new);
                        input_state.legal_moves = board_guard.get_moves(row, col)
                    },
                }
            }
        },
        None => {
            input_state.selected = Some(new); 
            input_state.legal_moves = board_guard.get_moves(row, col)
        },
    }
}
