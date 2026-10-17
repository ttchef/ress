use macroquad::prelude::*;

const WINDOW_SIZE: f32 = 800.0;
const GRID_SIZE: u32 = 8;
const CELL_SIZE: f32 = WINDOW_SIZE / GRID_SIZE as f32;

const WHITE_SQUARE: Color = Color::from_hex(0xeaedd1);
const BLACK_SQUARE: Color = Color::from_hex(0x769557);

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess Engine".to_owned(),
        window_width: WINDOW_SIZE as i32,
        window_height: WINDOW_SIZE as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

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
        next_frame().await;
    }
}
