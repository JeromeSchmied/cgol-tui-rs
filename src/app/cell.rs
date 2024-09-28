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
