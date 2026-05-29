use std::sync::{
    Arc, Mutex, MutexGuard,
    atomic::{AtomicBool, Ordering},
};

use crate::{
    Board,
    board::{GameState, reset, square_iter},
    input::{self, InputState},
    moves::{Colour, Coordinate, Piece},
};
use raylib::prelude::*;

pub const TILE_SIZE: i32 = 80;

pub fn chess_window(
    board: Arc<Mutex<Board>>,
    ready: Arc<AtomicBool>,
    input: Arc<Mutex<InputState>>,
) {
    let (mut rl, thread) = raylib::init()
        .size(TILE_SIZE * 8, TILE_SIZE * 8)
        .title("chess3")
        .build();

    let spritesheet = load_texture(
        &mut rl,
        &thread,
        include_bytes!(r"..\assets\spritesheet.png"),
    );

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
        } else if rl.is_key_pressed(KeyboardKey::KEY_R) {
            reset(&board, &input);
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

        if board.gamestate != GameState::Playing {
            draw_gamestate_window(&mut d, board.gamestate);
        }

        let inp = input.lock().unwrap();
        if inp.selected.is_some() {
            draw_move_indacators(&mut d, inp.legal_moves.iter().map(|mv| mv.to).collect());
        }
    }
}

fn load_texture(rl: &mut RaylibHandle, thread: &RaylibThread, data: &[u8]) -> Texture2D {
    let img = Image::load_image_from_mem(".png", data).unwrap();
    return rl.load_texture_from_image(thread, &img).unwrap();
}

fn draw_board(d: &mut RaylibDrawHandle, highlighted: &Vec<Coordinate>) {
    let light: Color = Color::new(237, 214, 176, 255);
    let dark: Color = Color::new(184, 135, 98, 255);
    let light_selected: Color = Color::new(247, 235, 114, 255);
    let dark_selected: Color = Color::new(220, 196, 75, 255);
    for (row, col) in square_iter() {
        let mut colour: Color = if (row + col) % 2 == 0 { light } else { dark };
        if highlighted.contains(&(row, col)) {
            colour = if (row + col) % 2 == 0 {
                light_selected
            } else {
                dark_selected
            }
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

fn draw_pieces(
    d: &mut RaylibDrawHandle,
    board: &Board,
    spritesheet: &Texture2D,
    sprite_w: f32,
    sprite_h: f32,
) {
    for (piece, row, col) in board.as_iter() {
        if let Some(p) = piece {
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

fn draw_move_indacators(d: &mut RaylibDrawHandle, squares: Vec<Coordinate>) {
    for (row, col) in squares {
        let x = col as i32 * TILE_SIZE;
        let y = row as i32 * TILE_SIZE;
        d.draw_circle(
            x + TILE_SIZE / 2,
            y + TILE_SIZE / 2,
            TILE_SIZE as f32 / 6.0,
            Color::new(0, 0, 0, 100),
        );
    }
}

fn piece_rect(piece: &Piece, width: f32, height: f32) -> Rectangle {
    use crate::moves::Colour::*;
    use crate::moves::PieceKind::*;
    let col: i32 = match piece.kind {
        King => 0,
        Knight => 2,
        Pawn => 3,
        Queen => 4,
        Rook => 5,
        Bishop => 1,
    };

    let row = match piece.colour {
        Black => 0,
        White => 1,
    };

    Rectangle {
        x: col as f32 * width,
        y: row as f32 * height,
        width,
        height,
    }
}

fn draw_gamestate_window(d: &mut RaylibDrawHandle, gamestate: GameState) {
    use GameState::*;

    let (title, subtitle) = match gamestate {
        Checkmate(Colour::White) => ("White wins!", "Black has been checkmated"),
        Checkmate(Colour::Black) => ("Black wins!", "White has been checkmated"),
        Stalemate => ("Draw", "Stalemate — no legal moves"),
        InsufficientMat => ("Draw", "Insufficient material"),
        FiftyMove => ("Draw", "Fifty-move rule"),
        Playing => return,
    };

    let box_x = 1 * TILE_SIZE;
    let box_y = 3 * TILE_SIZE;
    let box_w = 6 * TILE_SIZE;
    let box_h = 3 * TILE_SIZE;

    d.draw_rectangle(box_x + 8, box_y + 8, box_w, box_h, Color::new(0, 0, 0, 90));

    d.draw_rectangle(box_x, box_y, box_w, box_h, Color::new(245, 245, 245, 255));

    d.draw_rectangle_lines_ex(
        Rectangle::new(box_x as f32, box_y as f32, box_w as f32, box_h as f32),
        3.0,
        Color::BLACK,
    );

    let title_size = 34;
    let sub_size = 20;
    let hint_size = 18;

    let spacing = 10;

    let title_w = d.measure_text(title, title_size);
    let sub_w = d.measure_text(subtitle, sub_size);

    let hint = "Press R to restart";
    let hint_w = d.measure_text(hint, hint_size);

    let total_h = title_size + sub_size + hint_size + spacing * 2;
    let start_y = box_y + (box_h - total_h) / 2;

    let title_x = box_x + (box_w - title_w) / 2;
    let title_y = start_y;

    let sub_x = box_x + (box_w - sub_w) / 2;
    let sub_y = title_y + title_size + spacing;

    let hint_x = box_x + (box_w - hint_w) / 2;
    let hint_y = sub_y + sub_size + spacing + 6;

    d.draw_text(title, title_x, title_y, title_size, Color::BLACK);

    d.draw_text(
        subtitle,
        sub_x,
        sub_y,
        sub_size,
        Color::new(60, 60, 60, 255),
    );

    d.draw_text(
        hint,
        hint_x,
        hint_y,
        hint_size,
        Color::new(100, 100, 100, 255),
    );
}
