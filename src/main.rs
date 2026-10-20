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

#[macroquad::main(window_conf)]
async fn main() {
    let engine = ress::Engine::new();

    loop {
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

                // engine.board
            }
        }

        next_frame().await;
    }
}
