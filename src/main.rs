use macroquad::prelude::*;

const WINDOW_SIZE: f32 = 800.0;
const GRID_SIZE: usize = 8;
const CELL_SIZE: f32 = WINDOW_SIZE / GRID_SIZE as f32;

const WHITE_SQUARE: Color = Color::from_hex(0xeaedd1);
const BLACK_SQUARE: Color = Color::from_hex(0x769557);

#[derive(Copy, Clone, Debug)]
enum Piece {
    None,
    WK,
    BK,
}

struct Board {
    wk: Texture2D,
    bk: Texture2D,
    data: [[Piece; GRID_SIZE]; GRID_SIZE],
}

impl Board {
    async fn new() -> Board {
        Board {
            wk: load_texture("assets/textures/pieces/wk.png").await.unwrap(),
            bk: load_texture("assets/textures/pieces/bk.png").await.unwrap(),
            data: [[Piece::None; GRID_SIZE]; GRID_SIZE],
        }
    }

    fn draw(&self) {
        // Grid
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                draw_rectangle(
                    i as f32 * CELL_SIZE,
                    j as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    if (i + j) % 2 == 0 {
                        WHITE_SQUARE
                    } else {
                        BLACK_SQUARE
                    },
                );
            }
        }

        // Pieces
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                match self.data[x][y] {
                    Piece::BK => {
                        draw_texture_ex(
                            &self.bk,
                            x as f32 * CELL_SIZE,
                            y as f32 * CELL_SIZE,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                                ..Default::default()
                            },
                        );
                    }
                    Piece::WK => {
                        draw_texture_ex(
                            &self.wk,
                            x as f32 * CELL_SIZE,
                            y as f32 * CELL_SIZE,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                                ..Default::default()
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}

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
    let mut board = Board::new().await;
    board.data[0][0] = Piece::BK;

    loop {
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let (x, y) = (
                mouse_pos.0 as usize / CELL_SIZE as usize,
                mouse_pos.1 as usize / CELL_SIZE as usize,
            );
            board.data[x][y] = Piece::BK;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let mouse_pos = mouse_position();
            let (x, y) = (
                mouse_pos.0 as usize / CELL_SIZE as usize,
                mouse_pos.1 as usize / CELL_SIZE as usize,
            );
            board.data[x][y] = Piece::WK;
        }

        board.draw();
        next_frame().await;
    }
}
