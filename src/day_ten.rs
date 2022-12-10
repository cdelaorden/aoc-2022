// https://adventofcode.com/2022/day/10
// Cathode-Ray Tube

#[derive(Debug)]
enum Instruction {
  Noop,
  Add(i32)
}

pub fn cathode_ray_tube(data:&str) {
  let instructions = parse_instructions(data);
  // println!("Program {:?}", instructions);
  println!("Output after 20 cycles {}", run(&instructions, 20));
  println!("Signal strength {}", get_signal_strength_sum(&instructions));
  println!("--------");
  draw_crt(&instructions);
} 

fn get_signal_strength_sum (instructions:&Vec<Instruction>) -> i32 {
  let sample_at = [20, 60, 100, 140, 180, 220];
  let mut strengths = [0, 0, 0, 0, 0, 0];
  for (i, cycles) in sample_at.iter().enumerate() {
    let strength = run(instructions, *cycles);
    strengths[i] = strength * *cycles as i32;
  }
  strengths.iter().sum()
}

fn draw_crt (instructions:&Vec<Instruction>) -> () {
  let mut draw_index = 0;
  let mut pixels: Vec<char> = Vec::new();
  for cycle in 0..220 {
    let x = run(instructions, cycle);
    let sprite = [x-1, x, x+1];
    println!("Drawing: {}. X: {:?}", draw_index, sprite);
    if sprite.contains(&(&draw_index % 40)) {
      pixels.push('#')
    }
    else {
      pixels.push('.')  
    }    
    draw_index += 1;
  }
  // draw lines
  for i in 0..5 {
    println!("{}", &String::from_iter(pixels[i*40..(i+1)*40].iter()));   
  }
}

fn run(instructions:&Vec<Instruction>, num_cycles:u32) -> i32 {
  let mut x: i32 = 1;
  let mut current_index = 0;
  let mut wait = 0;
  let mut next_value: i32 = 1;
  for _i in 0..num_cycles {
    // println!("Cycle {}. X: {}. Wait: {}. Next: {}", _i, x, wait, next_value);
    if wait > 0 {
      wait -= 1;      
    }
    else {
      x = next_value;
      match instructions.get(current_index) {
        Some(Instruction::Add(value)) => {
          wait = 1;
          next_value = x + value;
        },
        Some(Instruction::Noop) => {          
          next_value = x;
        },
        None => break
      }
      current_index += 1;
    }
  }
  x
}

fn parse_instructions(data:&str) -> Vec<Instruction> {
  data.split_terminator("\n").into_iter().map(|line|{
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts[0] {
      "noop" => Instruction::Noop,
      "addx" => Instruction::Add(i32::from_str_radix(parts[1], 10).expect("Invalid addx value")),
      _ => panic!("Unknown instruction")
    }
  }).collect()
}