use std::env;
use std::fs;
use std::io::{stdin, stdout, Read, Write};

pub fn read_input_file() -> (u32, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {        
        println!("Call with `cargo run -- <day> [<path_to_input>]");
        println!("If ommited, <path_to_input> defaults to inputs/2022/<day>.txt (puzzle input)");
        std::process::exit(1);
    }
    let day_number = u32::from_str_radix(&args[1], 10).expect("Invalid day");
    let filename = env::args()
        .nth(2)
        .or_else(|| Some(format!("inputs/2022/{day_number}.txt")))
        .unwrap();
    let data = fs::read_to_string(&filename);
    if data.is_err() {
      eprintln!("ERROR: File {} not found! :(", filename);
      std::process::exit(1);
    }
    (day_number, data.unwrap())
}



pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press <Enter> to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
   
