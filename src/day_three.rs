pub fn sum_priorities(data:String) -> () {
  let rucksacks: Vec<&str> = data.split_terminator('\n').collect();
  // part one
  // get_sum_priorities(rucksacks);
  // part two
  get_sum_priorities_each_three(rucksacks);
}

fn get_sum_priorities_each_three (rucksacks: Vec<&str>) {
  let mut index = 0;
  let mut repeated_items: Vec<i32> = Vec::new();
  while index < rucksacks.len() {
    let first = rucksacks[index];
    let second = rucksacks[index+1];
    let third = rucksacks[index+2];
    for item in first.chars().into_iter() {
      if second.contains(item) && third.contains(item) {
        repeated_items.push(item_to_priority(item) as i32);
        break;
      }
    }
    index += 3;
    continue;
  }
  let result: i32 = repeated_items.iter().sum();
  println!("Result is {}", result);
}

fn get_sum_priorities(rucksacks: Vec<&str>) -> () {
  let mut sum_priorities = 0;
  rucksacks.iter().for_each(|rucksack| {
    let repeated_item = find_repeated_in_rucksack(rucksack);
    // println!("Repeated {}", repeated_item);
    // println!("Priority is {}", item_to_priority(repeated_item));
    sum_priorities += item_to_priority(repeated_item);
  });
  println!("Sum of priorities {}", sum_priorities)
}

fn find_repeated_in_rucksack(contents:&str) -> char {
  let (left, right) = contents.split_at(contents.len() / 2);
  for item in left.chars().into_iter() {
    if right.contains(item) {
      return item
    }    
  }
  panic!("No items were duplicated!")
}

fn item_to_priority(item:char) -> u32 {
  if item.is_ascii_lowercase() { 
    return item as u32 - 96;
  } 
  else { 
    return item as u32 - 38;
  }
}