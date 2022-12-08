use std::collections::HashSet;

pub fn tuning_trouble (data:&str) {
  let buffer: Vec<char> = data.chars().collect();
  for index in 0..buffer.len()-1 {
    if index < 3 { continue; }
    if are_all_different(buffer[index-3], buffer[index-2], buffer[index-1], buffer[index]){
      println!("Found different at {}", index + 1);
      break;
    }
  }
}

fn are_all_different (a:char, b:char, c:char, d:char) -> bool {
  let s = HashSet::from([a,b,c,d]);  
  s.len() == 4
}