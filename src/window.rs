use std::sync::{Arc, Mutex, MutexGuard, atomic::{AtomicBool, Ordering}};

use crate::{Board, input::InputState, moves::{Coordinate, Piece}, input};
use raylib::prelude::*;

pub const TILE_SIZE: i32 = 80;

pub fn chess_window(board: Arc<Mutex<Board>>, ready: Arc<AtomicBool>, input: Arc<Mutex<InputState>>) {
    let (mut rl, thread) = raylib::init()
        .size(TILE_SIZE * 8, TILE_SIZE * 8)
        .title("chess3")
        .build();

    let spritesheet = load_texture(&mut rl, &thread, include_bytes!(r"..\assets\spritesheet.png"));

    // spritesheet is organized as 
    // kbnpqr
    // KBNPQR
    let sprite_w = spritesheet.width as f32 / 6.0;
    let sprite_h = spritesheet.height as f32 / 2.0;

    let mut highlighted = Vec::new();

    ready.store(true, Ordering::SeqCst);

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            input::handle_click(&board, &input, &rl);
        }

        highlighted.clear();
        let selected = input.lock().unwrap().selected;
        if let Some((row, col)) = selected {
            highlighted.push((row, col));
        }

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGRAY);
        draw_board(&mut d, &highlighted);

        let board: MutexGuard<'_, Board> = board.lock().unwrap();
        draw_pieces(&mut d, &board, &spritesheet, sprite_w, sprite_h);

        if let Some((row, col)) = input.lock().unwrap().selected {
            draw_move_indacators(&mut d, board.get_moves(row, col).iter().map(|mv| mv.to).collect());
        }
    }
}

fn load_texture(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    data: &[u8],
) -> Texture2D {
    let img = Image::load_image_from_mem(".png", data).unwrap();
    return rl.load_texture_from_image(thread, &img).unwrap();
}

fn draw_board(d: &mut RaylibDrawHandle, highlighted: &Vec<Coordinate>) {
    let light: Color = Color::new(237, 214, 176, 255);
    let dark: Color = Color::new(184, 135, 98, 255);
    let light_selected: Color = Color::new(247, 235, 114, 255);
    let dark_selected: Color = Color::new(220, 196, 75, 255);
    for row in 0..8 {
        for col in 0..8 {
            let mut colour: Color = if (row + col) % 2 == 0 { light } else { dark };
            if highlighted.contains(&(row, col)) {
                colour = if (row + col) % 2 == 0 { light_selected } else { dark_selected }
            }

            d.draw_rectangle(
                col as i32 * TILE_SIZE,
                row as i32 * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
                colour,
            );
        }
    }
}

fn draw_pieces(d: &mut RaylibDrawHandle, board: &Board, spritesheet: &Texture2D, sprite_w: f32, sprite_h: f32) {
    for row in 0..8 {
        for col in 0..8 {
            let piece: &Option<Piece> = board.get_piece(row, col);

            if let Some(p) = *piece {
                let src = piece_rect(&p, sprite_w, sprite_h);
                let dst = Rectangle {
                    x: col as f32 * TILE_SIZE as f32,
                    y: row as f32 * TILE_SIZE as f32,
                    width: TILE_SIZE as f32,
                    height: TILE_SIZE as f32,
                };
                d.draw_texture_pro(spritesheet, src, dst, Vector2::zero(), 0.0, Color::WHITE);
            }
        }
    }
}

fn draw_move_indacators(d: &mut RaylibDrawHandle, squares: Vec<Coordinate>) {
    for (row, col) in squares {
        let x = col as i32 * TILE_SIZE;
        let y = row as i32 * TILE_SIZE;
        d.draw_circle(x + TILE_SIZE / 2, y + TILE_SIZE / 2, TILE_SIZE as f32 / 6.0, Color::new(0, 0, 0, 100));
    }
}

fn piece_rect(piece: &Piece, width: f32, height: f32) -> Rectangle {
    use crate::moves::PieceKind::*;
    use crate::moves::Colour::*;
    let col: i32 = match piece.kind {
        King   => 0,
        Knight => 2,
        Pawn   => 3,
        Queen  => 4,
        Rook   => 5, 
        Bishop => 1,
    };

    let row = match piece.colour {
        Black => 0,
        White => 1,
    };

    Rectangle { x: col as f32 * width, y: row as f32 * height, width, height }
}
