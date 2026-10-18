mod board;
mod piece;
mod renderer;

use crate::board::Board;
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
async fn main() {
    let renderer = Renderer::new().await;
    let mut board = Board::standard();

    loop {
        clear_background(BLACK);

        renderer.draw_board(&board);

        next_frame().await;
    }
}
