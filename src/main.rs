use macroquad::prelude::*;

#[macroquad::main("Chess Engine")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_rectangle(0., 0., 10., 10., RED);

        next_frame().await;
    }
}
