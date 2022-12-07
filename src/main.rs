use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let data = fs::read_to_string(filename).expect("Should find file");
   
    // day one
    get_elves_calories(data);
    
}

// day one - split, sum, sort
fn get_elves_calories(input:String) {
    let elves: Vec<&str> = input.split_terminator("\n\n").collect();    
    let mut calories_per_elf: Vec<i32> = elves.iter().map(sum_lines).collect();
    calories_per_elf.sort();
    calories_per_elf.reverse();
    let top_three_combined: i32 = calories_per_elf[0..3].iter().sum();
    println!("Top calories single {}", calories_per_elf[0]);    
    println!("Top 3 calories sum {}", top_three_combined);
}


fn sum_lines(line: &&str) -> i32 {
    return line.split("\n").fold(0, |acc, cal| acc + i32::from_str_radix(cal, 10).expect("Valid number"));
}