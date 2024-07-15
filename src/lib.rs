use ratatui::{style::Color, widgets::canvas::Shape};

use crate::shapes::HandleError;
use std::time::Duration;

/// Default poll duration
pub const DEF_DUR: Duration = Duration::from_millis(400);
/// Default Width and Height
// pub const DEF_WH: u16 = 32;

/// App
pub mod app;
/// Keymaps to handle input events
pub mod kmaps;
/// Starting shapes
pub mod shapes;
/// ui
pub mod ui;

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
    #[allow(unused)]
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}

/// the `Universe` in which game plays. Represented as a `Vec` of `Cell`s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Universe {
    width: u16,
    height: u16,
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
    fn get_idx<U1: Into<usize>, U2: Into<usize>>(&self, row: U1, col: U2) -> usize {
        let row = row.into();
        let col = col.into();
        // Convert (x;y) to index
        (row * self.width as usize) + col
    }

    fn live_neighbour_count(&self, row: u16, col: u16) -> u8 {
        let mut sum = 0;

        for delta_row in [self.height - 1, 0, 1] {
            for delta_col in [self.width - 1, 0, 1] {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (col + delta_col) % self.width;

                sum += self[(neighbour_row, neighbour_col)] as u8;
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
                    eprintln!("can't do nothing with this character: {ch}");
                }
            }
        }

        Universe {
            width: s[0].len() as u16,
            height: s.len() as u16,
            cells,
        }
    }

    /// Create universe with width, height: inserting starting shape into the middle
    ///
    /// # Errors
    ///
    /// if shape can't fit universe
    fn from_figur(wh: u16, figur: &[String]) -> Result<Universe, HandleError> {
        let figur = Universe::from_vec_str(figur);
        let figur_alive = figur
            .cells
            .iter()
            .filter(|cell| cell == &&Cell::Alive)
            .count();

        if wh < figur.height() || wh < figur.width() {
            return Err(HandleError::TooBig);
        }

        let cells = vec![Cell::default(); wh.pow(2).into()];
        let mut univ = Universe {
            cells,
            width: wh,
            height: wh,
        };

        let (start_row, start_col) = ((wh - figur.height()) / 2, (wh - figur.width()) / 2);

        let mut j = 0;
        for row in start_row as usize..start_row as usize + figur.height() as usize {
            for i in 0..figur.width() as usize {
                univ[(row, start_col as usize + i)] = figur.cells[j];
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
        let mut next = self.clone();

        for row in 0..self.width {
            for col in 0..self.height {
                let idx = (row, col);
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
            }
        }

        *self = next;
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }
}

impl Shape for Universe {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self[(x, y)] {
                    Cell::Alive => painter.paint(y.into(), x.into(), Color::White),
                    Cell::Dead => continue,
                }
            }
        }
    }
}

// impl fmt::Display for Universe {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(f, "╭{}╮\r", "─".repeat(self.width as usize * 2))?;
//         for line in self.cells.as_slice().chunks(self.width as usize) {
//             write!(f, "│")?;
//             for &cell in line {
//                 let symbol = if cell == Cell::Dead { ' ' } else { '◼' }; // ◻
//                 write!(f, "{symbol} ")?;
//             }
//             writeln!(f, "│\r")?;
//         }
//         writeln!(f, "╰{}╯\r", "─".repeat(self.width as usize * 2))?;
//         Ok(())
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;

    const WH: u16 = 20;

    fn gen_uni(w: u16, h: u16, cells: &[bool]) -> Universe {
        let cells = cells.iter().map(|c| (*c).into()).collect::<Vec<Cell>>();
        Universe {
            width: w,
            height: h,
            cells,
        }
    }

    #[test]
    fn rabbit_hole() {
        let rabbit = shapes::featherweigth_spaceship();
        let rabbit_uni = Universe::from_vec_str(&rabbit);
        let cells = [false, false, true, true, false, true, false, true, true];
        let uni = gen_uni(3, 3, &cells);
        assert_eq!(rabbit_uni, uni);
    }
    #[test]
    fn full() {
        let full = shapes::full(WH);
        let cells = [true; WH.pow(2) as usize];
        let uni = gen_uni(WH, WH, &cells);
        assert_eq!(full, uni);
    }
}
