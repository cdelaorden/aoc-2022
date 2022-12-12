// day one - split, sum, sort
pub fn get_elves_calories(input:&str) {
  let elves: Vec<&str> = input.split_terminator("\n\n").collect();    
  let mut calories_per_elf: Vec<i32> = elves.iter().map(sum_lines).collect();
  calories_per_elf.sort();
  calories_per_elf.reverse();
  println!("Part 1. Top calories single {}", calories_per_elf[0]);    
  let top_three_combined: i32 = calories_per_elf[0..3].iter().sum();
  println!("Part 2. Top 3 calories sum {}", top_three_combined);
}


fn sum_lines(line: &&str) -> i32 {
  return line.split("\n").fold(0, |acc, cal| acc + i32::from_str_radix(cal, 10).expect("Valid number"));
}