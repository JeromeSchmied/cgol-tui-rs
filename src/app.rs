use crate::{shapes, Universe, DEF_DUR, DEF_WH};
use std::time::Duration;

pub struct App {
    pub universe: Universe,
    i: usize,
    poll_t: Duration,
    paused: bool,
    wh: u16,
}
impl Default for App {
    fn default() -> Self {
        let i = 0;
        let wh = DEF_WH;
        App {
            wh,
            universe: shapes::get(wh, i).unwrap(),
            i,
            poll_t: DEF_DUR,
            paused: false,
        }
    }
}
impl App {
    pub fn new(wh: u16) -> Self {
        let i = 0;
        App {
            wh,
            universe: shapes::get(wh, i).unwrap(),
            i,
            poll_t: DEF_DUR,
            paused: false,
        }
    }
    pub fn paused(&self) -> bool {
        self.paused
    }
    pub fn poll_t(&self) -> Duration {
        self.poll_t
    }
    pub fn i(&self) -> usize {
        self.i
    }
    pub fn render_universe(&self) {
        println!("{}", self.universe);
    }
    pub fn wh(&self) -> u16 {
        self.wh
    }
    pub fn set_wh(&mut self, wh: u16) {
        self.wh = wh;
    }

    pub fn play_pause(&mut self, prev_poll_t: &mut Duration) {
        if self.paused() {
            // println!("Resuming: poll() = {:?}\r", prev_poll_t);
            self.poll_t = *prev_poll_t;
        } else {
            // println!("Pausing...\r");
            *prev_poll_t = self.poll_t;
            self.poll_t = Duration::MAX;
        }
        self.paused = !self.paused;
    }
    pub fn restart(&mut self) {
        self.universe = shapes::get(self.wh(), self.i).unwrap();
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
        // println!("poll time is now: {:?}\r", self.poll_t());
    }
    pub fn slower(&mut self, big: bool) {
        if !self.paused {
            let div = if big { 2 } else { 5 };
            self.poll_t = self
                .poll_t
                .checked_add(self.poll_t.checked_div(div).unwrap_or(DEF_DUR))
                .unwrap_or(DEF_DUR);
        }
        // println!("poll time is now: {:?}\r", self.poll_t());
    }

    pub fn next(&mut self) {
        if self.i + 1 != shapes::N as usize {
            self.i += 1;
        } else {
            self.i = 0;
        }
        if let Ok(shape) = shapes::get(self.wh(), self.i) {
            self.universe = shape;
        } else {
            eprintln!("couldn't switch to next shape");
        }
    }
    pub fn prev(&mut self) {
        if self.i > 0 {
            self.i -= 1;
        } else {
            self.i = shapes::N as usize - 1;
        }
        if let Ok(shape) = shapes::get(self.wh(), self.i) {
            self.universe = shape;
        } else {
            eprintln!("couldn't switch to previous shape");
        }
    }
}
