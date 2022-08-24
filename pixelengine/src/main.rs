pub const GRID_WIDTH: u8 = 80;
pub const GRID_HEIGHT: u8 = 24;

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
            // order tracks rendering
            // and cellular automata 
            // behavioral updating sequence
            order: 0,
        }
    }
}

pub struct Grid {
    width: u8,
    height: u8,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(&mut self, width: u8, height: u8) -> Self {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                self.cells.push(Cell::new(true));
            }
        }
        
        Self {
            width: width,
            height: height,
            cells: Vec::<Cell>::new(),
        }
    }
}