use crate::piece::{Kind, MoveVector, Piece, Side};
use crate::renderer::Renderer;

use macroquad::prelude::*;

pub const BOARD_SIZE: usize = 8;
// TODO: WINDOW_SIZE instead of 800
pub const CELL_SIZE: f32 = 800.0 / BOARD_SIZE as f32;

const WHITE_SQUARE: Color = Color::from_hex(0xeaedd1);
const BLACK_SQUARE: Color = Color::from_hex(0x769557);

#[derive(PartialEq)]
pub struct Square {
    pub row: usize,
    pub col: usize,
}

impl Square {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl std::ops::Add<MoveVector> for Square {
    type Output = Square;
    fn add(self, rhs: MoveVector) -> Self::Output {
        Square {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

// [0][0] is black back rank
// [row][col] indexing
pub struct Board {
    data: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    active: Option<Square>,
}

impl Board {
    pub fn standard() -> Self {
        let mut data = [[None; BOARD_SIZE]; BOARD_SIZE];

        let back_rank = [
            Kind::Rook,
            Kind::Knight,
            Kind::Bishop,
            Kind::Queen,
            Kind::King,
            Kind::Bishop,
            Kind::Knight,
            Kind::Rook,
        ];

        for col in 0..BOARD_SIZE {
            data[0][col] = Some(Piece::new(back_rank[col], Side::Black));
            data[1][col] = Some(Piece::new(Kind::Pawn, Side::Black));
            data[7][col] = Some(Piece::new(back_rank[col], Side::White));
            data[6][col] = Some(Piece::new(Kind::Pawn, Side::White));
        }

        Self { data, active: None }
    }

    pub fn clicked(&mut self, pos: (f32, f32)) {
        let (col, row) = (
            pos.0 as usize / CELL_SIZE as usize,
            pos.1 as usize / CELL_SIZE as usize,
        );

        let new = Some(Square::new(row, col));
        if self.active == new {
            self.active = None;
        } else {
            self.active = new;
        }
    }

    fn show_options(&self, piece: Piece) {
        let moves = piece.get_move_vectors();

        for move in moves {
            
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let background_color = if (row + col) % 2 == 0 {
                    WHITE_SQUARE
                } else {
                    BLACK_SQUARE
                };

                // Draw Background
                draw_rectangle(
                    col as f32 * CELL_SIZE,
                    row as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    background_color,
                );

                if let Some(piece) = self.data[row][col] {
                    let texture = &renderer.piece_textures[piece.index()];
                    draw_texture_ex(
                        texture,
                        col as f32 * CELL_SIZE,
                        row as f32 * CELL_SIZE,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                            ..Default::default()
                        },
                    );
                }
            }
        }

        if let Some(active) = &self.active {
            let mut color = YELLOW;
            color.a *= 0.2;

            draw_rectangle(
                active.col as f32 * CELL_SIZE,
                active.row as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
            );

            if let Some(piece) = self.data[active.row][active.col] {
                self.show_options(piece);
            }
        }
    }
}
