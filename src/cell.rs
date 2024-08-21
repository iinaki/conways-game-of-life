#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Cell { x, y }   
    }

    // Returns the positions of the 8 neighbours of a cell
    pub fn neighbour_positions(&self) -> Vec<Cell> {
        let mut positions = Vec::with_capacity(8);

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                if self.x + dx < 1 || self.y + dy < 1 {
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
}

#[derive(Debug)]
pub enum FailedToCreateCell {
    InvalidCoordinates(String),
}
