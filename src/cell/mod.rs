#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Debug)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Cell { x, y }
    }
}
