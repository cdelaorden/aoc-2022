// https://adventofcode.com/2022/day/11

pub fn monkey_in_the_middle (data:&str) {
  let mut monkeys = parse_data(data);
  println!("{:#?}", monkeys);
  
  // part 1 - monkey business
  for _i in 0..20 {
    play_round(&mut monkeys);
    // println!("After 20 round, {:#?}", monkeys);
    println!("Monkey business {}", get_monkey_business(&monkeys));
  }
}

#[derive(Debug)]
struct Monkey 
{
  items: Vec<i32>,
  operator: char,
  operand: String,
  divisible_by: i32,
  if_true_target: usize,
  if_false_target: usize,
  inspected: u32
}
const OLD: &str = "old";

fn play_round(monkeys:&mut Vec<Monkey>) {
  for i in 0..monkeys.len() {
    let monkey = &monkeys[i];  
    let mut moved_items = vec![]; 
    for item_index in 0..monkey.items.len() {
      let item = &monkey.items[item_index];
      let mut worry_level = match (monkey.operator, &monkey.operand) {
          ('+', _) if monkey.operand == OLD => item + item,
          ('*', _) if monkey.operand == OLD => item * item,
          ('+', number_str) => item + i32::from_str_radix(&number_str, 10).unwrap_or_default(),
          ('*', number_str) => item * i32::from_str_radix(&number_str, 10).unwrap_or_default(),
          _ => *item
      };
      worry_level = worry_level / 3;
      // where to throw
      if (worry_level % monkey.divisible_by) == 0 {
        moved_items.push((item_index, monkey.if_true_target, worry_level));
      }
      else {
        moved_items.push((item_index, monkey.if_false_target, worry_level));
      }            
    }
    moved_items.iter().enumerate().for_each(|(index, (original_index, target, num))|{
      let _ = &monkeys[*target].items.push(*num);      
      let _ = &monkeys[i].items.remove(*original_index-index);
      let _ = monkeys[i].inspected += 1;
    })
  }
}

fn get_monkey_business (monkeys:&Vec<Monkey>) -> u32 {
  // get the top monkeys by inspected, and multiply those values
  let mut inspected_values: Vec<u32> = monkeys.iter()
    .map(|m| m.inspected)
    .collect();

  inspected_values.sort();
  inspected_values.reverse();
  println!("Sorted inspected {:?}", inspected_values);
  inspected_values[0] * inspected_values[1]  
}

fn parse_data (data: &str) -> Vec<Monkey> {
  data.split_terminator("\n\n").map(|monkey_data|{
    let mut lines = monkey_data.split_terminator("\n");
    // skip first line
    lines.next();
    // extract items
    let monkey_items = extract_items(lines.next());    
    // extract operation
    let (operator, operand) = extract_operation(lines.next());
    // extract divisible by
    let divisible_by = extract_div_by(lines.next());
    let if_true_target = extract_throw_target(lines.next());
    let if_false_target = extract_throw_target(lines.next());
    // extract if_true_target
    // extract if_false_target
    Monkey {
      items: monkey_items,
      operand,
      operator,
      divisible_by,
      if_false_target,
      if_true_target,
      inspected: 0
    }
  }).collect()
}

fn extract_items (items_text: Option<&str>) -> Vec<i32> {
  if items_text.is_none() {
    vec![]
  }
  else {
    items_text.unwrap()
      .split_whitespace()
      .skip(2)
      .map(|item_with_comma| {
        i32::from_str_radix(&item_with_comma.replace(",", ""), 10).expect("Invalid item")
      })
      .collect()
  }
}

fn extract_operation(op_text: Option<&str>) -> (char, String) {
  if op_text.is_none() {
    return ('+', String::from("0"))
  }
  let rh = op_text.unwrap().split(" = ").nth(1).expect("Error parsing Operation");
  // keep * 19, or * old, or + 13
  let rh_parts: Vec<&str> = rh.split_whitespace().skip(1).collect();
  let operator = rh_parts[0];
  let operand = rh_parts[1];
  (operator.chars().nth(0).expect("Invalid operator"), String::from(operand))
}

fn extract_div_by (op_text: Option<&str>) -> i32 {
  if op_text.is_none() { return 1 }
  let number_str = op_text
    .unwrap()
    .split_whitespace()
    .last()
    .expect("Missing div by number");
    
  i32::from_str_radix(number_str, 10).expect("Error parsing div by")
}

fn extract_throw_target(target: Option<&str>) -> usize {
  let monkey_index = target.unwrap().split_whitespace().last().expect("Missing monkey id");
  usize::from_str_radix(monkey_index, 10).expect("Invalid monkey_index")
}