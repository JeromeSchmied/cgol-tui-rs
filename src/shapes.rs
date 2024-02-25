use super::*;

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

pub fn sir_robin() -> String {
    todo!()
}

pub fn snark_loop() -> String {
    todo!()
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

pub fn rand(width: u32, height: u32) -> Universe {
    let cells = (0..width * height)
        .map(|_i| {
            if
            /*i % 2 == 0 || i % 7 == 0*/
            fastrand::bool()
            /*js_sys::Math::random() < 0.5 */
            {
                Cell::Alive
            } else {
                Cell::Dead
            }
        })
        .collect();
    Universe {
        width,
        height,
        cells,
    }
}

pub fn stripes(width: u32, height: u32) -> Universe {
    let cells = (0..width * height)
        .map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        })
        .collect();
    Universe {
        width,
        height,
        cells,
    }
}
