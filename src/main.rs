use std::{fmt, vec};
use crate::Colour::{Black, White};

pub type Coordinate = (i8, i8);

mod draw;

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
    kind: PieceKind,
    colour: Colour,
    has_moved: bool,
}


#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    to_move: Colour,
}

#[derive(Debug, Clone, Copy)]
pub enum BoardMode {
    Standard,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Move {
    from: Coordinate,
    to: Coordinate,
}

impl Move {
    fn new(from: Coordinate, to: Coordinate) -> Move {
        Move {from: from, to: to}
    }
}

fn get_lexrep(piece: &Option<Piece>) -> &str {
    match piece {
        Some(piece) => match (piece.kind, piece.colour) {
            (PieceKind::Pawn, Colour::White) => "[P]",
            (PieceKind::Pawn, Colour::Black) => "[p]",
        },
        None => "[ ]",
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..8 {
            write!(f, "{} ", row)?;

            for col in 0..8 {
                let symbol = get_lexrep(self.get_piece(row, col));

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
                has_moved: false,
            });
        }

        for col in 0..8 {
            squares[6][col] = Some(Piece {
                kind: PieceKind::Pawn,
                colour: Colour::White,
                has_moved: false,
            });
        }

        Self { squares: squares, to_move: Colour::White }
    }

    fn dispatch(&self, p: Piece, row: i8, col: i8) -> Vec<Move> {
        use PieceKind::*;
        match p.kind {
            Pawn => self.pawn_moves(p, row, col) 
        }
    }

    pub fn get_piece(&self, row: i8, col: i8) -> &Option<Piece> {
        if row > 7 || col > 7 {
            panic!("row or col exceeds 7 {} {}", row, col)
        }
        &self.squares[row as usize][col as usize]
    }

    pub fn get_moves(&self, row: i8, col: i8) -> Vec<Move> {
       match *self.get_piece(row, col) {
            Some(p) => self.dispatch(p, row, col),
            None => vec![],
       }
    }

    fn pawn_moves(&self, p: Piece, row: i8, col: i8) -> Vec<Move> {
        let dir = match p.colour {
            Black => 1,
            White => -1,
        };

        let mut moves = Vec::new();
        let origin = (row, col);
        
        
        let new_row: i8 = row + dir;

        if (0..8).contains(&new_row) && self.get_piece(new_row, col).is_none() {
            moves.push(Move::new(origin, (new_row, col)));
            if !p.has_moved && self.get_piece(new_row + dir, col).is_none() {
               moves.push(Move::new(origin, (new_row + dir, col))); 
            }
        }

        for dc in [-1, 1] {
            let new_col = col + dc;
            if (0..8).contains(&new_col) && (0..8).contains(&new_row) {
                let target = self.get_piece(new_row, new_col);
                if target.is_some() && target.unwrap().colour != p.colour {
                    moves.push(Move::new((new_row, col), (new_row, new_col)));   
                }
            }
        }

        return moves;
    }
}

fn main() {
    let board = Board::new();
    println!("{}", board);


    println!("{:?}", board.get_piece(6, 0));
    println!("{:?}", board.get_moves(6, 0));

    draw::main();
}