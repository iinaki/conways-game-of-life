use crate::game::Game;

use ::rand::Rng;
use macroquad::prelude::*;

use std::time::Instant;

const SQUARES: i16 = 32;
const TEXT_SIZE: f32 = 25.;
const TEXT_POS_X: f32 = 0.2;
const TEXT_POS_Y: f32 = 0.9;
const TITLE_SIZE: f32 = 40.;
const TITLE_POS_X: f32 = 0.3;
const TITLE_POS_Y: f32 = 0.05;
const GENERATIONS_POS_X: f32 = 0.1;
const GENERATIONS_POS_Y: f32 = 0.1;
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

/// This function displays the user interface for the Game of Life, including rendering the game screen,
/// handling user input, and switching between different modes (playing, paused, edit).
///
/// # Parameters
///
/// - `game`: The current game state.
/// - `game_running`: A flag indicating whether the game is running.
/// - `game_paused`: A flag indicating whether the game is paused.
/// - `edit_mode`: A flag indicating whether the game is in edit mode.
/// - `generations_passed`: The number of generations that have passed in the game.
///
async fn display_ui(
    game: &mut Game,
    game_paused: &mut bool,
    edit_mode: &mut bool,
    generations_passed: &mut i32,
    acc: &mut f32,
    fps: &mut f32,
    prev_t: &mut Instant,
) {
    let elapsed = prev_t.elapsed().as_secs_f32();
    *prev_t = Instant::now();
    *acc += elapsed;

    handle_commands(game, game_paused, edit_mode, fps);

    render_screen(game, generations_passed, game_paused, edit_mode, fps, acc);

    next_frame().await;
}

/// Renders the game screen with grid lines and the live cells.
///
/// # Parameters
///
/// - `game`: The current game state.
/// - `generations_passed`: The number of generations that have passed in the game.
///
fn render_screen(
    game: &mut Game,
    generations_passed: &mut i32,
    game_paused: &mut bool,
    edit_mode: &mut bool,
    fps: &f32,
    acc: &mut f32,
) {
    clear_background(LIGHTGRAY);
    let screen_width = screen_width();
    let screen_height = screen_height();

    let game_size = screen_width.min(screen_height);
    let offset_x = (screen_width - game_size) / 2. + 10.;
    let offset_y = (screen_height - game_size) / 2. + 10.;
    let sq_size = (screen_height - offset_y * 2.) / SQUARES as f32;

    draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

    for i in 1..SQUARES {
        draw_line(
            offset_x,
            offset_y + sq_size * i as f32,
            screen_width - offset_x,
            offset_y + sq_size * i as f32,
            2.,
            LIGHTGRAY,
        );
    }

    for i in 1..SQUARES {
        draw_line(
            offset_x + sq_size * i as f32,
            offset_y,
            offset_x + sq_size * i as f32,
            screen_height - offset_y,
            2.,
            LIGHTGRAY,
        );
    }

    for (x, y) in game.live_cells() {
        draw_rectangle(
            offset_x + x as f32 * sq_size,
            offset_y + y as f32 * sq_size,
            sq_size,
            sq_size,
            GRAY,
        );
    }

    let text = format!("Generations: {}", generations_passed);
    draw_text(
        &text,
        screen_width * GENERATIONS_POS_X,
        screen_height * GENERATIONS_POS_Y,
        TEXT_SIZE,
        BLACK,
    );

    let text = format!("FPS: {}", 1. / fps);
    draw_text(
        &text,
        screen_width * GENERATIONS_POS_X,
        screen_height * GENERATIONS_POS_Y + 15.,
        TEXT_SIZE,
        BLACK,
    );

    render_mode(
        game,
        (offset_x, offset_y, sq_size),
        game_paused,
        edit_mode,
        generations_passed,
        fps,
        acc,
    );
}

fn render_mode(
    game: &mut Game,
    screen_params: (f32, f32, f32),
    game_paused: &mut bool,
    edit_mode: &mut bool,
    generations_passed: &mut i32,
    fps: &f32,
    acc: &mut f32,
) {
    if *game_paused {
        if *edit_mode {
            run_edit_mode(game, screen_params);
        } else {
            run_paused_mode();
        }
    } else {
        run_playing_mode(game, generations_passed, fps, acc);
    }
}

/// Handles the edit mode where users can add or remove cells.
fn run_edit_mode(game: &mut Game, screen_params: (f32, f32, f32)) {
    show_edit_hud();
    let (offset_x, offset_y, sq_size) = screen_params;

    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();

        let grid_x = ((mouse_x - offset_x) / sq_size).floor() as i32;
        let grid_y = ((mouse_y - offset_y) / sq_size).floor() as i32;

        if grid_x >= 0 && grid_x < SQUARES as i32 && grid_y >= 0 && grid_y < SQUARES as i32 {
            if game.is_cell_alive(grid_x, grid_y) {
                game.remove_cell(grid_x, grid_y);
            } else {
                game.add_cell(grid_x, grid_y);
            }
        }
    }
}

