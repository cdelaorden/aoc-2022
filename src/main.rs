mod input;
mod day_one;
mod day_two;
fn main() {
    let data = input::read_input_file();
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let data = fs::read_to_string(filename).expect("Should find file");
   
    // day one
    // day_one::get_elves_calories(data);
    day_two::get_total_score(data);
    
    
}

