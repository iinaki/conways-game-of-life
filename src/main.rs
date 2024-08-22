use conways_game_of_life::game::Game;

use macroquad::prelude::*;

use std::{thread::sleep, time::Duration};

const SQUARES: i16 = 32;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut game = Game::new_with_seed(
        [
            (6, 6),
            (6, 7),
            (6, 8),
            (7, 6),
            (7, 8),
            (8, 6),
            (8, 7),
            (8, 8),
        ]
        .to_vec(),
    );

    let mut game_paused = false;
    let mut edit_mode = false;

    loop {
        clear_background(LIGHTGRAY);

        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

        draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

        for i in 1..SQUARES {
            draw_line(
                offset_x,
                offset_y + sq_size * i as f32,
                screen_width() - offset_x,
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
                screen_height() - offset_y,
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

        if game_paused {
            if is_key_pressed(KeyCode::E) {
                edit_mode = !edit_mode;
            }

            if edit_mode {
                draw_text("Edit Mode", 300., 30., 40., BLACK);
                draw_text("Click on a live cell to kill it", 140., 550., 25., BLACK);
                draw_text(
                    "Click on a dead cell to make it alive",
                    140.,
                    565.,
                    25.,
                    BLACK,
                );
                draw_text("Press E to exit Edit Mode", 140., 580., 25., BLACK);

                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();

                    let grid_x = ((mouse_x - offset_x) / sq_size).floor() as i32;
                    let grid_y = ((mouse_y - offset_y) / sq_size).floor() as i32;

                    if grid_x >= 0
                        && grid_x < SQUARES as i32
                        && grid_y >= 0
                        && grid_y < SQUARES as i32
                    {
                        if game.is_cell_alive(grid_x, grid_y) {
                            game.remove_cell(grid_x, grid_y);
                        } else {
                            game.add_cell(grid_x, grid_y);
                        }
                    }
                }
            } else {
                draw_text("Game Paused", 300., 30., 40., BLACK);
                draw_text("Press E to enter Edit Mode", 140., 580., 25., BLACK);
            }
        }

        if !game_paused {
            sleep(Duration::from_secs(1));
            game.update_game();

            draw_text("Conway's Game of Life", 220., 30., 40., BLACK);
            draw_text("Press SPACE to pause", 140., 580., 25., BLACK);
        }

        if is_key_pressed(KeyCode::Space) {
            game_paused = !game_paused;
        }

        next_frame().await;
    }
}
