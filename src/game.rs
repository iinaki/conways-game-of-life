use std::collections::HashMap;

use crate::cell::Cell;

pub struct Game {
    live_cells: Vec<Cell>,
}

impl Game {
    pub fn new_with_seed(seed: Vec<(i32, i32)>) -> Self {
        let live_cells = seed.into_iter().map(|(x, y)| Cell::new(x, y)).collect();

        Game { live_cells }
    }

    // Create a new_live_live cells vec by populating it with the dead cells that have 3 live neighbours and the current live cells that have 2 or 3 live neighbours
    fn new_live_cells(&self) -> Vec<Cell> {
        let mut dead_cells_map = HashMap::new();
        let mut neighbours_count = HashMap::new();

        for cell in self.live_cells.iter() {
            for neighbor in cell.neighbour_positions() {
                if !self.contains(&neighbor) {
                    let counter = dead_cells_map.entry(neighbor).or_insert(0);
                    *counter += 1;
                } else {
                    let counter = neighbours_count.entry(cell.clone()).or_insert(0);
                    *counter += 1;
                }
            }
        }

        let mut new_live_cells = Vec::new();
        for (dead_cell, count) in dead_cells_map {
            if count == 3 {
                new_live_cells.push(dead_cell);
            }
        }

        for (cell, count) in neighbours_count {
            if count == 2 || count == 3 {
                new_live_cells.push(cell);
            }
        }

        new_live_cells
    }

    pub fn live_cells(&self) -> Vec<(i32, i32)> {
        self.live_cells.iter().map(|cell| cell.position()).collect()
    }

    pub fn update_game(&mut self) {
        self.live_cells = self.new_live_cells();
    }

    pub fn contains(&self, cell: &Cell) -> bool {
        self.live_cells.contains(cell)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_seed() {
        let mut game = Game::new_with_seed(vec![(1, 3), (1, 2), (1, 1)]);

        let live_cells = game.live_cells.clone();
        println!("LIVE CELLS 0 {:?}", live_cells);

        game.update_game();

        let live_cells = game.live_cells.clone();
        println!("LIVE CELLS 1 {:?}", live_cells);

        //  x
        //  x   ->  x x x
        //  x

        assert_eq!(game.live_cells.len(), 3);

        assert!(game.contains(&Cell::new(0, 2)));
        assert!(game.contains(&Cell::new(1, 2)));
        assert!(game.contains(&Cell::new(2, 2)));
    }

    #[test]
    fn second_iteration_should_return_to_original_state() {
        let mut game = Game::new_with_seed(vec![(1, 3), (1, 2), (1, 1)]);

        game.update_game();

        game.update_game();

        //  x                  x
        //  x   ->  x x x  ->  x
        //  x                  x

        assert_eq!(game.live_cells.len(), 3);

        assert!(game.contains(&Cell::new(1, 1)));
        assert!(game.contains(&Cell::new(1, 2)));
        assert!(game.contains(&Cell::new(1, 3)));
    }

    #[test]
    fn new_more_complex_seed() {
        let mut game = Game::new_with_seed(
            [
                (1, 1),
                (1, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 3),
            ]
            .to_vec(),
        );
        game.update_game();

        //                 x
        //  x x x        x   x
        //  x   x  ->  x       x
        //  x x x        x   x
        //                 x

        assert_eq!(game.live_cells.len(), 8);

        assert!(game.contains(&Cell::new(2, 0)));
        assert!(game.contains(&Cell::new(1, 1)));
        assert!(game.contains(&Cell::new(0, 2)));
        assert!(game.contains(&Cell::new(1, 3)));
        assert!(game.contains(&Cell::new(2, 4)));
        assert!(game.contains(&Cell::new(3, 3)));
        assert!(game.contains(&Cell::new(4, 2)));
        assert!(game.contains(&Cell::new(3, 1)));
    }

    #[test]
    fn complex_seed_second_iteration() {
        let mut game = Game::new_with_seed(
            [
                (1, 1),
                (1, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 3),
            ]
            .to_vec(),
        );

        game.update_game();
        game.update_game();

        //                 x              x
        //  x x x        x   x          x x x
        //  x   x  ->  x       x  ->  x x   x x
        //  x x x        x   x          x x x
        //                 x              x

        assert_eq!(game.live_cells.len(), 12);

        assert!(game.contains(&Cell::new(2, 0)));
        assert!(game.contains(&Cell::new(1, 1)));
        assert!(game.contains(&Cell::new(0, 2)));
        assert!(game.contains(&Cell::new(1, 3)));
        assert!(game.contains(&Cell::new(2, 4)));
        assert!(game.contains(&Cell::new(3, 3)));
        assert!(game.contains(&Cell::new(4, 2)));
        assert!(game.contains(&Cell::new(3, 1)));

        assert!(game.contains(&Cell::new(2, 1)));
        assert!(game.contains(&Cell::new(1, 2)));
        assert!(game.contains(&Cell::new(2, 3)));
        assert!(game.contains(&Cell::new(3, 2)));
    }

    #[test]
    fn complex_seed_third_iteration() {
        let mut game = Game::new_with_seed(
            [
                (1, 1),
                (1, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 3),
            ]
            .to_vec(),
        );

        game.update_game();
        game.update_game();
        game.update_game();

        //                 x              x            x x x
        //  x x x        x   x          x x x        x       x
        //  x   x  ->  x       x  ->  x x   x x  ->  x       x
        //  x x x        x   x          x x x        x       x
        //                 x              x            x x x

        assert_eq!(game.live_cells.len(), 12);

        assert!(game.contains(&Cell::new(0, 1)));
        assert!(game.contains(&Cell::new(0, 2)));
        assert!(game.contains(&Cell::new(0, 3)));

        assert!(game.contains(&Cell::new(1, 4)));
        assert!(game.contains(&Cell::new(2, 4)));
        assert!(game.contains(&Cell::new(3, 4)));

        assert!(game.contains(&Cell::new(4, 3)));
        assert!(game.contains(&Cell::new(4, 2)));
        assert!(game.contains(&Cell::new(4, 1)));

        assert!(game.contains(&Cell::new(1, 0)));
        assert!(game.contains(&Cell::new(2, 0)));
        assert!(game.contains(&Cell::new(3, 0)));
    }
}
