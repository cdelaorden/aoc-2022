
pub fn distress_signal(data: &str) {
  let pairs = parse(data);
  let mut in_order_indices = 0;
  pairs.iter().enumerate().for_each(|(idx, pair)| {
    if is_right_order(&pair.0, &pair.1){
      println!("Right order at {}", idx+1);
      in_order_indices += (idx + 1);
    }
  });
  println!("Part One. Sum of correct pair idx: {}", in_order_indices);

}

fn parse(data:&str) -> Vec<(Vec<Token>, Vec<Token>)> {
  let mut out = Vec::new();
  let pairs = data.split_terminator("\n\n");
  for pair in pairs {
    let lines: Vec<Vec<Token>> = pair.lines().map(parse_list).collect();    
    out.push((
      lines.get(0).expect("Missing part one").clone(), 
      lines.get(1).expect("Missing part two").clone())
    );
  }
  out
}

fn parse_list (contents:&str) -> Vec<Token> {
  // skip first and last chars since they are bracks always
  return contents.chars().map(|c|{
    match c {
        '['  =>Token::OpenList,
        ']' => Token::CloseList,
        ',' => Token::Separator,
        x => Token::Digit(x.to_digit(10).expect("Invalid number!"))
    }
  })
  .fold((Vec::new(), None), |mut acc, token|{
    if let Token::Digit(x) = token {
      if acc.1.is_some() {        
        acc.1 = acc.1.map(|n|(n*10)+x);
      }
      else {
        acc.1 = Some(x);
      }
    }
    else {
      if acc.1.is_some() {
        acc.0.push(Token::Digit(acc.1.unwrap()));
        acc.1 = None;
      }
      acc.0.push(token);
    }
    return acc
  }).0
  
}

fn is_right_order(first:&Vec<Token>, second: &Vec<Token>) -> bool {
  let mut x1 = 0;
  let mut x2 = 0;
  let mut left = first.clone();
  let mut right = second.clone();
  // println!("Is right order {:?}, {:?}", first, second);
  loop {
    let left_i = first.get(x1);
    let right_i = second.get(x2);
    // println!("Comparing {:?} vs {:?} ({}, {})", left_i, right_i, x1, x2);
    // left ends first
    if left_i.is_none() && right_i.is_some() { 
      // println!("Left finished");
      return true 
    };
    // right ends first
    if right_i.is_none() { return false; }
    match (left_i.unwrap(), right_i.unwrap()) {
      (Token::Digit(x), Token::Digit(y)) => {
        // println!("Digit vs Digit {} {}", x, y);
        if x < y {
          return true
        }
        else if x > y {
          return false
        }   
        x1 += 1;
        x2 += 1;
      },
      // left digit, right list      
      (Token::Digit(n), Token::OpenList) => {
        // convert left to list and continue where we were        
        left.insert(x1, Token::OpenList);
        left.insert(x1 +2, Token::CloseList);
        // println!("Found Digit {} vs OpenList, convert left to list of one {:?}", n, &left);
        // println!("Next comparison {:?} {:?}", left[x1], right[x2]);
        // x1 += 1;
        x2 += 1;
      },
      (Token::OpenList, Token::Digit(n)) => {
        // convert right to list and continue where we were
        right.insert(x2, Token::OpenList);
        right.insert(x2 + 2, Token::CloseList);
        // println!("Found List vs Digit {}, convert right to list of one {:?}", n, &right);
        // println!("Next comparison {:?} {:?}", left[x1], right[x2]);
        x1 += 1;
        // x2 += 1;
      },
      (Token::Digit(_), Token::CloseList) => {
        return false
      },
      (Token::CloseList, Token::Digit(_)) => {
        return true
      },
      _ => {   
        x1 += 1;
        x2 += 1;           
      }
    }   
  }  
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Token {
  OpenList,
  Digit(u32),
  Separator,
  CloseList
}



#[cfg(test)]
mod test {
  use super::*;
  
  #[test]
  fn it_parses_list(){
    let res = parse_list("[1,1,1]");
    assert_eq!(res, [
      Token::OpenList,
      Token::Digit(1),
      Token::Separator,
      Token::Digit(1),
      Token::Separator,
      Token::Digit(1),
      Token::CloseList
    ], "Simple digits");
    let res = parse_list("[[1],[2,3,4]]");
    assert_eq!(res, [
      Token::OpenList,
      Token::OpenList,
      Token::Digit(1),
      Token::CloseList,
      Token::Separator,
      Token::OpenList,
      Token::Digit(2),
      Token::Separator,
      Token::Digit(3),
      Token::Separator,
      Token::Digit(4),
      Token::CloseList,
      Token::CloseList
    ], "Nested lists");
    assert_eq!(parse_list("[[],[]]"), [
      Token::OpenList,
      Token::OpenList,
      Token::CloseList,
      Token::Separator,
      Token::OpenList,
      Token::CloseList,
      Token::CloseList
    ], "Empty lists");
    assert_eq!(parse_list("[123,10]"), [
      Token::OpenList,
      Token::Digit(123),
      Token::Separator,
      Token::Digit(10),
      Token::CloseList
    ], "Multi-digit numbers")
  }

  #[test]
  fn it_checks_order(){
    // assert_eq!(
    //   is_right_order(
    //     &parse_list("[1,1,3,1,1]"), 
    //     &parse_list("[1,1,5,1,1]")
    //   ),
    //   true
    // );

    assert_eq!(
      is_right_order(
        &parse_list("[[1],[2,3,4]]"), 
        &parse_list("[[1],4]")
      ),
      true
    );

    // assert_eq!(
    //   is_right_order(
    //     &parse_list("[9]"), 
    //     &parse_list("[[8,7,6]]")
    //   ),
    //   false
    // );

    // assert_eq!(
    //   is_right_order(
    //     &parse_list("[[4,4],4,4]"), 
    //     &parse_list("[[4,4],4,4,4]")
    //   ),
    //   true
    // );

    // assert_eq!(
    //   is_right_order(
    //     &parse_list("[7,7,7,7]"), 
    //     &parse_list("[7,7,7]")
    //   ),
    //   false
    // );

    // assert_eq!(
    //   is_right_order(
    //     &parse_list("[[[]]]"), 
    //     &parse_list("[[]]")
    //   ),
    //   false
    // );

    // assert_eq!(
    //   is_right_order(
    //     &parse_list("[1,[2,[3,[4,[5,6,7]]]],8,9]"), 
    //     &parse_list("[1,[2,[3,[4,[5,6,0]]]],8,9]")
    //   ),
    //   false
    // );

  }


}