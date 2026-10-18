use crate::{board, board::Board, board::Square, error::RessErr, piece::Piece};

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

    pub fn pos_to_cell(pos: (f32, f32)) -> (usize, usize) {
        let col = pos.0 / BOARD_CELL;
        let row = pos.1 / BOARD_CELL;
        (row as usize, col as usize)
    }

    pub fn draw_board(&self, board: &Board) -> Result<(), RessErr> {
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

                if Some(Square::new(row, col)?) == board.active_square() {
                    let mut color = GREEN;
                    color.a *= 0.3;
                    draw_rectangle(
                        col as f32 * BOARD_CELL,
                        row as f32 * BOARD_CELL,
                        BOARD_CELL,
                        BOARD_CELL,
                        color,
                    );
                }

                if let Some(piece) = board.get(Square::new(row, col)?) {
                    let tex = &self.piece_textures[piece.index()];
                    draw_texture_ex(
                        tex,
                        col as f32 * BOARD_CELL,
                        row as f32 * BOARD_CELL,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BOARD_CELL, BOARD_CELL)),
                            ..Default::default()
                        },
                    );
                }
            }
        }

        if let Some(active) = board.active_square()
            && let Some(piece) = board.get(active)
        {
            let moves = board.get_moves(active, piece);
            // TODO: Optimize later
            for row in 0..board::SIZE {
                for col in 0..board::SIZE {
                    if moves.get(Square::new(row, col)?) {
                        draw_circle(
                            col as f32 * BOARD_CELL + BOARD_CELL / 2.0,
                            row as f32 * BOARD_CELL + BOARD_CELL / 2.0,
                            BOARD_CELL / 3.0,
                            RED,
                        );
                    }
                }
            }
        }
        Ok(())
    }
}
