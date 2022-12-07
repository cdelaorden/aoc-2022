use std::env;
use std::fs;

pub fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let data = fs::read_to_string(filename).expect("Should find file");
    data
}
