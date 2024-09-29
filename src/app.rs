pub use area::Area;
pub use cell::Cell;
use ratatui::crossterm::event::{self, Event, KeyEventKind};
use ratatui::{backend::Backend, Terminal};
pub use shapes::HandleError;
pub use std::str::FromStr;
use std::{io, time::Duration};
pub use universe::Universe;

/// Default poll duration
const DEF_DUR: Duration = Duration::from_millis(400);

mod area;
mod cell;
/// Keymaps to handle input events
mod kmaps;
/// Starting shapes
pub mod shapes;
/// ui
mod ui;
/// Conway's Game of Life universe
mod universe;

#[cfg(test)]
mod tests;

pub struct App {
    pub available_universes: Vec<Universe>,
    universe: Universe,
    i: usize,
    pub poll_t: Duration,
    pub paused: bool,
    pub area: Area,
}
impl Default for App {
    fn default() -> Self {
        App {
            area: Area::default(),
            universe: Universe::default(),
            i: 0,
            poll_t: DEF_DUR,
            paused: false,
            available_universes: shapes::all(),
        }
    }
}
impl App {
    pub fn with_universes(self, universes: Vec<Universe>) -> Self {
        Self {
            available_universes: [universes, shapes::all()].concat(),
            ..self
        }
    }
    pub fn new(area: Area, available_universes: Vec<Universe>, poll_t: Duration) -> Self {
        App {
            area,
            universe: available_universes[0].clone(),
            i: 0,
            poll_t,
            paused: false,
            available_universes,
        }
    }
    pub fn len(&self) -> usize {
        self.available_universes.len() + shapes::N
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self) -> Universe {
        let true_len = self.available_universes.len();
        if self.i < true_len {
            self.available_universes
                .get(self.i)
                .expect("display area is too small to fit current shape")
                .clone()
        } else {
            shapes::get_special(self.i - true_len, self.area)
        }
    }

    // pub fn render_universe(&self) {
    //     println!("{}", self.universe);
    // }

    pub fn play_pause(&mut self, prev_poll_t: &mut Duration) {
        if self.paused {
            self.poll_t = *prev_poll_t;
        } else {
            *prev_poll_t = self.poll_t;
            self.poll_t = Duration::MAX;
        }
        self.paused = !self.paused;
    }
    pub fn restart(&mut self) {
        let univ = self.get();
        self.universe = Universe::from_figur(self.area, univ).unwrap();
    }

    pub fn tick(&mut self) {
        self.universe.tick();
    }

    pub fn faster(&mut self, big: bool) {
        if !self.paused {
            let div = if big { 2 } else { 5 };
            self.poll_t = self
                .poll_t
                .checked_sub(self.poll_t.checked_div(div).unwrap_or(DEF_DUR))
                .unwrap_or(DEF_DUR);
        }
    }
    pub fn slower(&mut self, big: bool) {
        if !self.paused {
            let div = if big { 2 } else { 5 };
            self.poll_t = self
                .poll_t
                .checked_add(self.poll_t.checked_div(div).unwrap_or(DEF_DUR))
                .unwrap_or(DEF_DUR);
        }
    }

    pub fn next(&mut self) {
        if self.i + 1 == self.len() {
            self.i = 0;
        } else {
            self.i += 1;
        }
        self.restart();
    }
    pub fn prev(&mut self) {
        if self.i > 0 {
            self.i -= 1;
        } else {
            self.i = self.len() - 1;
        }
        self.restart();
    }
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let mut prev_poll_t = self.poll_t;

        loop {
            terminal.draw(|f| ui::ui(f, self))?;

            // Wait up to `poll_t` for another event
            if event::poll(self.poll_t)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    match key.code {
                        kmaps::QUIT => break,
                        kmaps::SLOWER => self.slower(false),
                        kmaps::FASTER => self.faster(false),
                        kmaps::PLAY_PAUSE => self.play_pause(&mut prev_poll_t),
                        kmaps::RESTART => self.restart(),
                        kmaps::NEXT => self.next(),
                        kmaps::PREV => self.prev(),
                        kmaps::RESET => *self = Self::default(),
                        _ => {}
                    }
                } else {
                    // resize and restart
                    self.restart();
                }
            } else {
                // Timeout expired, updating life state
                self.tick();
            }
        }

        Ok(())
    }
}
