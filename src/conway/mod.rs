use std::iter::Iterator;
use std::num::Zero;

use rand::{self, Rng};

pub struct GameOfLife {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

trait Indexer: PartialOrd + Zero {
    fn to_usize(self) -> usize;
}

impl Indexer for isize {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Indexer for usize {
    fn to_usize(self) -> usize {
        self
    }
}

impl Indexer for i32 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> GameOfLife {
        assert!(width >= 2 && height >= 2);

        let mut cells = Vec::with_capacity(width);
        for x in 0..width {
            cells.push(Vec::with_capacity(height));
            for _ in 0..height {
                cells[x].push(Cell::new());
            }
        }

        GameOfLife {
            width: width,
            height: height,
            cells: cells,
        }
    }

    pub fn tick(&mut self) {
        let mut new_map = self.cells.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let population: u8 = {
                    self.get_neighbors(x, y)
                        .into_iter()
                        .map(|c| c.alive as u8)
                        .sum()
                };

                let cell = &mut self.get(x, y).unwrap();
                if cell.alive && population < 2 {
                    new_map[x][y].alive = false;
                } else if cell.alive && population >= 4 {
                    new_map[x][y].alive = false;
                } else if !cell.alive && population == 3 {
                    new_map[x][y].alive = true;
                }
            }
        }
        self.cells = new_map;
    }

    fn get<'a, T: Indexer>(&'a self, x: T, y: T) -> Option<&'a Cell> {
        if x < T::zero() || y < T::zero() {
            return None
        }
        let x = x.to_usize();
        let y = y.to_usize();
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.cells[x][y])
        }
    }

    fn get_mut<'a, T: Indexer>(&'a mut self, x: T, y: T) -> Option<&'a mut Cell> {
        if x < T::zero() || y < T::zero() {
            return None
        }
        let x = x.to_usize();
        let y = y.to_usize();
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&mut self.cells[x][y])
        }
    }

    fn get_neighbors(&self, cx: usize, cy: usize) -> Vec<&Cell> {
        let cx = cx as isize;
        let cy = cy as isize;
        let mut neighbors = vec![];

        for x in (cx - 1)..(cx + 2) {
            for y in (cy - 1)..(cy + 2) {
                if x == cx && y == cy {
                    continue;
                }
                match self.get(x, y) {
                    Some(c) => neighbors.push(c),
                    None => (),
                }
            }
        }

        // 3 cases: corner (3), edge (5) or interior cell (8).
        let len = neighbors.len();
        if len != 3 && len != 5 && len != 8 {
            panic!(format!("Unexpected number of neighbors. Expected 3, 5, or 8. Got {} at ({}, {})", len, cx, cy));
        }
        neighbors
    }

    pub fn iter_cells(&self) -> BoardCellIterator {
        BoardCellIterator::new(&self.cells)
    }

    pub fn clear(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.get_mut(x, y).unwrap().alive = false;
            }
        }
    }

    #[allow(dead_code)]
    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for x in 0..self.width {
            for y in 0..self.height {
                self.get_mut(x, y).unwrap().alive = rng.gen::<f64>() < 0.4;
            }
        }
    }

    pub fn glider(&mut self) {
        self.clear();
        let indexes = [(2, 1), (3, 2), (3, 3), (2, 3), (1, 3)];
        for &(x, y) in &indexes {
            self.get_mut(x, y).unwrap().alive = true;
        }
    }
}

pub struct BoardCellIterator<'a> {
    x: usize,
    y: usize,
    cells: &'a Vec<Vec<Cell>>,
}

impl<'a> BoardCellIterator<'a> {
    fn new(cells: &'a Vec<Vec<Cell>>) -> BoardCellIterator<'a> {
        BoardCellIterator {
            x: 0,
            y: 0,
            cells: cells,
        }
    }
}

impl<'a> Iterator for BoardCellIterator<'a> {
    type Item = (usize, usize, &'a Cell);

    fn next(&mut self) -> Option<(usize, usize, &'a Cell)> {
        if self.y >= self.cells[0].len() {
            return None;
        }

        let cell = &self.cells[self.x][self.y];
        let x = self.x;
        let y = self.y;

        self.x += 1;
        if self.x >= self.cells.len() {
            self.x = 0;
            self.y += 1;
        }

        Some((x, y, cell))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.cells.len() * self.cells[0].len();
        (size, Some(size))
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub alive: bool,
}

impl Cell {
    fn new() -> Cell {
        Cell { alive: false }
    }
}
