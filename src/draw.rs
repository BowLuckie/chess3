use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};

use crate::{Board, moves::get_lexrep};
use raylib::prelude::*;

const TILE_SIZE: i32 = 80;

pub fn draw(board: Arc<Mutex<Board>>, ready: Arc<AtomicBool>) {
    let (mut r1, thread) = raylib::init()
        .size(TILE_SIZE * 8, TILE_SIZE * 8)
        .title("chess3")
        .build();

    ready.store(true, Ordering::SeqCst);

    while !r1.window_should_close() {
        let mut d = r1.begin_drawing(&thread);

        d.clear_background(Color::DARKGRAY);
        draw_board(&mut d);

        let board = board.lock().unwrap();
        draw_pieces(&mut d, &board);
    }
}

fn draw_board(d: &mut RaylibDrawHandle) {
    let light = Color::new(240, 217, 181, 255);
    let dark = Color::new(181, 136, 99, 255);
    for row in 0..8 {
        for col in 0..8 {
            let colour = if (row + col) % 2 == 0 { light } else { dark };

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

fn draw_pieces(d: &mut RaylibDrawHandle, board: &Board) {
    for row in 0..8 {
        for col in 0..8 {
            let piece = board.get_piece(row, col);

            if let Some(p) = *piece {
                let opt = Some(p);
                let symbol = &get_lexrep(&opt);

                let font_size = 30;

                let text_width = d.measure_text(symbol, font_size);
                let text_height = font_size;

                let x = col as i32 * TILE_SIZE + (TILE_SIZE - text_width) / 2;
                let y = row as i32 * TILE_SIZE + (TILE_SIZE - text_height) / 2;

                d.draw_text(symbol, x, y, font_size, Color::BLACK);
            }
        }
    }
}
