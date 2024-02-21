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

    fn from_vec_str(s: Vec<String>) -> Self {
        let mut cells = Vec::new();

        for line in &s {
            for ch in line.chars() {
                if ch == '#' || ch == '1' {
                    cells.push(Cell::Alive);
                } else if ch == '_' || ch == ' ' || ch == '0' {
                    cells.push(Cell::Dead);
                } else {
                    eprintln!("Can't do nothing with this character: {}", ch);
                }
            }
        }

        Universe {
            width: s[0].len() as u32,
            height: s.len() as u32,
            cells,
        }
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
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

    pub fn from_figur(width: u32, height: u32, figur: Vec<String>) -> Self {
        // dbg!(height);
        // dbg!(width);

        let figur = Universe::from_vec_str(figur);
        println!("{}", &figur);

        assert!(height > figur.height());
        assert!(width > figur.width());

        let cells = (0..width * height).map(|_i| Cell::Dead).collect();
        let mut uni = Universe {
            cells,
            width,
            height,
        };

        let (start_row, start_col) = ((height - figur.height()) / 2, (width - figur.width()) / 2);
        // dbg!(start_row);
        // dbg!(start_col);
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

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
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

#[wasm_bindgen]
pub fn two_engine_cordership() -> String {
    todo!();
    // [
    //     "_".repeat(19),
    //     "##".into(),
    //     "_".repeat(19),
    //     "\n".into(),
    //     "_".repeat(19),
    //     "####".into(),
    //     "_".repeat(17),
    //     "\n".into(),
    // ]
    // .concat()
}

#[wasm_bindgen]
pub fn copperhead() -> Vec<String> {
    // ["_".repeat(5), "#_##".into(), "_".repeat(7), "#".into(), "_".repeat(6), "#".into(), "___##___#__###_"]
    [
        "_____#_##___".to_owned(),
        "____#______#".to_owned(),
        "___##___#__#".to_owned(),
        "##_#_____##_".to_owned(),
        "##_#_____##_".to_owned(),
        "___##___#__#".to_owned(),
        "____#______#".to_owned(),
        "_____#_##___".to_owned(),
    ]
    .to_vec()
}

#[wasm_bindgen]
pub fn gosper_glider_gun() -> Vec<String> {
    [
        ["_".repeat(24), "#".into(), "_".repeat(11)].concat(),
        ["_".repeat(22), "#_#".into(), "_".repeat(11)].concat(),
        [
            "_".repeat(12),
            "##______##".into(),
            "_".repeat(12),
            "##".into(),
        ]
        .concat(),
        [
            "_".repeat(11),
            "#___#____##".into(),
            "_".repeat(12),
            "##".into(),
        ]
        .concat(),
        [
            "##".into(),
            "_".repeat(8),
            "#_____#___##".into(),
            "_".repeat(14),
        ]
        .concat(),
        [
            "##".into(),
            "_".repeat(8),
            "#___#_##____#_#".into(),
            "_".repeat(11),
        ]
        .concat(),
        [
            "_".repeat(10),
            "#_____#".into(),
            "_".repeat(7),
            "#".into(),
            "_".repeat(11),
        ]
        .concat(),
        ["_".repeat(11), "#___#".into(), "_".repeat(20)].concat(),
        ["_".repeat(12), "##".into(), "_".repeat(22)].concat(),
    ]
    .to_vec()
}

#[wasm_bindgen]
pub fn sir_robin() -> String {
    todo!()
}

#[wasm_bindgen]
pub fn snark_loop() -> String {
    todo!()
}

#[wasm_bindgen]
pub fn featherweigth_spaceship() -> Vec<String> {
    ["__#".into(), "#_#".into(), "_##".into()].to_vec()
}
