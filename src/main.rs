use macroquad::prelude::*;

const WINDOW_SIZE: f32 = 800.0;
const GRID_COUNT: usize = 8;
const GRID_SIZE: f32 = WINDOW_SIZE / GRID_COUNT as f32;

const LIGHT_SQUARE_COLOR: Color = Color::from_hex(0xeaedd1);
const DARK_SQUARE_COLOR: Color = Color::from_hex(0x769557);

fn window_conf() -> Conf {
    Conf {
        window_title: "Ress".to_owned(),
        window_width: WINDOW_SIZE as i32,
        window_height: WINDOW_SIZE as i32,
        window_resizable: false,
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

#[macroquad::main(window_conf)]
async fn main() {
    let textures = textures_init().await;
    let mut active = None;

    let engine = ress::Engine::new();

    loop {
        // let dt = get_frame_time() * 1000.0;
        // println!("DT: {dt}");

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let file = mouse_pos.0 / GRID_SIZE as f32;
            let rank = mouse_pos.1 / GRID_SIZE as f32;
            if let Some(a) = active
                && a == ress::Square::new(rank as u8, file as u8)
            {
                active = None;
            } else {
                active = Some(ress::Square::new(rank as u8, file as u8));
            }
        }

        clear_background(BLACK);

        for rank in 0..GRID_COUNT {
            for file in 0..GRID_COUNT {
                let x = file as f32 * GRID_SIZE;
                let y = rank as f32 * GRID_SIZE;

                draw_rectangle(
                    x,
                    y,
                    GRID_SIZE,
                    GRID_SIZE,
                    if (rank + file) % 2 == 0 {
                        LIGHT_SQUARE_COLOR
                    } else {
                        DARK_SQUARE_COLOR
                    },
                );

                if let Some(active) = active
                    && ress::Square::new(rank as u8, file as u8) == active
                {
                    draw_rectangle(x, y, GRID_SIZE, GRID_SIZE, GREEN.with_alpha(0.5));
                }

                if let Some((piece, side)) =
                    engine.board.get(ress::Square::new(rank as u8, file as u8))
                {
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

        next_frame().await;
    }
}
