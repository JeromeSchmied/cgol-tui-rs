use crate::shapes::HandleError;
use ratatui::{style::Color, widgets::canvas::Shape};
use std::time::Duration;

/// Default poll duration
pub const DEF_DUR: Duration = Duration::from_millis(400);

/// App
pub mod app;
/// Keymaps to handle input events
pub mod kmaps;
/// Starting shapes
pub mod shapes;
/// ui
pub mod ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Area {
    pub width: u16,
    pub height: u16,
}
impl Area {
    pub fn new(width: impl Into<u16>, height: impl Into<u16>) -> Self {
        Area {
            width: width.into(),
            height: height.into(),
        }
    }
    pub fn with_width(self, width: impl Into<u16>) -> Self {
        Self::new(width, self.height)
    }
    pub fn with_height(self, height: impl Into<u16>) -> Self {
        Self::new(self.width, height)
    }
    pub fn add_to_width(self, width: impl Into<i32>) -> Self {
        self.with_width((self.width as i32 + width.into()) as u16)
    }
    pub fn add_to_height(self, height: impl Into<i32>) -> Self {
        self.with_height((self.height as i32 + height.into()) as u16)
    }
    pub const fn len(&self) -> usize {
        self.width as usize * self.height as usize
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<U1: Into<u16>, U2: Into<u16>> From<(U1, U2)> for Area {
    fn from(val: (U1, U2)) -> Self {
        Self {
            width: val.0.into(),
            height: val.1.into(),
        }
    }
}

/// information about one `Cell`: either `Dead` or `Alive`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Cell {
    #[default]
    Dead = 0,
    Alive = 1,
}
impl From<bool> for Cell {
    fn from(alive: bool) -> Self {
        if alive {
            Self::Alive
        } else {
            Self::Dead
        }
    }
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
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Universe {
    area: Area,
    cells: Vec<Cell>,
}
impl<U1: Into<usize>, U2: Into<usize>> std::ops::Index<(U1, U2)> for Universe {
    type Output = Cell;

    fn index(&self, idx: (U1, U2)) -> &Self::Output {
        let row = idx.0.into();
        let col = idx.1.into();
        // Convert (x;y) to index
        let idx = self.get_idx(row, col);

        &self.cells[idx]
    }
}
impl<U1: Into<usize>, U2: Into<usize>> std::ops::IndexMut<(U1, U2)> for Universe {
    fn index_mut(&mut self, idx: (U1, U2)) -> &mut Self::Output {
        let row = idx.0.into();
        let col = idx.1.into();
        // Convert (x;y) to index
        let idx = self.get_idx(row, col);

        &mut self.cells[idx]
    }
}

impl Universe {
    fn get_idx(&self, row: impl Into<usize>, col: impl Into<usize>) -> usize {
        let row = row.into();
        let col = col.into();
        assert!(
            (0..self.area.height).contains(&(row as u16)),
            "index out of range: len is {}, but index is {row}",
            self.area.height,
        );
        assert!(
            (0..self.area.width).contains(&(col as u16)),
            "index out of range: len is {}, but index is {col}",
            self.area.width,
        );
        // Convert (x;y) to index
        let idx = (row * self.area.width as usize) + col;
        assert!(
            idx < self.cells.len(),
            "index out of range: len is {}, but index is {idx}",
            self.cells.len()
        );
        idx
    }
    fn get_idx_res(&self, row: impl Into<usize>, col: impl Into<usize>) -> Option<usize> {
        let row = row.into();
        let col = col.into();
        if !(0..self.area.height).contains(&(row as u16)) {
            log::debug!("row is {row}, but len is {}", self.area.height);
            return None;
        }
        if !(0..self.area.width).contains(&(col as u16)) {
            log::debug!("col is {col}, but len is {}", self.area.width);
            return None;
        }
        // Convert (x;y) to index
        let idx = (row * self.area.width as usize) + col;
        if idx >= self.cells.len() {
            log::debug!("idx: {idx}, len: {}", self.cells.len());
            return None;
        }
        // log::debug!("idx: {idx}");
        Some(idx)
    }
    pub fn get(&self, idx: (impl Into<usize>, impl Into<usize>)) -> Option<&Cell> {
        // log::debug!("get()");
        let idx = self.get_idx_res(idx.0, idx.1)?;
        self.cells.get(idx)
    }
    // pub fn get_mut(&mut self, idx: (impl Into<usize>, impl Into<usize>)) -> Option<&mut Cell> {
    //     // log::debug!("get_mut()");
    //     let idx = self.get_idx(idx.0, idx.1);
    //     self.cells.get_mut(idx)
    // }

