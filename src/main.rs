use std::fmt;

pub type Coordinate = (u8, u8);

#[derive(Debug, Clone, Copy)]
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
    kind: PieceKind,
    colour: Colour,
}


#[derive(Debug, Clone, Copy)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    to_move: Colour,
}

#[derive(Debug, Clone, Copy)]
pub enum BoardMode {
    Standard,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    from: Coordinate,
    to: Coordinate,
}

impl Move {
    fn new_move(from: Coordinate, to: Coordinate) -> Move {
        Move {from: from, to: to}
    }
}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..8 {
            write!(f, "{} ", row)?;

            for col in 0..8 {
                let symbol = match self.squares[row][col] {
                    Some(piece) => match (piece.kind, piece.colour) {
                        (PieceKind::Pawn, Colour::White) => 'P',
                        (PieceKind::Pawn, Colour::Black) => 'p',
                    },
                    None => ' ',
                };

                write!(f, "[{}]", symbol)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Board {
    pub fn new() -> Self {
        let mut squares = [[None; 8]; 8];

        for col in 0..8 {
            squares[1][col] = Some(Piece {
                kind: PieceKind::Pawn,
                colour: Colour::Black,
            });
        }

        for col in 0..8 {
            squares[6][col] = Some(Piece {
                kind: PieceKind::Pawn,
                colour: Colour::White,
            });
        }

        Self { squares: squares, to_move: Colour::White }
    }

    fn dispatch(&self, p: Piece, row: u8, col: u8) -> Vec<Move> {
        use PieceKind::*;
        match p.kind {
            Pawn => self.pawn_moves(p, row, col) 
        }
    }

    pub fn get_piece(&self, row: u8, col: u8) -> &Option<Piece> {
        &self.squares[row as usize][col as usize]
    }

    pub fn get_moves(&self, row: u8, col: u8) -> Vec<Move> {
       match *self.get_piece(row, col) {
            Some(p) => self.dispatch(p, row, col),
            None => vec![],
       }
    }

    pub fn pawn_moves(&self, p: Piece, row: u8, col: u8) -> Vec<Move> {
        use Colour::*;
        let dir: i8 = match p.colour {
            Black => 1,
            White => -1,
        };

        let target_row = (row as i8 + dir) as u8;

        vec![Move::new_move((row, col), (target_row, col))]
    }
}

fn main() {
    let board = Board::new();
    println!("{}", board);

    println!("{:?}", board.get_piece(6, 0));
    println!("{:?}", board.get_moves(6, 0))
}