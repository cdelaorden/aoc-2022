pub fn sum_priorities(data:String) -> () {
  let rucksacks: Vec<&str> = data.split_terminator('\n').collect();
  println!("Rucksack 1 {} with length {}", rucksacks[0], rucksacks[0].len());
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