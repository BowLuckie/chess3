use crate::moves::{Coordinate, Move};

#[derive(Debug, Clone)]
pub struct InputState {
    pub selected: Option<Coordinate>,
    pending_move: Option<Move>
}

impl InputState {
    pub fn new() -> Self {
        Self { selected: None, pending_move: None }
    }

    pub fn take_pending(&mut self) -> Option<Move> {
        self.pending_move.take()
    }

    pub fn push_pending(&mut self, mv: Option<Move>) {
        self.pending_move = mv; 
    }
}