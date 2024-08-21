use std::collections::HashMap;

use crate::conways_error::ConwaysError;
use crate::cell::Cell;

pub struct Game {
    live_cells: Vec<Cell>,
}

impl Game {
    pub fn new() -> Self {
        let live_cells = vec![
            Cell::new(1, 3),
            Cell::new(1, 2),
            Cell::new(1, 1),
        ];

        Game {
            live_cells
        }
    }

    fn living_neighbours(&self, cell: &Cell) -> Vec<Cell>{
        let mut neighbours = Vec::new();

        for neighbour_cell in self.live_cells.iter() {
            if neighbour_cell == cell {
                continue;
            }

            if neighbour_cell.x == cell.x - 1 || neighbour_cell.x == cell.x || neighbour_cell.x == cell.x + 1 {
                if neighbour_cell.y == cell.y - 1 || neighbour_cell.y == cell.y || neighbour_cell.y == cell.y + 1 {
                    neighbours.push(neighbour_cell.clone());
                }
            }
        }

        neighbours
    }

    // Returns the positions of the 8 neighbours of a cell
    fn neighbour_positions(&self, cell: &Cell) -> Vec<Cell> {
        let mut positions = Vec::with_capacity(8);

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                positions.push(Cell { x: cell.x + dx, y: cell.y + dy });
            }
        }

        positions
    }

    // Create a new_live_live cells vec by populating it with the dead cells that have 3 live neighbours
    pub fn add_dead_cells(&self) -> Vec<Cell> {
        let mut dead_cells_map = HashMap::new();

        for cell in self.live_cells.iter() {
            for neighbor in self.neighbour_positions(cell) {
                if !self.live_cells.contains(&neighbor) {
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

        new_live_cells
    }

    pub fn update_game(&mut self) -> Result<(), ConwaysError> {
        // check 4
        let mut new_live_cells = self.add_dead_cells();

        //check 1,2 y 3
        for cell in self.live_cells.iter() {
            let neighbours = self.living_neighbours(cell);
            if neighbours.len() == 2 || neighbours.len() == 3 {
                new_live_cells.push(cell.clone());
            }
        }

        self.live_cells = new_live_cells;

        Ok(())
    }
}

// le dice a cargo q corra los tests solo si se corre con `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_game() {
        let mut game = Game::new();

        let live_cells = game.live_cells.clone();
        println!("LIVE CELLS 0 {:?}", live_cells);

        let _result = game.update_game();
        let live_cells = game.live_cells.clone();
        println!("LIVE CELLS 1 {:?}", live_cells);

        //  x
        //  x   ->  x x x
        //  x

        assert_eq!(live_cells.len(), 3);

        assert!(live_cells.contains(&Cell::new(0, 2)));
        assert!(live_cells.contains(&Cell::new(1, 2)));
        assert!(live_cells.contains(&Cell::new(2, 2)));

    }

    #[test]
    fn test_second_iteration_should_return_to_original_state() {
        let mut game = Game::new();

        let _result = game.update_game();

        let _result = game.update_game();
        let live_cells = game.live_cells.clone();

        assert_eq!(live_cells.len(), 3);

        assert!(live_cells.contains(&Cell::new(1, 1)));
        assert!(live_cells.contains(&Cell::new(1, 2)));
        assert!(live_cells.contains(&Cell::new(1, 3)));
    }
}
