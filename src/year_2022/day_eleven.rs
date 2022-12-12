// https://adventofcode.com/2022/day/11

use num_integer;

pub fn monkey_in_the_middle (data:&str) {
  let mut monkeys = parse_data(data);
  // println!("{:#?}", monkeys);
  let lcm = get_lcm(&monkeys);
  // part 1 - monkey business
  for _i in 0..20 {
    play_round(&mut monkeys, true, 0);
  }
  println!("Part One. Monkey business {}", get_monkey_business(&monkeys));
  // part 2
  let mut monkeys2 = parse_data(data);
  for _round in 0..10000 {
    play_round(&mut monkeys2, false, lcm);
  } 
  println!("Part Two. Monkey business {}", get_monkey_business(&monkeys2)); 
}

#[derive(Debug)]
struct Monkey 
{
  items: Vec<i64>,
  operator: char,
  operand: String,
  divisible_by: i64,
  if_true_target: usize,
  if_false_target: usize,
  inspected: u32
}
const OLD: &str = "old";

fn play_round(monkeys:&mut Vec<Monkey>, use_relief: bool, lcm: i64) {
  for i in 0..monkeys.len() {
    let monkey = &mut monkeys[i];  
    let mut moved_items = vec![]; 
    for item_index in 0..monkey.items.len() {
      let item = &monkey.items[item_index];
      let worry_level: i64;
      if use_relief {
        worry_level = apply_monkey_with_divide(monkey, *item);
      } 
      else {
        worry_level = apply_monkey_with_modulo(monkey, *item, lcm);
      }
      // where to throw
      if (worry_level % monkey.divisible_by) == 0 {
        moved_items.push((monkey.if_true_target, worry_level));
      }
      else {
        moved_items.push((monkey.if_false_target, worry_level));
      } 
    }
    monkey.inspected += monkey.items.len() as u32;
    monkey.items.clear();
    moved_items.iter().for_each(|(target, num)|{
      let _ = &monkeys[*target].items.push(*num);      
    })
  }
}

fn get_monkey_business (monkeys:&Vec<Monkey>) -> i64 {
  // get the top monkeys by inspected, and multiply those values
  let mut inspected_values: Vec<u32> = monkeys.iter()
    .map(|m| m.inspected)
    .collect();

  inspected_values.sort();
  inspected_values.reverse();
  // println!("Sorted inspected {:?}", inspected_values);
  inspected_values[0] as i64 * inspected_values[1]  as i64 
}

fn apply_monkey_with_divide(monkey: &Monkey, value: i64) -> i64 {
  let worry_level = match (monkey.operator, &monkey.operand) {
    ('+', _) if monkey.operand == OLD => value + value,
    ('*', _) if monkey.operand == OLD => value * value,
    ('+', number_str) => value + i64::from_str_radix(&number_str, 10).unwrap_or_default(),
    ('*', number_str) => value * i64::from_str_radix(&number_str, 10).unwrap_or_default(),
    _ => value  
  };
  worry_level / 3
}

fn apply_monkey_with_modulo(monkey: &Monkey, value: i64, modulo: i64) -> i64 {
  let worry_level = match (monkey.operator, &monkey.operand) {
    ('+', _) if monkey.operand == OLD => (value % modulo) * 2,
    ('*', _) if monkey.operand == OLD => (value % modulo) * (value % modulo),
    ('+', number_str) => (value % modulo) + (i64::from_str_radix(&number_str, 10).unwrap_or_default() % modulo),
    ('*', number_str) => (value % modulo) * (i64::from_str_radix(&number_str, 10).unwrap_or_default() % modulo),
    _ => panic!("Apply monkey match failed")  
  };
  worry_level % modulo
}

fn get_lcm (monkeys:&Vec<Monkey>) -> i64 {
  monkeys.iter()
    .map(|m| m.divisible_by)
    .reduce(|acc,n| num_integer::lcm(acc, n)).unwrap()
}

fn parse_data (data: &str) -> Vec<Monkey> {
  data.split_terminator("\n\n").map(|monkey_data|{
    let mut lines = monkey_data.split_terminator("\n");
    // skip first line with Monkey N:
    lines.next();
    let monkey_items = extract_items(lines.next());    
    let (operator, operand) = extract_operation(lines.next());
    let divisible_by = extract_div_by(lines.next());
    let if_true_target = extract_throw_target(lines.next());
    let if_false_target = extract_throw_target(lines.next());
    // would have been much better with a closure for the operation
    // but couldn´t make the types work
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

fn extract_items (items_text: Option<&str>) -> Vec<i64> {
  if items_text.is_none() {
    vec![]
  }
  else {
    items_text.unwrap()
      .split_whitespace()
      .skip(2)
      .map(|item_with_comma| {
        i64::from_str_radix(&item_with_comma.replace(",", ""), 10).expect("Invalid item")
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

fn extract_div_by (op_text: Option<&str>) -> i64 {
  if op_text.is_none() { return 1 }
  let number_str = op_text
    .unwrap()
    .split_whitespace()
    .last()
    .expect("Missing div by number");
    
  i64::from_str_radix(number_str, 10).expect("Error parsing div by")
}

fn extract_throw_target(target: Option<&str>) -> usize {
  let monkey_index = target.unwrap().split_whitespace().last().expect("Missing monkey id");
  usize::from_str_radix(monkey_index, 10).expect("Invalid monkey_index")
}