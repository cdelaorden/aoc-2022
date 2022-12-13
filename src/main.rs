use aoc_2022::read_input_file;
mod year_2022;

fn main() {
    let (day, data) = read_input_file();   
    match day {
        1 => year_2022::day_one::get_elves_calories(&data),
        2 => year_2022::day_two::get_total_score(&data),
        3 => year_2022::day_three::sum_priorities(&data),
        4 => year_2022::day_four::camp_cleanup(&data),
        5 => year_2022::day_five::get_top_of_stacks(&data),
        6 => year_2022::day_six::tuning_trouble(&data),
        7 => year_2022::day_seven::no_space_left(&data),
        8 => year_2022::day_eight::treetop_tree_house(&data),
        9 => year_2022::day_nine::rope_bridge(&data),
        10 => year_2022::day_ten::cathode_ray_tube(&data),
        11 => year_2022::day_eleven::monkey_in_the_middle(&data),
        12 => year_2022::day_twelve::hill_climbing_algorithm(&data),
        13 => year_2022::day_thirteen::distress_signal(&data),
        other => {
            println!("{}", format!("Exercise {} not found", other));
            std::process::exit(1);
        }
    }        
}

