use std::env;

fn main() {
    let text: Vec<String> = env::args().collect();
    let toLog = &text[1..];
    println!("Hello, world! {:#?}", toLog);
}
