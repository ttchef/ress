mod board;
mod piece;
mod renderer;

use crate::board::{Board, CELL_SIZE};
use crate::renderer::Renderer;

use macroquad::prelude::*;

const WINDOW_SIZE: f32 = 800.0;

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
async fn main() {
    let renderer = Renderer::new().await;
    let mut board = Board::standard();

    loop {
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            board.clicked(mouse_pos);
        }

        board.draw(&renderer);

        next_frame().await;
    }
}
