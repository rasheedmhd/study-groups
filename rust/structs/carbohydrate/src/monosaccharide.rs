pub enum Class {
    ALDEHYDE,
    KETONE,
}

pub enum Version {
    D,
    L,
}

enum FischerProj {
    H,
    OH,
}


struct CarbohydrateStructure {
    carbon: u8,
    class: Class,
    version: Version,
    structure: Vec<FischerProj>,
}

impl CarbohydrateStructure {
    fn new(
        carbon: u8,
        class: Class,
        version: Version,
    )  -> Self {
        // TODO:
        // determine Fischer Projection
    }
}

// 0 = H
// 1 = OH
let version = vec![0, 1];


struct D_glucose = {
    arrangement: [H,HO,H,H];
}

impl D_glucose {
    fn new(arrangement)
}





struct arrrangements {
    left: str,
    right: str,
}

// edge

struct glucose {
    arrangements: [arrangements; 4]
}

D_<whatever> = {
        arrangements: [
            { left: "H", right: "OH"},
            { left: "H", right: "OH"},
            { left: "H", right: "OH"},
            { left: "H", right: "OH"},
        ]
}
