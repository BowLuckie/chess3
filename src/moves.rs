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
    Knight,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub colour: Colour,
    pub has_moved: bool,
}

impl PieceKind {
    fn base_char(&self) -> char {
        match self {
            PieceKind::Pawn   => 'P',
            PieceKind::Knight => 'k',
        }
    }
}

pub fn get_lexrep(piece: &Option<Piece>) -> String {
    match piece {
        Some(p) => {
            let c = p.kind.base_char();
            match p.colour {
                Colour::White => c.to_string(),
                Colour::Black => c.to_lowercase().to_string(),
            }
        }
        None => " ".to_string(),
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
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
        use PieceKind::*;
        match p.kind {
            Pawn => self.pawn_moves(p, row, col),
            Knight => self.knight_moves(p, row, col),
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

    fn knight_moves(&self, p: Piece, row: i8, col: i8) -> Vec<Move> {
        const KNIGHT_DELTAS: [(i8, i8); 8] = [
            (-2, -1), (-2, 1),
            (-1, -2), (-1, 2),
            ( 1, -2), ( 1, 2),
            ( 2, -1), ( 2, 1),
        ];
        let origin = (row, col);
        let moves: Vec<Move> = KNIGHT_DELTAS.iter().filter_map(|(dr, dc)| {
            let (nr, nc) = (row + dr, col + dc);
            if !(0..8).contains(&nr) || !(0..8).contains(&nc) { return None; }
            let target = self.get_piece(nr, nc);
            if target.is_some_and(|t | t.colour == p.colour) { return None; }
            Some(Move::new(origin, (nr, nc)))
        }).collect();
        return moves;
    }
}
