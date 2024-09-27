use super::*;

/// Number of currently supported shapes
pub const N: u8 = 9;

#[derive(Debug)]
pub enum HandleError {
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
pub fn get(area: Area, i: usize) -> Result<Universe, HandleError> {
    if i > shapes::N as usize {
        return Err(HandleError::OutOfRange);
    }

    match i {
        0 => Universe::from_figur(area, &shapes::featherweigth_spaceship()),

        1 => Universe::from_figur(area, &shapes::copperhead()),

        2 => Universe::from_figur(area, &shapes::gosper_glider_gun()),

        3 => Ok(shapes::stripes(area)),

        4 => Ok(shapes::rand(area)),

        5 => Universe::from_figur(area, &shapes::rabbits()),

        6 => Universe::from_figur(area, &shapes::bonk_tie()),

        7 => Universe::from_figur(area, &shapes::acorn()),

        8 => Ok(shapes::full(area)),

        _ => Err(HandleError::OutOfRange),
    }
}
#[test]
fn get_test() {
    let area = Area::new(40u8, 40u8);
    for i in 0..N {
        assert!(get(area, i.into()).is_ok());
    }
    assert!(get(area, N.into()).is_err());
}

pub fn copperhead() -> Vec<String> {
    // ["_".repeat(5), "#_##".into(), "_".repeat(7), "#".into(), "_".repeat(6), "#".into(), "___##___#__###_"]
    [
        "_____#_##___".to_owned(),
        "____#______#".to_owned(),
        "___##___#__#".to_owned(),
        "##_#_____##_".to_owned(),
        "##_#_____##_".to_owned(),
        "___##___#__#".to_owned(),
        "____#______#".to_owned(),
        "_____#_##___".to_owned(),
    ]
    .to_vec()
}

pub fn gosper_glider_gun() -> Vec<String> {
    [
        ["_".repeat(24), "#".into(), "_".repeat(11)].concat(),
        ["_".repeat(22), "#_#".into(), "_".repeat(11)].concat(),
        [
            "_".repeat(12),
            "##______##".into(),
            "_".repeat(12),
            "##".into(),
        ]
        .concat(),
        [
            "_".repeat(11),
            "#___#____##".into(),
            "_".repeat(12),
            "##".into(),
        ]
        .concat(),
        [
            "##".into(),
            "_".repeat(8),
            "#_____#___##".into(),
            "_".repeat(14),
        ]
        .concat(),
        [
            "##".into(),
            "_".repeat(8),
            "#___#_##____#_#".into(),
            "_".repeat(11),
        ]
        .concat(),
        [
            "_".repeat(10),
            "#_____#".into(),
            "_".repeat(7),
            "#".into(),
            "_".repeat(11),
        ]
        .concat(),
        ["_".repeat(11), "#___#".into(), "_".repeat(20)].concat(),
        ["_".repeat(12), "##".into(), "_".repeat(22)].concat(),
    ]
    .to_vec()
}

/// 3x3
pub fn featherweigth_spaceship() -> Vec<String> {
    ["__#".into(), "#_#".into(), "_##".into()].to_vec()
}
#[test]
fn featherweight_spaceship_test() {
    let area = Area::new(3u8, 3u8);
    let m = Universe::from_vec_str(&featherweigth_spaceship());
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
pub fn rabbits() -> Vec<String> {
    [
        "#_____#_".into(),
        "__#___#_".into(),
        "__#__#_#".into(),
        "_#_#____".into(),
    ]
    .to_vec()
}
#[test]
fn rabbits_test() {
    let area = Area::new(8u8, 4u8);
    let m = Universe::from_vec_str(&rabbits());
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
pub fn bonk_tie() -> Vec<String> {
    [
        "##_".into(),
        "##_".into(),
        "__#".into(),
        "__#".into(),
        "__#".into(),
    ]
    .to_vec()
}
#[test]
fn bonk_tie_test() {
    let area = Area::new(3u8, 5u8);
    let m = Universe::from_vec_str(&bonk_tie());
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
pub fn acorn() -> Vec<String> {
    ["_#_____".into(), "___#___".into(), "##__###".into()].to_vec()
}
#[test]
fn acorn_test() {
    let area = Area::new(7u8, 3u8);
    let m = Universe::from_vec_str(&acorn());
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
    let area = Area::new(0u8, 0u8);
    let m = stripes(area);
    assert!(m.cells.is_empty());
    assert_eq!(m.area, area);
    dbg!(&m);
    assert!(m.get((4u8, 3u8)).is_none());
    assert!(m.get((3u8, 4u8)).is_none());
    assert!(m.get((0u8, 1u8)).is_none());
    assert!(m.get((1u8, 0u8)).is_none());
}

/// `area.len()`
pub fn full(area: Area) -> Universe {
    let cells = vec![Cell::Alive; area.len()];
    Universe { area, cells }
}
#[test]
fn full_test() {
    let area = Area::new(4u8, 3u8);
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

pub fn two_engine_cordership() -> String {
    todo!();
    // [
    //     "_".repeat(19),
    //     "##".into(),
    //     "_".repeat(19),
    //     "\n".into(),
    //     "_".repeat(19),
    //     "####".into(),
    //     "_".repeat(17),
    //     "\n".into(),
    // ]
    // .concat()
}

pub fn snark_loop() -> String {
    todo!()
}
