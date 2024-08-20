use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(WHITE);

        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, BLACK);

        next_frame().await
    }
}