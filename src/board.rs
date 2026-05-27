use crate::moves::{Colour, Piece, PieceKind, get_lexrep};
use std::fmt;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    pub to_move: Colour,
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
            kind,
            colour,
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
