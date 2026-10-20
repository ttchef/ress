use macroquad::prelude::*;

const WINDOW_SIZE: f32 = 800.0;

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

        next_frame().await;
    }
}
