#[allow(non_camel_case_types)]
#[allow(unused_variables)]

fn main() {
    let rust = study_group {
        name: String::from("Rust Study Group"),
        member_count: 23,
        lead: String::from("Starlet"),
        subject: String::from("The RPL Book"),
        organization: String::from("DevCongress"),
        domain: String::from("Programming Languauges"),
    };

    // // when typing String::from is too long
    // struct study_group {
    //     name: &str,
    //     member_count: i32,
    //     lead: &str,
    //     subject: &str
    // }

    // // When the struct instance doesn't have a field
    // let rust = study_group {
    //     name: String::from("Rust Study Group"),
    //     member_count: 23,
    //     lead: String::from("Starlet"),
    // };

    // Struct Update Syntax
    let lisp = study_group {
        name: String::from("Lisp Study Group"),
        member_count: 23,
        lead: String::from("Starlet"),
        subject: String::from("Structure and Interpretation of Computer Programs"),
        organization: String::from("DevCongress"),
        domain: String::from("Programming Languauges"),
    };

    let lisp = study_group {
        name: String::from("Lisp Study Group"),
        member_count: rust.member_count,
        lead: rust.lead,
        subject: String::from("Structure and Interpretation of Computer Programs"),
        organization: rust.organization,
        domain: rust.domain,
    };

    // We can't use a Struct more than once - of course why not.
    // let javascript = study_group {
    //     name: String::from("Javascript Study Group"),
    //     member_count: rust.member_count,
    //     lead: rust.lead,
    //     subject: String::from("You don't Know Javascript Series"),
    //     organization: rust.organization,
    //     domain: rust.domain
    // };

    let ruby = study_group {
        name: String::from("Ruby Study Group"),
        //member_count: rust.member_count,
        //lead: rust.lead,
        subject: String::from("The Odin Project"),
        //organization: rust.organization,
        //domain: rust.domain
        //..rust
        ..lisp
    };

    ////! What the F is Partial Copy
    let go = study_group {
        name: String::from("Go Study Group"),
        member_count: rust.member_count,
        lead: String::from("Yaw"),
        subject: String::from("The GPL Book"),
        organization: String::from("DevCongress"),
        domain: String::from("Programming Languauges"),
        ..lisp // This, Sir, the fuck, is partial copy. Thank You.
    };

    //println!("{:#?}", rust);
    //println!("{:#?}", lisp);
    //println!("{:#?}", javascript);
    println!("{:#?}", ruby);
    println!("{:#?}", go);

    let star1 = Lua(0.5, 0.9, 0.75, -0.5, -0.75);
    let star2 = Warik(0.5, 0.9, 0.75, -0.5, -0.75);

    let call_stack = starmaze(star1);
    // Struct Tuples are completely distinct, even if the individual values are made of the same type
    //starmaze(star2);
    println!("call_stack: {:#?}", call_stack);
}

#[derive(Debug)]
struct study_group {
    name: String,
    member_count: i32,
    lead: String,
    subject: String,
    organization: String,
    domain: String,
}

// Tuple Structs
#[derive(Debug)]
struct Star(i32, i32, i32, i32, i32);

#[derive(Debug)]
struct Lua(f32, f32, f32, f32, f32);

#[derive(Debug)]
struct Warik(f32, f32, f32, f32, f32);

struct StarofDavid(i32, i32, i32, i32, i32);

fn starmaze(starkind: Lua) -> Lua {
    let starkind = starkind;
    println!("{:#?}", starkind);
    println!("last 2: {:#?} {:#?}", starkind.3, starkind.4);
    // String Slices
    // println!("last 2: {:#?} {:#?}", starkind[..3]);
    starkind
}
