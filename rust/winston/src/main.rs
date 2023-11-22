use std::env;
use std::fs::File;
use std::io::Write;
use std::process;

#[derive(Debug)]
struct Data {
    data: String,
}

impl Data {
    fn validate(args: &[String]) -> Result<Data, &'static str> {
        if args.len() < 2 {
            return Err("Not enough data to log")
        }
        let to_log = args[1].clone();
        Ok( Data { data: to_log } )
    }
}


fn main() {

    let text: Vec<String> = env::args().collect();

    let data = Data::validate(&text).unwrap_or_else(|err| {
        eprintln!("Problem validating args: {err}");
        process::exit(1);
    });

    let toLog = &data;
    println!("{:?}", toLog);

    let mut logs_file = File::create("logs.txt").expect("file creation failed");

    // writeToFile(toLog);
    //    fs::write("logs.txt", b"hello from black sherif");
    logs_file.write(toLog.data.as_bytes()).expect("write failed");
}
