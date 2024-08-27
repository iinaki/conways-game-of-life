const NEIGHBOUR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

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
        NEIGHBOUR_OFFSETS
            .iter()
            .map(|&(dx, dy)| Cell {
                x: self.x + dx,
                y: self.y + dy,
            })
            .collect()
    }

    /// Returns the position of the cell as a tuple.
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
