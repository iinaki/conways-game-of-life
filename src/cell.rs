/// Represents a cell of the Game of Life.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Cell {
    x: i32,
    y: i32,
}

impl Cell {
    /// Creates a new cell with the specified coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Cell { x, y }
    }

    // Returns the positions of the 8 neighbours of a cell.
    pub fn neighbour_positions(&self) -> Vec<Cell> {
        let mut positions = Vec::with_capacity(8);

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                if self.x + dx < 0 || self.y + dy < 0 {
                    continue;
                }

                positions.push(Cell {
                    x: self.x + dx,
                    y: self.y + dy,
                });
            }
        }

        positions
    }

    /// Returns the position of the cell as a tuple.
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
