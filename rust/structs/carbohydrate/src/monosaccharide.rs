// General Formula
// CH<sub-2>O<sub-n>
// n is any num > 2

pub enum Class {
    ALDEHYDE,
    KETONE,
}

pub enum Molecule {
    OH,
    H,
}

// Carbohydrates have versions designated with D- and L-.
// The D version presents OH group at the right of the carbohydrate structure
// Therefore in our code we use RIGHT for D to show the kind of version that the glocuse is.
pub enum Version {
    RIGHT, // D-
    LEFT,  // L-
}

// Carbohydrates exist in the body in both chain and ring forms
// We capture the form into the CarbohydrateStructure
// In cyclic/ring forms, there are 2 versions
// alpha and beta. We capture that into the Form since the
// distinct variants are just "chain" or 2 other forms of cyclic.
pub enum Form {
    CHAIN,
    ALPHA_RING,
    BETA_RING,
}

// A molecule pair arranged in the carbohydrate structure
struct Pair {
    right: Molecule,
    left: Molecule,
}

// A representation of the structure of molecules of a carbohydrate in 2D.
// Provides information on the orientation of atoms around each carbon atom.
struct FischerProjection(Pair, Pair, Pair, Pair);

// Holds information of all the parts that come together to form
// the structure of a carbohydrate and also the projection
struct CarbohydrateStructure {
    // number of carbons tells the prefix of the carbohydrates
    // eg tri - tetr -
    carbon: u8,
    // A part of the structure
    // all carbs have an aldehyde or ketone functional group
    class: Class,
    version: Version,
    structure: FischerProjection,
    form: Form,
}

// impl CarbohydrateStructure {
//     fn new(
//         carbon: u8,
//         class: Class,
//         version: Version,
//     )  -> Self {
//         // TODO:
//         // determine Fischer Projection
//     }

//     fn new_d() -> Self {
//         //
//     }

//    fn new_l() -> Self {
//        //
//    }
// }

// // 0 = H
// // 1 = OH
// let version = vec![0, 1];

// struct D_glucose = {
//     arrangement: [H,HO,H,H];
// }

// impl D_glucose {
//     fn new(arrangement)
// }

// struct arrrangements {
//     left: str,
//     right: str,
// }

// // edge

// struct glucose {
//     arrangements: [arrangements; 4]
// }

// D_<whatever> = {
//         arrangements: [
//             { left: "H", right: "OH"},
//             { left: "H", right: "OH"},
//             { left: "H", right: "OH"},
//             { left: "H", right: "OH"},
//         ]
// }
