use crate::{board, board::Board, piece::Piece};

use macroquad::prelude::*;

pub const WINDOW_SIZE: f32 = 800.0;

const BOARD_CELL: f32 = WINDOW_SIZE / board::SIZE as f32;
const WHITE_COLOR: Color = Color::from_hex(0xeaedd1);
const BLACK_COLOR: Color = Color::from_hex(0x769557);

pub struct Renderer {
    pub piece_textures: Vec<Texture2D>,
}

impl Renderer {
    pub async fn new() -> Self {
        let mut piece_textures = Vec::new();

        for i in 0..12 {
            let path = Piece::from_index(i).asset_path();
            piece_textures.push(
                load_texture(path.as_str())
                    .await
                    .expect("Piece texture paths are valid"),
            );
        }

        Self { piece_textures }
    }

    pub fn draw_board(&self, board: &Board) {
        for row in 0..board::SIZE {
            for col in 0..board::SIZE {
                draw_rectangle(
                    col as f32 * BOARD_CELL,
                    row as f32 * BOARD_CELL,
                    BOARD_CELL,
                    BOARD_CELL,
                    if (row + col) % 2 == 0 {
                        WHITE_COLOR
                    } else {
                        BLACK_COLOR
                    },
                );
            }
        }
    }
}
