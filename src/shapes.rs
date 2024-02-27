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
pub fn get(wh: u16, i: usize) -> Result<Universe, HandleError> {
    if i > shapes::N as usize {
        return Err(HandleError::OutOfRange);
    }

    match i {
        0 => Universe::from_figur(wh, &shapes::featherweigth_spaceship()),

        1 => Universe::from_figur(wh, &shapes::copperhead()),

        2 => Universe::from_figur(wh, &shapes::gosper_glider_gun()),

        3 => Ok(shapes::stripes(wh)),

        4 => Ok(shapes::rand(wh)),

        5 => Universe::from_figur(wh, &shapes::rabbits()),

        6 => Universe::from_figur(wh, &shapes::bonk_tie()),

        7 => Universe::from_figur(wh, &shapes::acorn()),

        8 => Ok(shapes::full(wh)),

        _ => Err(HandleError::OutOfRange),
    }
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

pub fn featherweigth_spaceship() -> Vec<String> {
    ["__#".into(), "#_#".into(), "_##".into()].to_vec()
}

pub fn rabbits() -> Vec<String> {
    [
        "#_____#_".into(),
        "__#___#_".into(),
        "__#__#_#".into(),
        "_#_#____".into(),
    ]
    .to_vec()
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

/// 7×3
pub fn acorn() -> Vec<String> {
    ["_#_____".into(), "___#___".into(), "##__###".into()].to_vec()
}

pub fn rand(wh: u16) -> Universe {
    let cells = (0..wh * wh)
        .map(|_i| {
            if fastrand::bool() {
                Cell::Alive
            } else {
                Cell::Dead
            }
        })
        .collect();
    Universe {
        width: wh,
        height: wh,
        cells,
    }
}

pub fn stripes(wh: u16) -> Universe {
    let cells = (0..wh * wh)
        .map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        })
        .collect();
    Universe {
        width: wh,
        height: wh,
        cells,
    }
}

pub fn full(wh: u16) -> Universe {
    let cells = vec![Cell::Alive; wh as usize * wh as usize];
    Universe {
        width: wh,
        height: wh,
        cells,
    }
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
