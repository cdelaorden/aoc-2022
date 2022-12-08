#![allow(dead_code)]
mod input;
mod day_one;
mod day_two;
mod day_three;
mod day_four;


fn main() {
    let data = input::read_input_file();   
    // day_one::get_elves_calories(data);
    // day_two::get_total_score(data);
    // day_three::sum_priorities(data);
    day_four::camp_cleanup(data);
    
    
}

