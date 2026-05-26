use crate::moves::{Colour, Piece, PieceKind, get_lexrep};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    pub to_move: Colour,
}

impl Board {
    pub fn new() -> Self {
        let mut squares = [[None; 8]; 8];
        for col in 0..8 {
            squares[1][col] = Some(Piece {
                kind: PieceKind::Pawn,
                colour: Colour::Black,
                has_moved: false,
            });
            squares[6][col] = Some(Piece {
                kind: PieceKind::Pawn,
                colour: Colour::White,
                has_moved: false,
            });
        }
        Self {
            squares,
            to_move: Colour::White,
        }
    }

    pub fn get_piece(&self, row: i8, col: i8) -> &Option<Piece> {
        assert!(row < 8 && col < 8, "row or col exceeds 7: {} {}", row, col);
        &self.squares[row as usize][col as usize]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..8 {
            write!(f, "{} ", row)?;
            for col in 0..8 {
                write!(f, "[{}]", get_lexrep(self.get_piece(row, col)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
