
pub fn get_total_score (data:String) -> i32 {
  let lines:Vec<&str> = data.split_terminator("\n").collect();
  let matches: Vec<Vec<char>> = lines.iter().map(|line| {
    return line.chars().collect();
  }).collect();
  let results = matches.iter().fold(0, |acc, m| 
    acc + play_game(m[0], m[2])
  );
  println!("Result {}", results);
  0
}

#[derive(Debug)]
enum Hand {
  Rock,
  Paper,
  Scissors
}

// plays rock_paper_scissors and returns a score
pub fn play_game (other: char, me: char) -> i32 {
  let hand1 = input_to_hand(other);
  let hand2 = input_to_hand(me);
  println!("Play was {:?} vs {:?}", hand1, hand2);
  match (hand1, hand2) {
      (Hand::Rock, Hand::Rock) => {
        return 3 + 1;
      }
      (Hand::Rock, Hand::Paper) => {
        return 6 + 2;
      }
      (Hand::Rock, Hand::Scissors) => {
        return 0 + 3;
      }
      (Hand::Paper, Hand::Rock) => {
        return 0 + 1;        
      }
      (Hand::Paper, Hand::Paper) => {
        return 3 + 2;
      }
      (Hand::Paper, Hand::Scissors) => {
        return 6 + 3;
      }
      (Hand::Scissors, Hand::Rock) => {
        return 6 + 1;
      }
      (Hand::Scissors, Hand::Paper) => {
        return 0 + 2;
      }
      (Hand::Scissors, Hand::Scissors) => {
        return 3 + 3;
      }
  }  
}

fn input_to_hand (input:char) -> Hand {
  if input == 'A' || input == 'X' { return Hand::Rock;}
  else if input == 'B' || input == 'Y' { return Hand::Paper; }
  else if input == 'C' || input == 'Z' { return Hand::Scissors; }
  else {
    panic!("Unknown input {}", input)
  }
}