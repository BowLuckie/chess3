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
    Queen,
    Rook,
    Bishop,
    King
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub colour: Colour,
    pub has_moved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

fn in_bounds_point(point: Coordinate) -> bool {
    (0..8).contains(&point.0) && (0..8).contains(&point.1)
}

impl Move {
      
    pub fn new(from: Coordinate, to: Coordinate) -> Self {
        if !(in_bounds_point(from) && in_bounds_point(to)) {
            panic!("attempted to create a move out ofo bounds!")
        }
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
        if p.colour != self.turn() {
            return vec![];
        }
        match p.kind {
            Pawn => self.pawn_moves(p, row, col),
            Knight => self.knight_moves(p, row, col),
            Queen => self.queen_moves(p, row, col),
            Rook => self.rook_moves(p, row, col),
            Bishop => self.bishop_moves(p, row, col),
            King => self.king_moves(p, row, col),
        }
    }

    fn in_bounds(&self, r: i8, c: i8) -> bool {
        (0..8).contains(&r) && (0..8).contains(&c)
    }

    fn pawn_moves(&self, p: Piece, row: i8, col: i8) -> Vec<Move> {
        let dir = match p.colour {
            Colour::Black => 1,
            Colour::White => -1,
        };

        let mut moves = Vec::new();
        let origin = (row, col);
        let new_row = row + dir;

        if self.in_bounds(new_row, col) && self.get_piece(new_row, col).is_none() {
            moves.push(Move::new(origin, (new_row, col)));

            let two_row = new_row + dir;
            if !p.has_moved && self.in_bounds(two_row, col) && self.get_piece(two_row, col).is_none() {
                moves.push(Move::new(origin, (two_row, col)));
            }
        }

        for dc in [-1, 1] {
            let new_col = col + dc;
            if self.in_bounds(new_row, new_col) {
                if let Some(target) = self.get_piece(new_row, new_col) {
                    if target.colour != p.colour {
                        moves.push(Move::new(origin, (new_row, new_col)));
                    }
                }
            }
        };

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

    fn raw_slide(&self, p: Piece, row: i8, col: i8, directions: Vec<Coordinate>, max: Option<i8>) -> Vec<Move> {
        let mut moves = Vec::new();
        let origin = (row, col);

        for (drow, dcol) in directions {
            let (mut trow, mut tcol) = (row + drow, col + dcol);
            let mut distance: i8 = 0;

            while self.in_bounds(trow, tcol) && distance < max.unwrap_or(8) {
                let target = self.get_piece(trow, tcol);

                if target.is_none() {
                    moves.push(Move::new(origin, (trow, tcol)));
                } else if target.unwrap().colour != p.colour {
                    moves.push(Move::new(origin, (trow, tcol)));
                    break;
                } else {
                    break;
                }

                trow += drow;
                tcol += dcol;
                distance += 1;
            }
        }

        return moves;
    }

    fn queen_moves(&self, p: Piece, row: i8, col: i8) -> Vec<Move> {
        let directions = vec![
            (1, 0), 
            (1, 1), 
            (0, 1),     
            (-1, 1),
            (-1, 0),     
            (-1, -1),
            (0, -1),    
            (1, -1),
        ];
        return self.raw_slide(p, row, col, directions, None);
    }

    fn rook_moves(&self, p: Piece, row: i8, col: i8) -> Vec<Move> {
       let directions = vec![
            (1, 0), 
            (0, 1),     
            (-1, 0),     
            (0, -1),    
        ];
        return self.raw_slide(p, row, col, directions, None);
    }

    fn bishop_moves(&self, p: Piece, row: i8, col: i8) -> Vec<Move> {
       let directions = vec![
            (1, 1), 
            (-1, 1),
            (-1, -1),
            (1, -1),
        ];
        return self.raw_slide(p, row, col, directions, None);
    }
    
    fn king_moves(&self, p: Piece, row: i8, col: i8) -> Vec<Move> {
        let directions = vec![
            (1, 0), 
            (1, 1), 
            (0, 1),     
            (-1, 1),
            (-1, 0),     
            (-1, -1),
            (0, -1),    
            (1, -1),
        ];
        return self.raw_slide(p, row, col, directions, Some(1));
    }
}
