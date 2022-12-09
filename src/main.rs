#![allow(dead_code)]
mod input;
mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;
fn main() {
    let data = input::read_input_file();   
    // day_one::get_elves_calories(data);
    // day_two::get_total_score(data);
    // day_three::sum_priorities(data);
    // day_four::camp_cleanup(data);
    // day_five::get_top_of_stacks(data);
    // day_six::tuning_trouble(&data);
    // day_seven::no_space_left(&data)
    day_eight::treetop_tree_house(&data);
    
}

