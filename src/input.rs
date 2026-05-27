use crate::moves::{Coordinate, Move};

pub struct InputState {
    pub selected: Option<Coordinate>,
    pub pending_move: Option<Move>
}

impl InputState {
    pub fn new() -> Self {
        Self { selected: None, pending_move: None }
    }
}