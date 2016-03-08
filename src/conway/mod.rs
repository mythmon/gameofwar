mod cell;
mod cells;

use std::iter::Iterator;
use rand::{self, Rng};

pub use self::cells::Cells;
pub use self::cell::Cell;
pub use self::cell::Team;

pub struct GameOfLife {
    width: usize,
    height: usize,
    pub cells: Cells,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> GameOfLife {

        GameOfLife {
            width: width,
            height: height,
            cells: Cells::new(width, height),
        }
    }

    pub fn tick(&mut self) {
        let mut new_cells = self.cells.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let neighbors = self.cells.get_neighbors(x, y);
                let parents: Vec<&Cell> = neighbors.iter().filter(|c| c.alive).map(|c| *c).collect();
                let population = parents.len();

                let cell = &mut self.cells.get(x, y).unwrap();
                if cell.alive && (population < 2 || population > 3) {
                    new_cells.get_mut(x, y).unwrap().alive = false;
                } else if !cell.alive && population == 3 {
                    let new_cell = new_cells.get_mut(x, y).unwrap();
                    new_cell.alive = true;
                    new_cell.inherit_from(&parents[..]);
                }
            }
        }
        self.cells = new_cells;
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.cells.get_mut(x, y).unwrap().alive = false;
            }
        }
    }

    #[allow(dead_code)]
    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for x in 0..self.width {
            for y in 0..self.height {
                self.cells.set(x, y, rng.gen());
            }
        }
    }

    #[allow(dead_code)]
    pub fn glider(&mut self) {
        self.clear();
        let indexes = [(2, 1), (3, 2), (3, 3), (2, 3), (1, 3)];
        for &(x, y) in &indexes {
            self.cells.get_mut(x, y).unwrap().alive = true;
        }
    }
}
