use crate::game::Game;

use macroquad::prelude::*;

use std::{thread::sleep, time::Duration};

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

async fn display_ui(
    game: &mut Game,
    game_running: &mut bool,
    game_paused: &mut bool,
    edit_mode: &mut bool,
    generations_passed: &mut i32,
) {
    let (offset_x, offset_y, sq_size) = render_screen(game, generations_passed);

    if *game_paused {
        if *edit_mode {
            run_edit_mode(game, &offset_x, &offset_y, &sq_size);
        } else {
            run_paused_mode();
        }
    } else {
        run_playing_mode(game, generations_passed);
    }

    handle_commands(game_running, game_paused, edit_mode);

    next_frame().await;
}

fn render_screen(game: &Game, generations_passed: &i32) -> (f32, f32, f32) {
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

    (offset_x, offset_y, sq_size)
}

fn run_edit_mode(game: &mut Game, offset_x: &f32, offset_y: &f32, sq_size: &f32) {
    show_edit_hud();

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

fn show_edit_hud() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    draw_text("Edit Mode", screen_width * TITLE_POS_X, screen_height * TITLE_POS_Y, TITLE_SIZE, BLACK);
    draw_text("Click on a live cell to kill it", screen_width * TEXT_POS_X, screen_height * TEXT_POS_Y - 15., TEXT_SIZE, BLACK);
    draw_text(
        "Click on a dead cell to make it alive",
        screen_width * TEXT_POS_X, screen_height * TEXT_POS_Y,
        TEXT_SIZE,
        BLACK,
    );
    draw_text("Press E to exit Edit Mode", screen_width * TEXT_POS_X, screen_height * TEXT_POS_Y + 15., TEXT_SIZE, BLACK);
}

fn run_paused_mode() {
    show_pause_hud();
}

fn show_pause_hud() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    draw_text("Game Paused", screen_width * TITLE_POS_X, screen_height * TITLE_POS_Y, TITLE_SIZE, BLACK);
    draw_text("Press E to enter Edit Mode", screen_width * TEXT_POS_X, screen_height * TEXT_POS_Y, TEXT_SIZE, BLACK);
    draw_text("Press X to exit the game", screen_width * TEXT_POS_X, screen_height * TEXT_POS_Y + 15., TEXT_SIZE, BLACK);
}

fn run_playing_mode(game: &mut Game, generations_passed: &mut i32) {
    sleep(Duration::from_secs(1));
    game.update_game();

    show_playing_hud();
    *generations_passed += 1;
}

fn show_playing_hud() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    draw_text("Conway's Game of Life", screen_width * TITLE_POS_X, screen_height * TITLE_POS_Y, TITLE_SIZE, BLACK);
    draw_text("Press SPACE to pause", screen_width * TEXT_POS_X, screen_height * TEXT_POS_Y, TEXT_SIZE, BLACK);
    draw_text("Press X to exit the game", screen_width * TEXT_POS_X, screen_height * TEXT_POS_Y + 15., TEXT_SIZE, BLACK);
}

fn handle_commands(game_running: &mut bool, game_paused: &mut bool, edit_mode: &mut bool) {
    if is_key_pressed(KeyCode::X) {
        *game_running = !*game_running;
    }

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
}

// Runs the UI by creating a new instance of the Game of Life
pub async fn run_ui() {
    let mut game = Game::new_with_seed(DEFAULT_SEED.to_vec());

    let mut game_running = true;
    let mut game_paused = false;
    let mut edit_mode = false;
    let mut generations_passed = 0;

    while game_running {
        display_ui(
            &mut game,
            &mut game_running,
            &mut game_paused,
            &mut edit_mode,
            &mut generations_passed,
        )
        .await;
    }
}
