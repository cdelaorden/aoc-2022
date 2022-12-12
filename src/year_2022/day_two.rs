
pub fn get_total_score (data:&str) {
  let lines:Vec<&str> = data.split_terminator("\n").collect();
  let matches: Vec<Vec<char>> = lines.iter().map(|line| {
    return line.chars().collect();
  }).collect();
  // first part, get total results
  let results = matches.iter().fold(0, |acc, m| 
    acc + play_game(m[0], m[2])
  );
  println!("Part one: {}", results);
  // second part, calculate what's needed to get the results
  let results = matches.iter().fold(0, |acc, m| {
    let needed = show_needed(m[0], m[2]);
    acc + play_game(m[0], hand_to_char(needed))
  });
  println!("part Two {}", results);
}

#[derive(Debug)]
enum Hand {
  Rock,
  Paper,
  Scissors
}

// plays rock_paper_scissors and returns a score for `me`: 0,3,6 for L-D-W + 1,2,3 for R, P, S
pub fn play_game (other: char, me: char) -> i32 {
  let hand1 = char_to_hand(other);
  let hand2 = char_to_hand(me);
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

fn show_needed (input:char, code:char) -> Hand {
  let input_hand = char_to_hand(input);
  match code {
      'X' => show_to_loose(input_hand),
      'Y' => show_to_draw(input_hand),
      'Z' => show_to_win(input_hand),
      _ => panic!("Invalid code {}", code)
  }
}

fn show_to_win(input:Hand) -> Hand {
  match input {
    Hand::Paper => Hand::Scissors,
    Hand::Rock => Hand::Paper,
    Hand::Scissors => Hand::Rock
  }
}

fn show_to_loose(input:Hand) -> Hand {
  match input {
    Hand::Paper => Hand::Rock,
    Hand::Rock => Hand::Scissors,
    Hand::Scissors => Hand::Paper
  }
}

fn show_to_draw(input:Hand) -> Hand {
  input
}


fn char_to_hand (input:char) -> Hand {
  if input == 'A' || input == 'X' { return Hand::Rock;}
  else if input == 'B' || input == 'Y' { return Hand::Paper; }
  else if input == 'C' || input == 'Z' { return Hand::Scissors; }
  else {
    panic!("Unknown input {}", input)
  }
}

fn hand_to_char(hand:Hand) -> char {
  match hand {
    Hand::Rock => 'A',
    Hand::Paper => 'B',
    Hand::Scissors => 'C'
  }
}