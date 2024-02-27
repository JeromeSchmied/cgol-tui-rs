use crate::{shapes, Universe, DEF_DUR};
use std::time::Duration;

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
            eprintln!("couldn't switch to next shape\r");
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
