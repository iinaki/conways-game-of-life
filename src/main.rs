use conways_game_of_life::game::Game;

use macroquad::prelude::*;

use std::{collections::LinkedList, thread::sleep, time::Duration};

const SQUARES: i16 = 16;


#[macroquad::main("Conway's Game of Life")]
async fn main() {
    //let mut game = Game::new_with_seed(vec![(1, 3), (1, 2), (1, 1)]);

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

    let mut game_over = false;

    loop {
        if !game_over {
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
        }
        
        sleep(Duration::from_secs(2));

        game.update_game();
        next_frame().await;
    }
}
