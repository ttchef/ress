use macroquad::prelude::*;

const WINDOW_SIZE: f32 = 800.0;
const GRID_SIZE: f32 = WINDOW_SIZE / ress::SIZE as f32;

const LIGHT_SQUARE_COLOR: Color = Color::from_hex(0xeaedd1);
const DARK_SQUARE_COLOR: Color = Color::from_hex(0x769557);

fn window_conf() -> Conf {
    Conf {
        window_title: "Ress".to_owned(),
        window_width: WINDOW_SIZE as i32,
        window_height: WINDOW_SIZE as i32,
        window_resizable: false,
        sample_count: 4,
        ..Default::default()
    }
}

async fn textures_init() -> Vec<Texture2D> {
    const PATHS: [&str; 12] = [
        "assets/textures/pieces/wp.png",
        "assets/textures/pieces/wn.png",
        "assets/textures/pieces/wb.png",
        "assets/textures/pieces/wr.png",
        "assets/textures/pieces/wq.png",
        "assets/textures/pieces/wk.png",
        "assets/textures/pieces/bp.png",
        "assets/textures/pieces/bn.png",
        "assets/textures/pieces/bb.png",
        "assets/textures/pieces/br.png",
        "assets/textures/pieces/bq.png",
        "assets/textures/pieces/bk.png",
    ];

    let mut textures = Vec::with_capacity(12);

    for p in PATHS {
        textures.push(load_texture(p).await.unwrap());
    }
    textures
}

fn square_to_screen(rank: u8, file: u8, flipped: bool) -> (f32, f32) {
    let (file, rank) = if flipped {
        (ress::SIZE - 1 - file, rank)
    } else {
        (file, ress::SIZE - 1 - rank)
    };
    (file as f32 * GRID_SIZE, rank as f32 * GRID_SIZE)
}

fn screen_to_square(x: f32, y: f32, flipped: bool) -> Option<ress::Square> {
    if x < 0.0 || y < 0.0 {
        return None;
    }
    let file = (x / GRID_SIZE) as u8;
    let rank = (y / GRID_SIZE) as u8;
    if file >= ress::SIZE || rank >= ress::SIZE {
        return None;
    }
    let (rank, file) = if flipped {
        (rank, ress::SIZE - 1 - file)
    } else {
        (ress::SIZE - 1 - rank, file)
    };
    Some(ress::Square::new(rank, file))
}

#[macroquad::main(window_conf)]
async fn main() {
    let textures = textures_init().await;
    let mut active = None;
    let mut flipped = false;

    let engine = ress::Engine::new();

    loop {
        // let dt = get_frame_time() * 1000.0;
        // println!("DT: {dt}");

        if is_key_pressed(KeyCode::F) {
            flipped = !flipped;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if let Some(sq) = screen_to_square(mouse_pos.0, mouse_pos.1, flipped) {
                active = if active == Some(sq) { None } else { Some(sq) }
            }
        }

        clear_background(BLACK);

        for rank in 0..ress::SIZE {
            for file in 0..ress::SIZE {
                let (x, y) = square_to_screen(rank, file, flipped);

                draw_rectangle(
                    x,
                    y,
                    GRID_SIZE,
                    GRID_SIZE,
                    if (rank + file) % 2 == 1 {
                        LIGHT_SQUARE_COLOR
                    } else {
                        DARK_SQUARE_COLOR
                    },
                );

                if let Some(active) = active
                    && ress::Square::new(rank, file) == active
                {
                    draw_rectangle(x, y, GRID_SIZE, GRID_SIZE, GREEN.with_alpha(0.5));
                }

                if let Some((piece, side)) = engine.board.get(ress::Square::new(rank, file)) {
                    draw_texture_ex(
                        &textures[piece as usize + 6 * side as usize],
                        x,
                        y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(GRID_SIZE, GRID_SIZE)),
                            ..Default::default()
                        },
                    );
                }
            }
        }

        if let Some(active) = active
            && let Some((piece, side)) = engine.board.get(active)
        {
            let moves = engine.get_moves(piece, side, Some(active));
            for rank in 0..ress::SIZE {
                for file in 0..ress::SIZE {
                    if moves.contains(ress::Square::new(rank, file).into()) {
                        let (x, y) = square_to_screen(rank, file, flipped);
                        draw_circle(
                            x + GRID_SIZE / 2.0,
                            y + GRID_SIZE / 2.0,
                            GRID_SIZE / 4.0,
                            BLACK.with_alpha(0.5),
                        );
                    }
                }
            }
        }

        next_frame().await;
    }
}
