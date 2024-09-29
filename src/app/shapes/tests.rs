use super::*;

#[test]
fn frame_test00() {
    let area = Area::new(3, 2);
    let univ = Universe::from_vec_str(&["...".to_owned(), "...".to_owned()]);
    let frame = frame(area);
    print!("{frame}");
    assert_eq!(univ, frame);
}
#[test]
fn frame_test0() {
    let area = Area::new(3, 3);
    let univ = Universe::from_vec_str(&["...".to_owned(), ".O.".to_owned(), "...".to_owned()]);
    let frame = frame(area);
    print!("{frame}");
    assert_eq!(univ, frame);
}
#[test]
fn frame_test1() {
    let area = Area::new(4, 4);
    let univ = Universe::from_vec_str(&[
        "....".to_owned(),
        ".OO.".to_owned(),
        ".OO.".to_owned(),
        "....".to_owned(),
    ]);
    let frame = frame(area);
    print!("{frame}");
    assert_eq!(univ, frame);
}
#[test]
fn frame_test2() {
    let area = Area::new(5, 5);
    let univ = Universe::from_vec_str(&[
        ".....".to_owned(),
        ".OOO.".to_owned(),
        ".O.O.".to_owned(),
        ".OOO.".to_owned(),
        ".....".to_owned(),
    ]);
    let frame = frame(area);
    print!("{frame}");
    assert_eq!(univ, frame);
}
#[test]
fn frame_test3() {
    let area = Area::new(6, 6);
    let univ = Universe::from_vec_str(&[
        "......".to_owned(),
        ".OOOO.".to_owned(),
        ".O..O.".to_owned(),
        ".O..O.".to_owned(),
        ".OOOO.".to_owned(),
        "......".to_owned(),
    ]);
    let frame = frame(area);
    print!("{frame}");
    assert_eq!(univ, frame);
}

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