/// Displays the HUD for edit mode, showing instructions to the user.
fn show_edit_hud() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    draw_text(
        "Edit Mode",
        screen_width * TITLE_POS_X,
        screen_height * TITLE_POS_Y,
        TITLE_SIZE,
        BLACK,
    );
    draw_text(
        "Click on a live cell to kill it",
        screen_width * TEXT_POS_X,
        screen_height * TEXT_POS_Y - 15.,
        TEXT_SIZE,
        BLACK,
    );
    draw_text(
        "Click on a dead cell to make it alive",
        screen_width * TEXT_POS_X,
        screen_height * TEXT_POS_Y,
        TEXT_SIZE,
        BLACK,
    );
    draw_text(
        "Press E to exit Edit Mode",
        screen_width * TEXT_POS_X,
        screen_height * TEXT_POS_Y + 15.,
        TEXT_SIZE,
        BLACK,
    );
}

/// Runs the game in paused mode, displaying the pause HUD.
fn run_paused_mode() {
    show_pause_hud();
}

/// Displays the HUD for paused mode, showing instructions for the user.    
fn show_pause_hud() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    draw_text(
        "Game Paused",
        screen_width * TITLE_POS_X,
        screen_height * TITLE_POS_Y,
        TITLE_SIZE,
        BLACK,
    );
    draw_text(
        "Press E to enter Edit Mode",
        screen_width * TEXT_POS_X,
        screen_height * TEXT_POS_Y,
        TEXT_SIZE,
        BLACK,
    );
    draw_text(
        "Press X to exit the game",
        screen_width * TEXT_POS_X,
        screen_height * TEXT_POS_Y + 15.,
        TEXT_SIZE,
        BLACK,
    );
    draw_text(
        "Press R to restart the game with a random seed",
        screen_width * TEXT_POS_X,
        screen_height * TEXT_POS_Y + 30.,
        TEXT_SIZE,
        BLACK,
    );
}

/// Runs the game in playing mode, where time passes and the game is updated.
fn run_playing_mode(game: &mut Game, generations_passed: &mut i32, fps: &f32, acc: &mut f32) {
    if *acc >= *fps {
        game.update_game();
        *generations_passed += 1;
        *acc -= fps;
    }

    show_playing_hud();
}

/// Displays the HUD for playing mode, showing instructions for the user.
fn show_playing_hud() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    draw_text(
        "Conway's Game of Life",
        screen_width * TITLE_POS_X,
        screen_height * TITLE_POS_Y,
        TITLE_SIZE,
        BLACK,
    );
    draw_text(
        "Press SPACE to pause",
        screen_width * TEXT_POS_X,
        screen_height * TEXT_POS_Y,
        TEXT_SIZE,
        BLACK,
    );
    draw_text(
        "Press X to exit the game",
        screen_width * TEXT_POS_X,
        screen_height * TEXT_POS_Y + 15.,
        TEXT_SIZE,
        BLACK,
    );
    draw_text(
        "Press R to restart the game with a random seed",
        screen_width * TEXT_POS_X,
        screen_height * TEXT_POS_Y + 30.,
        TEXT_SIZE,
        BLACK,
    );
    draw_text(
        "Use top and down arrows to change the speed of the game",
        screen_width * TEXT_POS_X,
        screen_height * TEXT_POS_Y + 45.,
        TEXT_SIZE,
        BLACK,
    );
}

/// Handles users commands.
fn handle_commands(game: &mut Game, game_paused: &mut bool, edit_mode: &mut bool, fps: &mut f32) {
    if is_key_pressed(KeyCode::Space) {
        *game_paused = !*game_paused;
    }

    if is_key_pressed(KeyCode::E) {
        if *game_paused {
            *edit_mode = !*edit_mode;
        } else {
            *game_paused = !*game_paused;
            *edit_mode = !*edit_mode;
        }
    }

    if is_key_pressed(KeyCode::R) {
        restart_game_with_random_seed(game);
    }

    if is_key_pressed(KeyCode::Up) {
        *fps = (*fps * 0.9).max(1.0 / 144.0);
    }

    if is_key_pressed(KeyCode::Down) {
        *fps = (*fps * 1.1).min(1.0);
    }
}

/// Restarts the game with a random seed. The amount of cells and their positions are randomly generated.
fn restart_game_with_random_seed(game: &mut Game) {
    let mut rng = ::rand::thread_rng();

    let num_cells = rng.gen_range(1..=(SQUARES * SQUARES) as usize);

    let mut seed = Vec::with_capacity(num_cells);

    for _ in 0..num_cells {
        let x = rng.gen_range(0..SQUARES) as i32;
        let y = rng.gen_range(0..SQUARES) as i32;
        seed.push((x, y));
    }

    *game = Game::new_with_seed(seed);
}

/// Runs the user interface of the Game of Life.
///
/// Creates a new instance of the game with a default seed
/// and manages the game loop. Handles user input and updates the game state
/// based on user interactions.
///
pub async fn run_ui() {
    let mut game = Game::new_with_seed(DEFAULT_SEED.to_vec());

    let mut game_paused = false;
    let mut edit_mode = false;
    let mut generations_passed = 0;

    let mut fps: f32 = 1.0 / 5.0;
    let mut acc = 0.0_f32;
    let mut prev_t = Instant::now();

    // let current_time = Instant::now();
    // println!("Current time: {:?}", current_time.as_secs_f64());

    while !is_key_pressed(KeyCode::X) {
        display_ui(
            &mut game,
            &mut game_paused,
            &mut edit_mode,
            &mut generations_passed,
            &mut acc,
            &mut fps,
            &mut prev_t,
        )
        .await;
    }
}
