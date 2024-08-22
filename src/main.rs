use conways_game_of_life::ui::run_ui;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    run_ui().await;
}
