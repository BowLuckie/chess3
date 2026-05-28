use crate::moves::{Colour, Coordinate, Move, Piece, PieceKind};
use std::{fmt, ops::Not};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    to_move: Colour,
}

impl PieceKind {
    fn base_char(&self) -> char {
    use PieceKind::*;
        match self {
            Pawn   => 'P',
            Knight => 'N',
            Queen => 'Q',
            Rook => 'R',
            Bishop => 'B',
            King => 'K',
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

impl Board {
    pub fn new() -> Self {
        use Colour::*;
        use PieceKind::*;

        let mut squares: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        let place = |
            squares: &mut [[Option<Piece>; 8]; 8],
            row: usize,
            col: usize,
            kind: PieceKind,
            colour: Colour,
        | {
            squares[row][col] = Some(Piece {
                kind,
                colour,
                has_moved: false,
            });
        };

        for col in 0..8 {
            place(&mut squares, 1, col, Pawn, Black);
            place(&mut squares, 6, col, Pawn, White);
        }

        for col in [0, 7] {
            place(&mut squares, 0, col, Rook, Black);
            place(&mut squares, 7, col, Rook, White);
        }

        for col in [1, 6] {
            place(&mut squares, 0, col, Knight, Black);
            place(&mut squares, 7, col, Knight, White);
        }

        for col in [2, 5] {
            place(&mut squares, 0, col, Bishop, Black);
            place(&mut squares, 7, col, Bishop, White);
        }

        place(&mut squares, 0, 3, Queen, Black);
        place(&mut squares, 7, 3, Queen, White);

        place(&mut squares, 0, 4, King, Black);
        place(&mut squares, 7, 4, King, White);

        Self {
            squares,
            to_move: White,
        }
    }

    pub fn get_piece(&self, row: i8, col: i8) -> &Option<Piece> {
        assert!(row < 8 && col < 8, "row or col exceeds 7: {} {}", row, col);
        &self.squares[row as usize][col as usize]
    }

    pub fn get_piece_by_cord(&self, coordinate: Coordinate) -> &Option<Piece> {
        let row = coordinate.0;
        let col = coordinate.1;
        self.get_piece(row, col)
    }

    pub fn raw_move(&mut self, mv: Move) {
        let (orow, ocol) = mv.from;
        let (trow, tcol) = mv.to;

        let piece = self.get_piece(orow, ocol)
            .expect(&format!("no piece to move at {} {}", orow, ocol));

        self.squares[orow as usize][ocol as usize] = None;
        self.squares[trow as usize][tcol as usize] = Some(piece);
    }

    pub fn check_move(&self, mv: Move) -> bool {
        let origin: Coordinate = mv.from;
        let (orow, ocol) = origin;
        let piece: Option<Piece> = *self.get_piece_by_cord(origin);
        if piece.is_none() {
            return false;
        }
        let moves_unchecked = self.get_moves(orow, ocol);
        // TODO move unwinding
        return moves_unchecked.contains(&mv);
    }

    pub fn switch_turn(&mut self) {
        self.to_move = !self.to_move;
    }

    pub fn turn(&self) -> Colour {
        self.to_move
    }
}

impl Not for Colour {
    type Output = Self;
    fn not(self) -> Self::Output {
        use Colour::*;
        match self {
            White => Black,
            Black => White,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();

        for row in 0..8 {
            out.push_str(&format!("{} ", row));

            for col in 0..8 {
                out.push_str(&format!("[{}]", get_lexrep(self.get_piece(row, col))));
            }

            out.push('\n');
        }

        out.push_str("  ");
        for col in 0..8 {
            out.push_str(&format!(" {} ", col));
        }
        out.push('\n');

        write!(f, "{}", out)
    }
}
