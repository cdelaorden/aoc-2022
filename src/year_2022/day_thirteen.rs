use std::{cmp::{Ordering}};

pub fn distress_signal(data: &str) {
  part_one_sorted_pairs(data);
  part_two_decoder_key(data);
}

fn part_one_sorted_pairs(data:&str){
  let pairs = parse_in_pairs(data);
  let mut in_order_indices = 0;
  pairs.iter().enumerate().for_each(|(idx, pair)| {
    let ordered = is_right_order(&eval(&pair.0), &eval(&pair.1));
    if let Some(true) = ordered {
      in_order_indices += idx + 1;
    }
  });
  println!("Part One. Sum of correct pair idx: {}", in_order_indices);
}

fn part_two_decoder_key(data:&str){
  let mut data_with_divider_packets = data.to_string();
  data_with_divider_packets.push_str("\n[[2]]\n[[6]]\n");
  let mut all_packets = parse_all(&data_with_divider_packets);
  
  all_packets.sort_by(|a,b| {
    match is_right_order(a, b) {
      Some(true) => Ordering::Less,
      Some(false) => Ordering::Greater,
      None => Ordering::Equal
    }
  });
  let mut decoder_key = 1;
  let first_index = all_packets.iter().position(|p| {
    p == &PacketData::List([PacketData::List([PacketData::Number(2)].to_vec())].to_vec())
  });
  if first_index.is_some() {
    decoder_key *= first_index.unwrap() + 1
  }
  else { panic!("First divider missing!")}
  let second_index = all_packets.iter().position(|p| {
    p == &PacketData::List([PacketData::List([PacketData::Number(6)].to_vec())].to_vec())
  });
  if second_index.is_some() {
    decoder_key *= second_index.unwrap() + 1
  }
  else { panic!("Second divider missing!") }
  println!("Part Two. Decoder key {}", decoder_key);
}

fn parse_in_pairs(data:&str) -> Vec<(Vec<Token>, Vec<Token>)> {
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
// now parse every line as a Packet
fn parse_all(data:&str) -> Vec<PacketData> {
  let lines = data.lines();
  let mut out = Vec::new();
  for line in lines {
    if line.len() > 0 {
      out.push(eval(&parse_list(line)));
    }
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

fn is_right_order(first:&PacketData, second:&PacketData) -> Option<bool> {
  match (first, second) {
      (PacketData::Number(a), PacketData::Number(b)) => {
        if a < b {
          Some(true)
        }
        else if a > b {
          Some(false)
        }        
        else {
          None
        }
      },
      (PacketData::Number(a), PacketData::List(_packets)) => {
        is_right_order(&PacketData::List(vec![PacketData::Number(*a)]), second)
      },
      (PacketData::List(_packets), PacketData::Number(b)) => {
        is_right_order(first, &PacketData::List(vec![PacketData::Number(*b)]))
      },
      (PacketData::List(packets_a), PacketData::List(packets_b)) => {
        for x in 0..packets_a.len() {
          let next_a = packets_a.get(x);
          let next_b = packets_b.get(x);
          if next_a.is_some() && next_b.is_some() {
            // compare both
            let member_order = is_right_order(next_a.unwrap(), next_b.unwrap());
            if member_order.is_some() {
              return member_order
            }
          }
          else if next_a.is_none() && next_b.is_some() {
            // first finished first
            return Some(true)
          }
          else {
            return Some(false)
          }
        }
        Some(true)
      }
  }
}

fn eval(tokens:&Vec<Token>) -> PacketData {
  let mut stack:Vec<Vec<PacketData>> = vec![];
  let mut token_iter = tokens.iter();
  while token_iter.len() > 0 {
    let t = token_iter.next().unwrap();
    let slen = stack.len();
    match t {
      Token::Digit(x) => {
        stack[slen-1].push(PacketData::Number(*x))
      },   
      Token::OpenList => {
        stack.push(Vec::new());        
      },  
      Token::CloseList => {
        if stack.len() > 1 {
          let inner_list = stack.pop().unwrap();
          stack[slen-2].push(PacketData::List(inner_list));
        }
        else {
          // end
        }
      }
      _ => {}
    }
  }
  PacketData::List(stack.first().unwrap().clone())
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Token {
  OpenList,
  Digit(u32),
  Separator,
  CloseList
}

type Packets = Vec<PacketData>;
#[derive(Eq, PartialEq, Debug, Clone)]
enum PacketData {
  Number(u32),
  List(Packets)
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
  fn it_evals_tokens(){
    assert_eq!(
      eval(&parse_list("[1,1]")),
      PacketData::List([PacketData::Number(1), PacketData::Number(1)].to_vec())
    );

    assert_eq!(
      eval(&parse_list("[1,[1]]")),
      PacketData::List([
        PacketData::Number(1), 
        PacketData::List([
          PacketData::Number(1)
        ].to_vec())
      ].to_vec())
    );
    assert_eq!(
      eval(&parse_list("[[[]]]")),
      PacketData::List([
        PacketData::List([ 
          PacketData::List([].to_vec())
        ].to_vec())
      ].to_vec())
    );
  }

  #[test]
  fn it_checks_order(){
    let cases = vec![
      ("[1,1,3,1,1]", "[1,1,5,1,1]", Some(true)),
      ("[[1],[2,3,4]]", "[[1],4]", Some(true)),
      ("[9]", "[[8,7,6]]", Some(false)),
      ("[[4,4],4,4]", "[[4,4],4,4,4]", Some(true)),
      ("[7,7,7,7]", "[7,7,7]", Some(false)),
      ("[]", "[3]", Some(true)),
      ("[[[]]]", "[[]]", Some(false)),
      ("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]", Some(false))
    ];
    for c in cases {
      let a = eval(&parse_list(c.0));
      let b = eval(&parse_list(c.1));
      println!("Comparing {} {}", c.0, c.1);
      assert_eq!(
        is_right_order(&a, &b),
        c.2
      )
    }
  }
}