    fn live_neighbour_count(&self, row: u16, col: u16) -> u8 {
        let mut sum = 0;

        for delta_row in [self.area.height - 1, 0, 1] {
            for delta_col in [self.area.width - 1, 0, 1] {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.area.height;
                let neighbour_col = (col + delta_col) % self.area.width;

                sum += self[(neighbour_row, neighbour_col)] as u8;
                // sum += *self.get((neighbour_row, neighbour_col)).unwrap() as u8;
            }
        }
        sum
    }

    /// Convert properly formatted Vec of Strings to Universe
    fn from_vec_str(s: &[String]) -> Self {
        let mut cells = Vec::new();

        for line in s {
            if line.starts_with('!') {
                continue;
            }
            for ch in line.chars() {
                if ch == '#' || ch == '1' || ch == 'O' {
                    cells.push(Cell::Alive);
                } else if ch == '_' || ch == ' ' || ch == '0' || ch == '.' {
                    cells.push(Cell::Dead);
                } else {
                    eprintln!("can't do nothing with this character: {ch}");
                }
            }
        }

        let area = Area {
            width: s[0].len() as u16,
            height: s.len() as u16,
        };
        Universe { area, cells }
    }

    fn from_str(s: &str) -> Self {
        let v = s
            .trim()
            .lines()
            .map(std::convert::Into::into)
            .collect::<Vec<String>>();
        Self::from_vec_str(&v)
    }

    /// Create universe with width, height: inserting starting shape into the middle
    ///
    /// # Errors
    ///
    /// if shape can't fit universe
    fn from_figur(area: Area, figur: &[String]) -> Result<Universe, HandleError> {
        let figur = Universe::from_vec_str(figur);
        let figur_alive = figur
            .cells
            .iter()
            .filter(|cell| *cell == &Cell::Alive)
            .count();

        if area < figur.area {
            return Err(HandleError::TooBig);
        }

        let cells = vec![Cell::default(); area.len()];
        let mut univ = Universe { area, cells };

        let (start_row, start_col) = (
            (area.height - figur.height()) / 2,
            (area.width - figur.width()) / 2,
        );

        let mut j = 0;
        for row in start_row as usize..start_row as usize + figur.height() as usize {
            for i in 0..figur.width() as usize {
                univ[(row, start_col as usize + i)] = figur.cells[j];
                // *univ.get_mut((row, start_col as usize + i)).unwrap() = figur.cells[j];
                j += 1;
            }
        }

        let univ_alive = univ
            .cells
            .iter()
            .filter(|cell| *cell == &Cell::Alive)
            .count();
        if figur_alive == univ_alive {
            Ok(univ)
        } else {
            Err(HandleError::Other)
        }
    }

    /// update life: `Universe`
    pub fn tick(&mut self) {
        let mut next = self.clone();

        for row in 0..self.height() {
            for col in 0..self.width() {
                let idx = (row, col);
                // let cell = self.get(idx).unwrap();
                let cell = self[idx];
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
                // *next.get_mut(idx).unwrap() = next_cell;
            }
        }

        *self = next;
    }

    pub fn width(&self) -> u16 {
        self.area.width
    }

    pub fn height(&self) -> u16 {
        self.area.height
    }
}

impl Shape for Universe {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                match self.get((y, x)) {
                    Some(Cell::Alive) => painter.paint(x.into(), y.into(), Color::White),
                    Some(Cell::Dead) => continue,
                    None => unreachable!("got None"),
                }
            }
        }
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "╭{}╮\r", "─".repeat(self.area.width as usize * 2))?;
        for line in self.cells.as_slice().chunks(self.area.width as usize) {
            write!(f, "│")?;
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' }; // ◻
                write!(f, "{symbol} ")?;
            }
            writeln!(f, "│\r")?;
        }
        writeln!(f, "╰{}╯\r", "─".repeat(self.area.width as usize * 2))
    }
}
#[cfg(test)]
mod tests;
