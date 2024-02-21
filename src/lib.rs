use wasm_bindgen::prelude::*;
// extern crate js_sys;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut sum = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                sum += self.cells[idx] as u8;
            }
        }
        sum
    }

    fn from_str(s: &str) -> Self {
        let mut height = 1;
        let mut cells = Vec::new();

        for ch in s.chars() {
            if ch == '#' || ch == '1' {
                cells.push(Cell::Alive);
            } else if ch == '_' || ch == ' ' || ch == '0' {
                cells.push(Cell::Dead);
            } else if ch == '\n' {
                height += 1;
            } else {
                eprintln!("Can't do nothing with this character: {}", ch);
            }
        }

        Universe {
            width: s.len() as u32 / height,
            height,
            cells,
        }
    }
}

/// Public functions exported to JavaScript as well.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.width {
            for col in 0..self.height {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                let next_cell = match (cell, live_neighbours) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, n) if n < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, n) if n > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new(width: u32, height: u32) -> Self {
        let cells = (0..width * height)
            .map(|_i| {
                if
                /*i % 2 == 0 || i % 7 == 0*/
                fastrand::bool()
                /*js_sys::Math::random() < 0.5 */
                {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn new_figur(width: u32, height: u32) -> Self {
        dbg!(height);
        dbg!(width);

        // 8×12

        // 64-8 = 56 => 28:row
        // 64-12 = 52 => 26:col

        // 32-8 = 24 => 12:row
        // 32-12 = 20 => 10:col

        let figur = Universe::from_str(&copperhead());
        dbg!(&figur);
        println!("{}", &figur);

        let cells = (0..width * height).map(|_i| Cell::Dead).collect();
        let mut uni = Universe {
            cells,
            width,
            height,
        };

        let (start_row, start_col) = ((height - figur.height()) / 2, (width - figur.width()) / 2);
        dbg!(start_row);
        dbg!(start_col);
        println!();

        let mut j = 0;
        for row in start_row as usize..start_row as usize + figur.height() as usize {
            let idx = uni.get_index(row as u32, start_col);
            for i in 0..figur.width() as usize {
                uni.cells[idx + i] = figur.cells[j];
                j += 1;
            }
        }

        uni
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new(64, 64)
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "╭{}╮", "─".repeat(self.width as usize * 2))?;
        for line in self.cells.as_slice().chunks(self.width as usize) {
            write!(f, "│")?;
            for &cell in line {
                let symbol = if cell == Cell::Dead { ' ' } else { '◼' }; // ◻
                write!(f, "{} ", symbol)?;
            }
            writeln!(f, "│")?;
        }
        writeln!(f, "╰{}╯", "─".repeat(self.width as usize * 2))?;
        Ok(())
    }
}

fn two_engine_cordership() -> String {
    todo!()
}

fn copperhead() -> String {
    "_____#_##___
____#______#
___##___#__#
##_#_____##_
##_#_____##_
___##___#__#
____#______#
_____#_##___"
        .to_string()
}
fn gosper_glider_gun() -> String {
    todo!()
}
fn sir_robin() -> String {
    todo!()
}
fn snark_loop() -> String {
    todo!()
}
