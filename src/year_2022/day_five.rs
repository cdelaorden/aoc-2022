use std::collections::{HashMap};

pub fn get_top_of_stacks(data:&str) -> () {
  let parts: Vec<&str> = data.split("\n\n").collect();
  let diagram = parts.first().expect("Diagram not found in input file");
  let instructions = parts.last().expect("Move instructions not found in input file");
  // println!("Diagram {:#?}", diagram);
  let stacks = setup_stacks(diagram);
  let stacks_2 = stacks.clone();
  let commands = parse_commands(instructions);
  // part 1 
  println!("Part One");
  interpret(stacks, &commands, false);
  // part 2
  println!("Part Two");
  interpret(stacks_2, &commands, true);
}

fn interpret(mut stacks: HashMap<String, Vec<char>>, commands: &Vec<Command>, move_in_order: bool) -> () {
  let mut result: String = String::new();
  commands.iter().for_each(|cmd| {
    // println!("Executing {:?}", cmd);
    let mut items_to_append = extract_items(stacks.entry(cmd.from.to_string()).or_default(), cmd.amount as usize, move_in_order);
    let stack_to = stacks.entry(cmd.to.to_string()).or_default();
    stack_to.append(&mut items_to_append);
    // if !move_in_order {
    //   for _x in 0..cmd.amount {
    //     let item = stack_from.pop().expect("Stack is empty!");
    //     // println!("Moving {} from {} to {}", item, cmd.from, cmd.to);
    //     stack_to.push(item);
    //   }
    // }
    // else {
    //   let mut items_to_move = stack_from.split_off(stack_from.len() - cmd.amount as usize);
    //   stack_to.append(&mut items_to_move);
    //   // let last_index = stacks[cmd.from as usize].len()-1;
    //   // let items: Vec<char> = stacks[cmd.from as usize][last_index..cmd.amount as usize].into();
    //   // for item in items.iter(){
    //   //   stacks[cmd.to as usize].push(*item);
    //   // }
    // }  

    // dbg!(&stacks);  
  });
  for index in 0..stacks.len() {
    let contents = stacks.get(&index.to_string()).expect("Stack not found");
    let top = contents.last();
    match top {
      Some (c) => result.push(*c),
      None => {}
    }
  }
  println!("Final stack top {}", result)
}

fn extract_items(vec: &mut Vec<char>, amount: usize, in_order: bool) -> Vec<char> {
  if in_order {
    vec.split_off(vec.len() - amount)
  }
  else {
    let mut items:Vec<char> = vec![];
    for _x in 0..amount {
      let item = vec.pop().expect("Stack is empty!");
      // println!("Moving {} from {} to {}", item, cmd.from, cmd.to);
      items.push(item);
    }
    items
  }
}

fn setup_stacks (diagram:&str) -> HashMap<String, Vec<char>> {
  let mut stacks = HashMap::new();
  let mut lines: Vec<&str> = diagram.split("\n").collect();
  let last_line = lines.pop().expect("Invalid stack diagram");
  let max_queue_number = last_line
    .trim()
    .split(" ")
    .last()
    .map(|x| { u32::from_str_radix(x, 10).expect("Invalid stack number (last)") })
    .expect("Invalid max queue");

  for i in 0..max_queue_number {
    stacks.insert(i.to_string(), Vec::new());
  }
  lines.reverse();
  lines.iter().for_each(|stack_row| {
    let mut index:usize = 0;
    let mut stack_index = 0;
    while index+3 <= stack_row.len() {
      let code = stack_row[index..index+3].chars().nth(1);
      match code {
          Some (letter) => {
            if letter != ' ' {
              let stack_contents = stacks.entry(stack_index.to_string()).or_insert(Vec::new());
              stack_contents.push(letter)
            }
          }
          _ => {}
      }      
      stack_index += 1;
      index += 4;
    }
  });
  // println!("Stacks {:?} {:?}", max_queue_number, stacks);
  stacks
}

fn parse_commands (commands:&str) -> Vec<Command> {
  commands.split("\n").map(|cmd| {
    let parts: Vec<&str> = cmd.split(" ").collect();
    Command { 
      amount: u32::from_str_radix(parts[1], 10).expect("Invalid instruction (amount)"),
      from : u32::from_str_radix(parts[3], 10).expect("Invalid instruction (origin)") - 1,
      to: u32::from_str_radix(parts[5], 10).expect("Invalid instruction (to)") - 1
    }
  }).collect()
}

#[derive(Debug)]
struct Command {
  amount: u32,
  from: u32,
  to: u32
}