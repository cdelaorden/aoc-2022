pub fn get_top_of_stacks(data:String) -> () {
  let parts: Vec<&str> = data.split("\n\n").collect();
  let diagram = parts.first().expect("Diagram not found in input file");
  let instructions = parts.last().expect("Move instructions not found in input file");
  // println!("Diagram {:#?}", diagram);
  let stacks = setup_stacks(diagram);
  let commands = parse_commands(instructions);
  interpret(stacks, commands);
  // println!("Instructions {:#?}", parse_commands(instructions));
}

fn interpret(mut stacks: Vec<Vec<char>>, commands: Vec<Command>) -> () {
  let mut result: String = String::new();
  commands.iter().for_each(|cmd| {
    for _x in 0..cmd.amount {
      let item = stacks[cmd.from as usize].pop().expect("Stack is empty!");
      // println!("Moving {} from {} to {}", item, cmd.from, cmd.to);
      stacks[cmd.to as usize].push(item);
    }
  });
  for i in 0..stacks.len() {
    let top = stacks[i].last();
    match top {
      Some (c) => result += &c.to_string(),
      None => {}
    }
  }
  println!("Final stack top {}", result)
}

fn setup_stacks (diagram:&str) -> Vec<Vec<char>> {
  let mut stacks: Vec<Vec<char>> = Vec::new();
  let mut lines: Vec<&str> = diagram.split("\n").collect();
  let last_line = lines.pop().expect("Invalid stack diagram");
  let max_queue_number = last_line
    .trim()
    .split(" ")
    .last()
    .map(|x| { u32::from_str_radix(x, 10).expect("Invalid stack number (last)") })
    .expect("Invalid max queue");

  for _i in 0..max_queue_number {
    stacks.push(Vec::new())
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
              stacks[stack_index].push(letter)
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