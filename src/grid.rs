use crate::cell::{Cell, CellColor};
use crate::opt::GameType;
use rand::Rng;
use std::collections::HashMap;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize, concentration: i32, game_type: GameType) -> Self {
        let mut grid = Grid {
            width,
            height,
            cells: vec![Cell::Dead; width * height],
        };

        let mut rng = rand::thread_rng();

        for i in 0..grid.cells.len() {
            let r = rng.gen_range(0..100);
            if r < concentration {
                match game_type {
                    GameType::Classic => {
                        grid.cells[i] = Cell::Alive {
                            color: CellColor::C1,
                        }
                    }
                    GameType::Immigration {
                        type1_concentration,
                    } => {
                        let t1 = rng.gen_range(0..100);
                        if t1 < type1_concentration {
                            grid.cells[i] = Cell::Alive {
                                color: CellColor::C1,
                            }
                        } else {
                            grid.cells[i] = Cell::Alive {
                                color: CellColor::C2,
                            }
                        }
                    }
                }
            }
        }

        grid
    }

    pub fn next(&self) -> Grid {
        let mut grid = Grid {
            width: self.width,
            height: self.height,
            cells: vec![Cell::Dead; self.width * self.height],
        };

        for i in 0..self.width {
            for j in 0..self.height {
                if let Some(color) = self.is_cell_alive(i, j) {
                    let index = grid.get_index(i, j);
                    grid.cells[index] = Cell::Alive { color };
                }
            }
        }

        grid
    }

    fn is_cell_alive(&self, row: usize, column: usize) -> Option<CellColor> {
        let mut neighbors: HashMap<CellColor, i32> = HashMap::new();
        let cell = self.get_cell(row, column);

        for i in -1..2 {
            let r = (row as isize + i).rem_euclid(self.width as isize) as usize;
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }

                let c = (column as isize + j).rem_euclid(self.height as isize) as usize;

                match self.get_cell(r, c) {
                    Cell::Alive { color } => {
                        let counter = neighbors.entry(color).or_insert(0);
                        *counter += 1;
                    }
                    Cell::Dead => {}
                }
            }
        }

        let neighbors_count: i32 = neighbors.values().sum();

        match cell {
            Cell::Alive { color } => {
                if neighbors_count == 2 || neighbors_count == 3 {
                    Some(color)
                } else {
                    None
                }
            }
            Cell::Dead => {
                if neighbors_count == 3 {
                    let neighbor = neighbors
                        .iter()
                        .max_by(|a, b| a.1.cmp(&b.1))
                        .map(|(k, _v)| k)
                        .unwrap();

                    Some(*neighbor)
                } else {
                    None
                }
            }
        }
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Cell {
        self.cells[self.get_index(row, column)]
    }
}
