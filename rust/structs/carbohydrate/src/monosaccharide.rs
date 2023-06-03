pub enum Class {
    ALDEHYDE,
    KETONE,
}

pub struct CarbohydrateStructure {
    pub carbon: u8,
    pub class: Class,
}
