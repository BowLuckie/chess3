use crate::board::Board;

pub type Coordinate = (i8, i8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceKind {
    Pawn,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub colour: Colour,
    pub has_moved: bool,
}

pub fn get_lexrep(piece: &Option<Piece>) -> &str {
    match piece {
        Some(p) => match (p.kind, p.colour) {
            (PieceKind::Pawn, Colour::White) => "P",
            (PieceKind::Pawn, Colour::Black) => "p",
        },
        None => " ",
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Move {
    pub fn new(from: Coordinate, to: Coordinate) -> Self {
        Move { from, to }
    }
}

impl Board {
    pub fn get_moves(&self, row: i8, col: i8) -> Vec<Move> {
        match *self.get_piece(row, col) {
            Some(p) => self.dispatch(p, row, col),
            None => vec![],
        }
    }

    fn dispatch(&self, p: Piece, row: i8, col: i8) -> Vec<Move> {
        match p.kind {
            PieceKind::Pawn => self.pawn_moves(p, row, col),
        }
    }

    fn pawn_moves(&self, p: Piece, row: i8, col: i8) -> Vec<Move> {
        let dir: i8 = match p.colour {
            Colour::Black => 1,
            Colour::White => -1,
        };
        let mut moves = Vec::new();
        let origin = (row, col);
        let new_row = row + dir;

        if (0..8).contains(&new_row) && self.get_piece(new_row, col).is_none() {
            moves.push(Move::new(origin, (new_row, col)));
            if !p.has_moved && self.get_piece(new_row + dir, col).is_none() {
                moves.push(Move::new(origin, (new_row + dir, col)));
            }
        }

        for dc in [-1i8, 1] {
            let new_col = col + dc;
            if (0..8).contains(&new_col) && (0..8).contains(&new_row) {
                let target = self.get_piece(new_row, new_col);
                if target.is_some_and(|t| t.colour != p.colour) {
                    moves.push(Move::new(origin, (new_row, new_col)));
                }
            }
        }

        moves
    }
}
