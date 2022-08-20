fn main() {
    println!("Hello, world!");
}

pub struct Cell {
    is_full: bool,
    order: u8,
}

pub struct Grid {
    width: u8,
    height: u8,
    cells: Vec<Cell>,
}