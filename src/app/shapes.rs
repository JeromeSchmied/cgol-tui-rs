use super::*;

pub const N: usize = 4;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum HandleError {
    OutOfRange,
    TooBig,
    Other,
}

pub fn all() -> Vec<Universe> {
    vec![
        Universe::from_str(FEATHERWEIGTH_SPACESHIP),
        Universe::from_str(GOSPER_GLIDER_GUN),
        Universe::from_str(COPPERHEAD),
        Universe::from_str(RABBITS),
        Universe::from_str(BONK_TIE),
        Universe::from_str(ACORN),
    ]
}

pub fn get_special(i: usize, area: Area) -> Universe {
    match i {
        0 => full(area),
        1 => frame(area),
        2 => rand(area),
        3 => stripes(area),
        i => unreachable!("index out of bounds: len is {N} but index is {i}"),
    }
}

pub fn rand(area: Area) -> Universe {
    let cells = (0..area.len()).map(|_i| fastrand::bool().into()).collect();
    Universe { area, cells }
}

pub fn stripes(area: Area) -> Universe {
    let cells = (0..area.len())
        .map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        })
        .collect();
    Universe { area, cells }
}

pub fn empty(area: Area) -> Universe {
    let cells = vec![Cell::Dead; area.len()];
    Universe::new(area, cells)
}

pub fn full(area: Area) -> Universe {
    let cells = vec![Cell::Alive; area.len()];
    Universe { area, cells }
}

/// height: 5
/// width: 5
///  01234
/// 0.....0
/// 1.---.1
/// 2.|.|.2
/// 3.---.3
/// 4.....4
///  01234
pub fn frame(area: Area) -> Universe {
    let mut univ = empty(area);
    if area.height < 3 || area.width < 3 {
        return univ;
    }
    // horizontal
    for i in [1, area.height - 2] {
        for j in 1..area.width - 1 {
            univ[(i, j)] = Cell::Alive;
        }
    }

    // vertical
    for j in [1, area.width - 2] {
        for i in 2..area.height - 2 {
            univ[(i, j)] = Cell::Alive;
        }
    }
    univ
}

pub const COPPERHEAD: &str = "\
.....O.OO...
....O......O
...OO...O..O
OO.O.....OO.
OO.O.....OO.
...OO...O..O
....O......O
.....O.OO...";

pub const GOSPER_GLIDER_GUN: &str = "\
!Name: Gosper glider gun
!Author: Bill Gosper
!The first known gun and the first known finite pattern with unbounded growth.
!www.conwaylife.com/wiki/index.php?title=Gosper_glider_gun
........................O
......................O.O
............OO......OO............OO
...........O...O....OO............OO
OO........O.....O...OO
OO........O...O.OO....O.O
..........O.....O.......O
...........O...O
............OO
";

/// 3x3
pub const FEATHERWEIGTH_SPACESHIP: &str = "\
..O
O.O
.OO";

/// 8x4
pub const RABBITS: &str = "\
O.....O.
..O...O.
..O..O.O
.O.O....";

/// 3×5
pub const BONK_TIE: &str = "\
OO
OO
..O
..O
..O";

/// 7×3
pub const ACORN: &str = "\
.O
...O
OO..OOO";
