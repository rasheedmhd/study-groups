#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#[derive(Debug)]


pub struct Card {
    pub number: i64,
    pub suit: i64,
}

fn main() {

    let s1: String = String::from("Hello World");
    let s1 = String::from("Hello World");
    let s2 = 17;

    println!("s1: {}", s1);
    println!("s2: {}", 17);

    // Taking and returning ownership
    // this is an anti-pattern if we don't want the ownership, then we shouldn't take printlnfn main() {
    let mut ace_of_spades = Card {
        number: 1,
        suit: 1,
    };

    ace_of_spades = print_card(ace_of_spades);

    fn print_card(card: Card) -> Card {
        println!("Number: {} Suit: {}", card.number, card.suit);
        return card;
    }

    // FAILS TO PASS THE BORROW CHECKER
    // albeit this code is safe
    // it tries to do a mutable borrow of the individual
    // items of the vector not the whole vector it self
    // but the borrow checker seems not smart enough to realize that
    //    let mut v = vec![1, 2];
    //    let one = &mut v[0];
    //    let two = &mut v[1];
    //    *two += *one;

    // an abstraction over unsafe
    let mut v = vec![1, 2];
    let mut iter = v.iter_mut();
    let one = iter.next().unwrap();
    let two = iter.next().unwrap();
    *two += *one;

    println!("one: {}, two: {}", one, two);

    //create_next_card(&ace_of_spades);

    let mut v1 = vec![1, 2];
    v1.insert(0, v[0]);
    v1.get_mut(v[0]);

    println!("v1: {:#?}", v1);

    let mut str_slice = "Qu'est-ce que";
    println!("{}", str_slice);
    //str_slice.push_str("tu fais da la vie");
    println!("{}", str_slice);

    let x = 5;
    let y = x.clone();
    println!("x: {} and y: {}", x,y);

    give_ownership(); //

    //println!("give ownership: {}", give_ownership());
}

// we can have data that we are sharing with others in rust
// because we are sharing with others, (&) we cannot change
// that data(this prevents data races) but we can make a copy
// of the data and change parts of it or all of it.

//fn create_next_card(card: &Card) -> Card {
//    let mut next_card = card.clone();
//    next_card.number += 11;
//    next_card
//}

// COPY
//rust copy is a byte by byte copy of memory
//moving ownership from one to another sometmes
//copy only works on data types that implements the Copy trait
//copying from shared references is okay but copying from pointers
//that point to mutable data can cause double frees and rust prevents this

//stack
//heap
//where are function args(pointers) and local variables stored
//    ..calling conventions
//    ..go stack based args
//    ..rust call conventions

// ownership rules and what each means
//    ..1: every data has an owner
//    ..2: each data has only one owner
//    ..3: when the data gets out of scope, it is dropped

// what is a scope
// why does rust has string literals/str/string slices and Strings
// defined in the std lib

// String -> Representation
//    ..1: pointer to some growable bytes
//    ..2: length
//    ..3: capacity

// What is happening here
//    let x = 5;
//    let y = x;

//    let s1 = String::from("hello");
//    let s2 = s1;

// Undstanding how rust stores Strings and the need to prevent
// double-frees let's one understands moves intuitively

// Shallow copy vrs Deep Copy

fn give_ownership() -> String {             // give_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}


// When we call give_ownership(), what happens to the returned value?