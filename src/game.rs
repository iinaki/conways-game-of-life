use std::collections::HashMap;

use crate::cell::Cell;

/// Represents the state of Conway's Game of Life. Updates the game state based on the four rules of the game.
pub struct Game {
    live_cells: Vec<Cell>,
}

impl Game {
    /// Creates a new `Game` instance with a given seed. Each tuple represents the coordinates of a live cell.
    ///
    /// # Parameters
    ///
    /// - `seed`: A vector of tuples where each tuple contains the coordinates of a live cell.
    ///
    /// # Returns
    ///
    /// A new `Game` instance with cells initialized from the `seed`.
    ///
    pub fn new_with_seed(seed: Vec<(i32, i32)>) -> Self {
        let live_cells = seed.into_iter().map(|(x, y)| Cell::new(x, y)).collect();

        Game { live_cells }
    }

    /// Calculates the next generation of live cells based on Conway's rules.
    ///
    /// It identifies dead cells that have exactly 3 live neighbors and then
    /// identifies current live cells that have 2 or 3 live neighbors.
    ///
    /// # Returns
    ///
    /// A vector of `Cell` objects representing the next generation of live cells.
    ///
    fn new_live_cells(&self) -> Vec<Cell> {
        let mut dead_cells_map = HashMap::new();
        let mut neighbours_count = HashMap::new();

        for cell in self.live_cells.iter() {
            for neighbor in cell.neighbour_positions() {
                if self.live_cells.contains(&neighbor) {
                    let counter = neighbours_count.entry(cell.clone()).or_insert(0);
                    *counter += 1;
                } else {
                    let counter = dead_cells_map.entry(neighbor).or_insert(0);
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

    /// Returns an iterator over the positions of the live cells.
    pub fn live_cells_positions(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        self.live_cells.iter().map(|cell| cell.position())
    }

    /// Updates the game to the next generation.
    pub fn update_game(&mut self) {
        self.live_cells = self.new_live_cells();
    }

    /// Checks if a cell with the given coordinates is alive.
    pub fn is_cell_alive(&self, x: i32, y: i32) -> bool {
        self.live_cells.contains(&Cell::new(x, y))
    }

    /// Removes a cell from the game.
    pub fn remove_cell(&mut self, x: i32, y: i32) {
        self.live_cells.retain(|cell| cell.position() != (x, y));
    }

    /// Adds a new cell to the game.
    pub fn add_cell(&mut self, x: i32, y: i32) {
        self.live_cells.push(Cell::new(x, y));
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

        assert!(game.is_cell_alive(0, 2));
        assert!(game.is_cell_alive(1, 2));
        assert!(game.is_cell_alive(2, 2));
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

        assert!(game.is_cell_alive(1, 1));
        assert!(game.is_cell_alive(1, 2));
        assert!(game.is_cell_alive(1, 3));
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

        assert!(game.is_cell_alive(2, 0));
        assert!(game.is_cell_alive(1, 1));
        assert!(game.is_cell_alive(0, 2));
        assert!(game.is_cell_alive(1, 3));
        assert!(game.is_cell_alive(2, 4));
        assert!(game.is_cell_alive(3, 3));
        assert!(game.is_cell_alive(4, 2));
        assert!(game.is_cell_alive(3, 1));
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

        assert!(game.is_cell_alive(2, 0));
        assert!(game.is_cell_alive(1, 1));
        assert!(game.is_cell_alive(0, 2));
        assert!(game.is_cell_alive(1, 3));
        assert!(game.is_cell_alive(2, 4));
        assert!(game.is_cell_alive(3, 3));
        assert!(game.is_cell_alive(4, 2));
        assert!(game.is_cell_alive(3, 1));

        assert!(game.is_cell_alive(2, 1));
        assert!(game.is_cell_alive(1, 2));
        assert!(game.is_cell_alive(2, 3));
        assert!(game.is_cell_alive(3, 2));
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

        assert!(game.is_cell_alive(0, 1));
        assert!(game.is_cell_alive(0, 2));
        assert!(game.is_cell_alive(0, 3));

        assert!(game.is_cell_alive(1, 4));
        assert!(game.is_cell_alive(2, 4));
        assert!(game.is_cell_alive(3, 4));

        assert!(game.is_cell_alive(4, 3));
        assert!(game.is_cell_alive(4, 2));
        assert!(game.is_cell_alive(4, 1));

        assert!(game.is_cell_alive(1, 0));
        assert!(game.is_cell_alive(2, 0));
        assert!(game.is_cell_alive(3, 0));
    }
}
