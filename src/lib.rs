/// Default poll duration
pub const DEF_DUR: Duration = Duration::from_millis(400);
/// Default Width and Height
pub const DEF_WH: u32 = 32;
/// Help message
pub const HELP: &str = r#"Blocking poll() & non-blocking read()
 - Prints current state of Conway's Game of Life if there's no event
 - Use Esc or `q` to quit
 - `j`, `k`: decreasing, increasing speed
 - press Space to pause, play
 - hit `n` to switch to next shape
 - if you want to restart, push `r`
 - to reset everything to starting phase, hit `R`
 - and now, press Enter to continue
"#;

/// Keymaps to handle input events
pub mod kmaps;
/// Starting shapes
pub mod shapes;

/// information about one `Cell`: either `Dead` or `Alive`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}

/// the `Universe` in which game plays. Represented as a `Vec` of `Cell`s.
#[derive(Debug)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    /// Convert (x;y) to index
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut sum = 0;

        for delta_row in [self.height - 1, 0, 1] {
            for delta_col in [self.width - 1, 0, 1] {
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

    /// Convert properly formatted Vec of Strings to Universe
    fn from_vec_str(s: &[String]) -> Self {
        let mut cells = Vec::new();

        for line in s {
            for ch in line.chars() {
                if ch == '#' || ch == '1' {
                    cells.push(Cell::Alive);
                } else if ch == '_' || ch == ' ' || ch == '0' {
                    cells.push(Cell::Dead);
                } else {
                    eprintln!("Can't do nothing with this character: {ch}");
                }
            }
        }

        Universe {
            width: s[0].len() as u32,
            height: s.len() as u32,
            cells,
        }
    }

    /// Create universe with width, height: inserting starting shape into the middle
    ///
    /// # Errors
    ///
    /// if shape can't fit universe
    pub fn from_figur(wh: u32, figur: &[String]) -> Result<Universe, HandleError> {
        let figur = Universe::from_vec_str(figur);
        let figur_alive = figur
            .cells
            .iter()
            .filter(|cell| cell == &&Cell::Alive)
            .count();

        println!("{}\r", &figur);

        if wh < figur.height() || wh < figur.width() {
            return Err(HandleError::TooBig);
        }

        let cells = (0..wh * wh).map(|_i| Cell::Dead).collect();
        let mut univ = Universe {
            cells,
            width: wh,
            height: wh,
        };

        let (start_row, start_col) = ((wh - figur.height()) / 2, (wh - figur.width()) / 2);
        println!("\r");

        let mut j = 0;
        for row in start_row as usize..start_row as usize + figur.height() as usize {
            let idx = univ.get_index(row as u32, start_col);
            for i in 0..figur.width() as usize {
                univ.cells[idx + i] = figur.cells[j];
                j += 1;
            }
        }

        let univ_alive = univ
            .cells
            .iter()
            .filter(|cell| cell == &&Cell::Alive)
            .count();
        if figur_alive == univ_alive {
            Ok(univ)
        } else {
            Err(HandleError::Other)
        }
    }

    /// update life: `Universe`
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
                    (Cell::Alive, 2 | 3) => Cell::Alive,
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// toggles cell at (`row`;`col`)
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }
}

use crate::shapes::HandleError;
use std::{fmt, time::Duration};

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "╭{}╮\r", "─".repeat(self.width as usize * 2))?;
        for line in self.cells.as_slice().chunks(self.width as usize) {
            write!(f, "│")?;
            for &cell in line {
                let symbol = if cell == Cell::Dead { ' ' } else { '◼' }; // ◻
                write!(f, "{symbol} ")?;
            }
            writeln!(f, "│\r")?;
        }
        writeln!(f, "╰{}╯\r", "─".repeat(self.width as usize * 2))?;
        Ok(())
    }
}

pub struct App {
    universe: Universe,
    pub wh: u32,
    i: usize,
    poll_t: Duration,
    paused: bool,
}
impl Default for App {
    fn default() -> Self {
        let wh = 36;
        let i = 0;
        App {
            universe: shapes::get(wh, i).unwrap(),
            wh,
            i,
            poll_t: DEF_DUR,
            paused: false,
        }
    }
}
impl App {
    pub fn paused(&self) -> bool {
        self.paused
    }
    pub fn poll_t(&self) -> Duration {
        self.poll_t
    }
    pub fn render_universe(&self) {
        println!("{}", self.universe);
    }

    pub fn play_pause(&mut self, prev_poll_t: &mut Duration) {
        if self.paused() {
            println!("Resuming: poll() = {:?}\r", prev_poll_t);
            self.poll_t = *prev_poll_t;
        } else {
            println!("Pausing...\r");
            *prev_poll_t = self.poll_t;
            self.poll_t = Duration::MAX;
        }
        self.paused = !self.paused;
    }
    pub fn restart(&mut self) {
        self.universe = shapes::get(self.wh, self.i).unwrap();
    }

    pub fn smaller(&mut self) {
        if let Ok(shape) = shapes::get(self.wh - 1, self.i) {
            self.universe = shape;
            self.wh -= 1;
        } else {
            eprintln!("Couldn't make smaller");
        }
    }
    pub fn bigger(&mut self) {
        if let Ok(shape) = shapes::get(self.wh + 1, self.i) {
            self.universe = shape;
        }
        self.wh += 1;
    }

    pub fn tick(&mut self) {
        self.universe.tick();
    }

    pub fn faster(&mut self, big: bool) {
        let div = if big { 2 } else { 5 };
        self.poll_t = self
            .poll_t
            .checked_sub(self.poll_t.checked_div(div).unwrap_or(DEF_DUR))
            .unwrap_or(DEF_DUR);
    }
    pub fn slower(&mut self, big: bool) {
        let div = if big { 2 } else { 5 };
        self.poll_t = self
            .poll_t
            .checked_add(self.poll_t.checked_div(div).unwrap_or(DEF_DUR))
            .unwrap_or(DEF_DUR);
    }

    pub fn next(&mut self) {
        if self.i + 1 != shapes::N as usize {
            self.i += 1;
        } else {
            self.i = 0;
        }
        if let Ok(shape) = shapes::get(self.wh, self.i) {
            self.universe = shape;
        } else {
            eprintln!("Couldn't switch to next shape\r");
        }
    }
    pub fn prev(&mut self) {
        if self.i > 0 {
            self.i -= 1;
        } else {
            self.i = shapes::N as usize - 1;
        }
        if let Ok(shape) = shapes::get(self.wh, self.i) {
            self.universe = shape;
        } else {
            eprintln!("Couldn't switch to previous shape\r");
        }
    }
}
