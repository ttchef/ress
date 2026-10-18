mod board;
mod error;
mod piece;
mod renderer;

use crate::board::{Board, Square};
use crate::renderer::{Renderer, WINDOW_SIZE};

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess Engine".to_owned(),
        window_width: WINDOW_SIZE as i32,
        window_height: WINDOW_SIZE as i32,
        window_resizable: false,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let renderer = Renderer::new().await;
    let mut board = Board::standard();

    loop {
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (row, col) = Renderer::pos_to_cell(mouse_position());
            board.clicked(Square::new(row, col)?);
        }

        renderer.draw_board(&board)?;
        next_frame().await;
    }
}
