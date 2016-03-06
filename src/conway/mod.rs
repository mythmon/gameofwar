use std::iter::Iterator;
use std::num::Zero;

use rand::{self, Rng};
use utils::Indexer;

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
                let population: u8 = {
                    self.cells.get_neighbors(x, y)
                        .into_iter()
                        .map(|c| c.alive as u8)
                        .sum()
                };

                let cell = &mut self.cells.get(x, y).unwrap();
                if cell.alive && (population < 2 || population > 3) {
                    new_cells.get_mut(x, y).unwrap().alive = false;
                } else if !cell.alive && population == 3 {
                    new_cells.get_mut(x, y).unwrap().alive = true;
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
                self.cells.get_mut(x, y).unwrap().alive = rng.gen::<f64>() < 0.4;
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

#[derive(Clone, Debug)]
pub struct Cell {
    pub alive: bool,
}

impl Cell {
    fn new() -> Cell {
        Cell { alive: false }
    }
}

#[derive(Clone, Debug)]
pub struct Cells {
    cells: Vec<Vec<Cell>>,
}

impl Cells {
    pub fn new(width: usize, height: usize) -> Cells {
        assert!(width >= 2 && height >= 2);

        let cells = (0..width)
                    .map(|_| (0..height)
                             .map(|_| Cell::new())
                             .collect())
                    .collect();

        Cells { cells: cells }
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
                if let Some(c) = self.get(x, y) {
                    neighbors.push(c);
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

    pub fn iter(&self) -> CellsIterator {
        CellsIterator::new(&self.cells)
    }

    pub fn get<T: Indexer>(&self, x: T, y: T) -> Option<&Cell> {
        if x < T::zero() || y < T::zero() {
            return None;
        }
        let x = x.as_usize();
        let y = y.as_usize();
        if x >= self.cells.len() || y >= self.cells[0].len() {
            None
        } else {
            Some(&self.cells[x][y])
        }
    }

    pub fn get_mut<T: Indexer>(&mut self, x: T, y: T) -> Option<&mut Cell> {
        if x < T::zero() || y < T::zero() {
            return None;
        }
        let x = x.as_usize();
        let y = y.as_usize();
        if x >= self.cells.len() || y >= self.cells[0].len() {
            None
        } else {
            Some(&mut self.cells[x][y])
        }
    }
}

pub struct CellsIterator<'a> {
    x: usize,
    y: usize,
    cells: &'a Vec<Vec<Cell>>,
}

impl<'a> CellsIterator<'a> {
    fn new(cells: &'a Vec<Vec<Cell>>) -> CellsIterator<'a> {
        CellsIterator {
            x: 0,
            y: 0,
            cells: cells,
        }
    }
}

impl<'a> Iterator for CellsIterator<'a> {
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
        // TODO: calculate number of remaining cells, not total
        let size = self.cells.len() * self.cells[0].len();
        (0, Some(size))
    }
}
