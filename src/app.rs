pub use area::Area;
pub use cell::Cell;
use ratatui::crossterm::event::{self, poll, Event, KeyEventKind};
use ratatui::{backend::Backend, Terminal};
pub use shapes::HandleError;
use std::io;
use std::time::Duration;
pub use universe::Universe;

/// Default poll duration
pub const DEF_DUR: Duration = Duration::from_millis(400);

mod area;
mod cell;
/// Keymaps to handle input events
mod kmaps;
/// Starting shapes
mod shapes;
/// ui
mod ui;
/// Conway's Game of Life universe
mod universe;

#[cfg(test)]
mod tests;

impl App {
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let mut prev_poll_t = self.poll_t;

        loop {
            terminal.draw(|f| ui::ui(f, self))?;

            // Wait up to `poll_t` for another event
            if poll(self.poll_t)? {
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

pub struct App {
    pub universe: Universe,
    pub i: usize,
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
        }
    }
}
impl App {
    pub fn new(area: Area, universe: Universe, poll_t: Duration) -> Self {
        App {
            area,
            universe,
            i: 0,
            poll_t,
            paused: false,
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
        self.universe = shapes::get(self.area, self.i)
            .inspect_err(|e| log::error!("{e:?}"))
            .expect("display area is too small to fit current shape");
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
        if self.i + 1 == shapes::N as usize {
            self.i = 0;
        } else {
            self.i += 1;
        }
        if let Ok(shape) = shapes::get(self.area, self.i) {
            self.universe = shape;
        } else {
            log::error!(
                "couldn't switch to next shape: number of shapes: {}, idx: {}, universe: {:?}",
                shapes::N,
                self.i,
                self.universe
            );
        }
    }
    pub fn prev(&mut self) {
        if self.i > 0 {
            self.i -= 1;
        } else {
            self.i = shapes::N as usize - 1;
        }
        if let Ok(shape) = shapes::get(self.area, self.i) {
            self.universe = shape;
        } else {
            log::error!(
                "couldn't switch to previous shape: number of shapes: {}, idx: {}, universe: {:?}",
                shapes::N,
                self.i,
                self.universe
            );
        }
    }
}
