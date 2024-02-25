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
 - and now, press Enter to continue
"#;

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
    /// # Buggy!
    ///
    /// doesn't work properly for blonk-tie, banana-spark, bunnies
    ///
    /// # Errors
    ///
    /// if shape can't fit universe
    pub fn from_figur(wh: u32, figur: &[String]) -> Result<Universe, ShapeError> {
        let figur = Universe::from_vec_str(figur);
        let figur_alive = figur
            .cells
            .iter()
            .filter(|cell| cell == &&Cell::Alive)
            .count();

        println!("{}\r", &figur);

        if wh < figur.height() || wh < figur.width() {
            return Err(ShapeError::TooBig);
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
            Err(ShapeError::Other)
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

#[derive(Debug)]
pub enum ShapeError {
    OutOfRange,
    TooBig,
    Other,
}

/// Returns universe created from `i`. shape if exists
///
/// # Errors
///
/// `from_figur()`
/// `IndexOutOfRange`
pub fn get_shape(wh: u32, i: usize) -> Result<Universe, ShapeError> {
    if i > shapes::N as usize {
        return Err(ShapeError::OutOfRange);
    }

    match i {
        0 => Universe::from_figur(wh, &shapes::featherweigth_spaceship()),

        1 => Universe::from_figur(wh, &shapes::copperhead()),

        2 => Universe::from_figur(wh, &shapes::gosper_glider_gun()),

        3 => Ok(shapes::stripes(wh, wh)),

        4 => Ok(shapes::rand(wh, wh)),

        5 => Universe::from_figur(wh, &shapes::rabbits()),

        6 => Universe::from_figur(wh, &shapes::bonk_tie()),

        7 => Universe::from_figur(wh, &shapes::banana_spark()),

        _ => Err(ShapeError::OutOfRange),
    }
    // todo!();
    // Ok()
}

/// Keymaps to handle input events
pub mod kmaps {
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

    /// Create Event from ch
    fn ch_to_event(ch: char) -> Event {
        Event::Key(KeyCode::Char(ch).into())
    }

    pub fn play_pause() -> Vec<Event> {
        vec![ch_to_event(' ')]
    }

    pub fn slower() -> Vec<Event> {
        vec![ch_to_event('j'), Event::Key(KeyCode::Down.into())]
    }
    pub fn slower_big() -> Vec<Event> {
        vec![
            ch_to_event('J'),
            Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::SHIFT)),
        ]
    }

    pub fn faster() -> Vec<Event> {
        vec![ch_to_event('k'), Event::Key(KeyCode::Up.into())]
    }
    pub fn faster_big() -> Vec<Event> {
        vec![
            ch_to_event('K'),
            Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::SHIFT)),
        ]
    }

    pub fn quit() -> Vec<Event> {
        vec![
            Event::Key(KeyCode::Esc.into()),
            ch_to_event('q'),
            Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
        ]
    }

    pub fn restart() -> Vec<Event> {
        vec![ch_to_event('r')]
    }

    pub fn reset() -> Vec<Event> {
        vec![ch_to_event('R')]
    }

    pub fn next() -> Vec<Event> {
        vec![ch_to_event('n')]
    }

    pub fn bigger() -> Vec<Event> {
        vec![ch_to_event('+')]
    }
    pub fn smaller() -> Vec<Event> {
        vec![ch_to_event('-')]
    }

    // mouse-bullshit, no-need
    // pub fn toggle() -> Vec<Event> {
    // vec![Event::Mouse(MouseEvent {
    //     kind: MouseEventKind::Down(MouseButton::Left),
    //     column,
    //     row,
    //     modifiers,
    // })]
    // vec![Event::Mouse(MouseEventKind::Down(MouseButton::Left).into())]
    // vec![Event::Mouse(MouseEvent{MouseEventKind::Down(
    //     MouseButton::Left,
    // ), ..})]
    // }

    // to use mouse to toggle cells, these can be useful:
    // - terminal::size()
    // - Mouse(Event)::Push(Left)
    // - Drag(Left)
    // - execute!(io::stdout(), (Enable/Disable)MouseCapture)
    // - Cursor::position()
}

pub fn faster(poll_t: &mut Duration, big: bool) {
    let div = if big { 2 } else { 5 };
    *poll_t = poll_t
        .checked_sub(poll_t.checked_div(div).unwrap_or(DEF_DUR))
        .unwrap_or(DEF_DUR);
}

pub fn slower(poll_t: &mut Duration, big: bool) {
    let div = if big { 2 } else { 5 };
    *poll_t = poll_t
        .checked_add(poll_t.checked_div(div).unwrap_or(DEF_DUR))
        .unwrap_or(DEF_DUR);
}
