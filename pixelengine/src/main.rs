fn main() {
    println!("Hello, world!");

}

pub struct Cell {
    is_full: bool,
    order: u8,
}

impl Cell {
    pub fn new(is_full: bool) -> Self {
        Self {
            is_full: is_full,
            order: 0,
        }
    }
}

pub struct Grid {
    width: u8,
    height: u8,
    cells: Vec<Cell>,
}