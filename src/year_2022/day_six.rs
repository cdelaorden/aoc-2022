use std::collections::HashSet;

pub fn tuning_trouble (data:&str) {
  let buffer: Vec<char> = data.chars().collect();
  // part 1 - decode packet
  for index in 0..buffer.len()-1 {    
    if index < 3 { continue; }
    if are_all_different(&buffer[index-3..=index], 4) {
      println!("Part One. Packet marker at {}", index + 1);
      break;
    }
  }
  // part 2 - decode message
  for index in 0..buffer.len()-1 {    
    if index < 13 { continue; }
    if are_all_different(&buffer[index-13..=index], 14) {
      println!("Part Two. Message marker at {}", index + 1);
      break;
    }
  }
}

fn are_all_different (chars: &[char], expected_size: usize) -> bool {
  let s: HashSet<char> = HashSet::from_iter(chars.iter().cloned());  
  s.len() == expected_size
}