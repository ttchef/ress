use macroquad::prelude::*;

const WINDOW_SIZE: f32 = 800.0;
const GRID_SIZE: usize = 8;
const CELL_SIZE: f32 = WINDOW_SIZE / GRID_SIZE as f32;

const WHITE_SQUARE: Color = Color::from_hex(0xeaedd1);
const BLACK_SQUARE: Color = Color::from_hex(0x769557);

#[derive(Copy, Clone, Debug)]
enum Piece {
    WK,
    BK,
}

const PIECE_PATHS: [&str; 12] = [
    "assets/textures/pieces/wk.png",
    "assets/textures/pieces/wq.png",
    "assets/textures/pieces/wr.png",
    "assets/textures/pieces/wb.png",
    "assets/textures/pieces/wn.png",
    "assets/textures/pieces/wp.png",
    "assets/textures/pieces/bk.png",
    "assets/textures/pieces/bq.png",
    "assets/textures/pieces/br.png",
    "assets/textures/pieces/bb.png",
    "assets/textures/pieces/bn.png",
    "assets/textures/pieces/bp.png",
];

struct Board {
    textures: [Texture2D; PIECE_PATHS.len()],
    data: [[Option<Piece>; GRID_SIZE]; GRID_SIZE],
}

impl Board {
    async fn new() -> Board {
        let mut textures = std::array::from_fn(|_| Texture2D::empty());

        for i in 0..PIECE_PATHS.len() {
            textures[i] = load_texture(PIECE_PATHS[i]).await.unwrap();
        }

        Board {
            textures,
            data: [[None; GRID_SIZE]; GRID_SIZE],
        }
    }

    fn draw(&self) {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                draw_rectangle(
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    if (x + y) % 2 == 0 {
                        WHITE_SQUARE
                    } else {
                        BLACK_SQUARE
                    },
                );

                let piece = self.data[x][y];

                if let Some(piece) = piece {
                    draw_texture_ex(
                        &self.textures[piece as usize],
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                            ..Default::default()
                        },
                    );
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

    loop {
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let (x, y) = (
                mouse_pos.0 as usize / CELL_SIZE as usize,
                mouse_pos.1 as usize / CELL_SIZE as usize,
            );
            board.data[x][y] = Some(Piece::BK);
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let mouse_pos = mouse_position();
            let (x, y) = (
                mouse_pos.0 as usize / CELL_SIZE as usize,
                mouse_pos.1 as usize / CELL_SIZE as usize,
            );
            board.data[x][y] = Some(Piece::WK);
        }

        board.draw();
        next_frame().await;
    }
}
