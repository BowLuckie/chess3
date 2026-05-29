use crate::moves::{Colour, Coordinate, Move, Piece, PieceKind};
use std::{fmt, ops::Not};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    pub to_move: Colour,
    pub white_king: Coordinate,
    pub black_king: Coordinate,
}

pub struct SquareIter<'a> {
    board: &'a Board,
    idx: usize,
}

impl<'a> Iterator for SquareIter<'a> {
    type Item = (Option<Piece>, i8, i8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= 64 {
            return None;
        }

        let row = 7 - (self.idx / 8) as i8;
        let col = (self.idx % 8) as i8;

        let piece = self.board.get_piece(row, col);

        self.idx += 1;

        Some((*piece, row, col))
    }
}

pub fn square_iter() -> impl Iterator<Item = (i8, i8)> {
    (0..64).map(|idx| {
        let row = 7 - (idx / 8) as i8;
        let col = (idx % 8) as i8;
        (row, col)
    })
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
            black_king: (0, 4),
            white_king: (7, 4),
        }
    }

    pub fn as_iter(&self) -> SquareIter<'_> {
        SquareIter { board: self, idx: 0 }
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

    pub fn raw_move(&mut self, mv: Move, simulate: bool) {
        use PieceKind::*;
        use Colour::*;

        let (orow, ocol) = mv.from;
        let (trow, tcol) = mv.to;

        let mut piece = self.get_piece(orow, ocol)
            .expect(&format!("no piece to move at {} {}", orow, ocol));

        piece.has_moved = true;

        if piece.kind == King {
            match piece.colour {
                White => self.white_king = mv.to,
                Black => self.black_king = mv.to,
            }
        }

        self.squares[orow as usize][ocol as usize] = None;
        self.squares[trow as usize][tcol as usize] = Some(piece);
        if let Some(winner) = self.check_game_over(self.to_move) {
            if !simulate {println!("gameover! {:?}", winner)}
        }
    }

    pub fn check_move(&self, mv: Move) -> bool {
        if !self.get_moves_unchecked(mv.from.0, mv.from.1, false).contains(&mv) {
            return false;
        }

        let mut copy: Board = self.clone();
        copy.raw_move(mv, true);
        return !copy.king_in_check(self.to_move); 
    }

    pub fn switch_turn(&mut self) {
        self.to_move = !self.to_move;
    }

    pub fn king_in_check(&self, colour: Colour) -> bool {
        let king_pos = match colour {
            Colour::White => self.white_king,
            Colour::Black => self.black_king,
        };
        for (piece, row, col) in self.as_iter() {
            let Some(p) = piece else {
                continue;
            };

            if p.colour == colour {
                continue;
            }

            if self
                .get_moves_unchecked(row, col, true)
                .iter()
                .any(|mv| mv.to == king_pos)
            {
                return true;
            }
    }
        false
    }

    pub fn check_game_over(&self, colour: Colour) -> Option<Colour> {
        return None;
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

        for (row, col) in square_iter() {
            // Print row label at the start of each row
            if col == 0 {
                out.push_str(&format!("{} ", row));
            }

            let piece = self.get_piece(row, col);
            out.push_str(&format!("[{}]", get_lexrep(piece)));

            // End of row → newline
            if col == 7 {
                out.push('\n');
            }
        }

        // Column labels
        out.push_str("  ");
        for col in 0..8 {
            out.push_str(&format!(" {} ", col));
        }
        out.push('\n');

        write!(f, "{}", out)
    }
}
