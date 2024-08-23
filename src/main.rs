use conways_game_of_life::game::Game;
use conways_game_of_life::ui::run_ui;

const DEFAULT_SEED: [(i32, i32); 8] = [
    (14, 13),
    (14, 14),
    (14, 15),
    (15, 13),
    (15, 15),
    (16, 13),
    (16, 14),
    (16, 15),
];

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let game = Game::new_with_seed(DEFAULT_SEED.to_vec());

    run_ui(game).await;
}
