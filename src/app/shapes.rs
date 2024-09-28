use area::Area;
use cell::Cell;
use universe::Universe;

use super::*;

pub const N: usize = 4;

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

// height: 5
// width: 5
//  01234
// 0.....0
// 1.---.1
// 2.|.|.2
// 3.---.3
// 4.....4
//  01234
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
#[test]
fn frame_test00() {
    let area = Area::new(3, 2);
    let univ = Universe::from_vec_str(&["___".to_owned(), "___".to_owned()]);
    let frame = frame(area);
    print!("{frame}");
    assert_eq!(univ, frame);
}
#[test]
fn frame_test0() {
    let area = Area::new(3, 3);
    let univ = Universe::from_vec_str(&["___".to_owned(), "_#_".to_owned(), "___".to_owned()]);
    let frame = frame(area);
    print!("{frame}");
    assert_eq!(univ, frame);
}
#[test]
fn frame_test1() {
    let area = Area::new(4, 4);
    let univ = Universe::from_vec_str(&[
        "____".to_owned(),
        "_##_".to_owned(),
        "_##_".to_owned(),
        "____".to_owned(),
    ]);
    let frame = frame(area);
    print!("{frame}");
    assert_eq!(univ, frame);
}
#[test]
fn frame_test2() {
    let area = Area::new(5, 5);
    let univ = Universe::from_vec_str(&[
        "_____".to_owned(),
        "_###_".to_owned(),
        "_#_#_".to_owned(),
        "_###_".to_owned(),
        "_____".to_owned(),
    ]);
    let frame = frame(area);
    print!("{frame}");
    assert_eq!(univ, frame);
}
#[test]
fn frame_test3() {
    let area = Area::new(6, 6);
    let univ = Universe::from_vec_str(&[
        "______".to_owned(),
        "_####_".to_owned(),
        "_#__#_".to_owned(),
        "_#__#_".to_owned(),
        "_####_".to_owned(),
        "______".to_owned(),
    ]);
    let frame = frame(area);
    print!("{frame}");
    assert_eq!(univ, frame);
}

pub const COPPERHEAD: &str = "\
_____#_##___
____#______#
___##___#__#
##_#_____##_
##_#_____##_
___##___#__#
____#______#
_____#_##___";

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
__#
 #_#
 _##";

#[test]
fn featherweight_spaceship_test() {
    let area = Area::new(3, 3);
    let m = Universe::from_str(FEATHERWEIGTH_SPACESHIP);
    assert_eq!(m.area, area);
    dbg!(&m);
    let alive = [(0u8, 2u8), (1u8, 0u8), (1u8, 2u8), (2u8, 1u8), (2u8, 2u8)];
    for alive_cell in alive {
        dbg!(alive_cell);
        assert_eq!(m.get(alive_cell), Some(&Cell::Alive));
    }
    assert!(m.get((3u8, 3u8)).is_none());
    assert!(m.get((3u8, 4u8)).is_none());
    assert!(m.get((4u8, 3u8)).is_none());
}

/// 8x4
pub const RABBITS: &str = "\
#_____#_
__#___#_
__#__#_#
_#_#____";

#[test]
fn rabbits_test() {
    let area = Area::new(8, 4);
    let m = Universe::from_str(RABBITS);
    assert_eq!(m.area, area);
    dbg!(&m);
    let alive = [
        (0u8, 0u8),
        (0u8, 6u8),
        (1u8, 2u8),
        (1u8, 6u8),
        (2u8, 2u8),
        (2u8, 5u8),
        (2u8, 7u8),
        (3u8, 1u8),
        (3u8, 3u8),
    ];
    for alive_cell in alive {
        dbg!(alive_cell);
        assert_eq!(m.get(alive_cell), Some(&Cell::Alive));
    }
    assert!(m.get((4u8, 8u8)).is_none());
    assert!(m.get((8u8, 4u8)).is_none());
}

/// 3×5
pub const BONK_TIE: &str = "\
##_
##_
__#
__#
__#";
#[test]
fn bonk_tie_test() {
    let area = Area::new(3, 5);
    let m = Universe::from_str(BONK_TIE);
    assert_eq!(m.area, area);
    dbg!(&m);
    let alive = [
        (0u8, 0u8),
        (0u8, 1u8),
        (1u8, 0u8),
        (1u8, 1u8),
        (2u8, 2u8),
        (3u8, 2u8),
        (4u8, 2u8),
    ];
    for alive_cell in alive {
        dbg!(alive_cell);
        assert_eq!(m.get(alive_cell), Some(&Cell::Alive));
    }
    assert!(m.get((4u8, 3u8)).is_none());
    assert!(m.get((3u8, 4u8)).is_none());
}

/// 7×3
pub const ACORN: &str = "\
_#_____
___#___
##__###";

#[test]
fn acorn_test() {
    let area = Area::new(7, 3);
    let m = Universe::from_str(ACORN);
    assert_eq!(m.area, area);
    dbg!(&m);
    let alive = [
        (0u8, 1u8),
        (1u8, 3u8),
        (2u8, 0u8),
        (2u8, 1u8),
        (2u8, 4u8),
        (2u8, 5u8),
        (2u8, 6u8),
    ];
    for alive_cell in alive {
        dbg!(alive_cell);
        assert_eq!(m.get(alive_cell), Some(&Cell::Alive));
    }
    assert!(m.get((4u8, 3u8)).is_none());
    assert!(m.get((3u8, 4u8)).is_none());
}

/// `area.len()`
pub fn rand(area: Area) -> Universe {
    let cells = (0..area.len()).map(|_i| fastrand::bool().into()).collect();
    Universe { area, cells }
}

/// `area.len()`
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
#[test]
fn stripes_test() {
    let area = Area::new(0, 0);
    let m = stripes(area);
    assert!(m.cells.is_empty());
    assert_eq!(m.area, area);
    dbg!(&m);
    assert!(m.get((4u8, 3u8)).is_none());
    assert!(m.get((3u8, 4u8)).is_none());
    assert!(m.get((0u8, 1u8)).is_none());
    assert!(m.get((1u8, 0u8)).is_none());
}

pub fn empty(area: Area) -> Universe {
    let cells = vec![Cell::Dead; area.len()];
    Universe::new(area, cells)
}

/// `area.len()`
pub fn full(area: Area) -> Universe {
    let cells = vec![Cell::Alive; area.len()];
    Universe { area, cells }
}
#[test]
fn full_test() {
    let area = Area::new(4, 3);
    let m = full(area);
    assert_eq!(m.area, area);
    assert!(m.cells.iter().all(|j| *j == Cell::Alive));
    dbg!(&m);
    for i in 0..m.height() - 1 {
        for j in 0..m.width() - 1 {
            dbg!((i, j));
            assert_eq!(m.get((i, j)), Some(&Cell::Alive));
        }
    }
    assert!(m.get((4u8, 3u8)).is_none());
    assert!(m.get((3u8, 4u8)).is_none());
}
