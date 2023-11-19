use std::env;
use std::fs;

// fn writeToFile(data: String) -> Result<(), Error> {
//     fs::write("logged.txt", data)
// }

fn main() {
    let text: Vec<String> = env::args().collect();
    let toLog = &text[1..];
    println!("{:?}", toLog);

    // writeToFile(toLog);
    fs::write("logs.txt", b"hello from black sherif");
}
