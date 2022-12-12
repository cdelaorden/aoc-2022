use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum HeadMovement {
  Right,
  Left,
  Up,
  Down
}

type Pos2D = (i32, i32);

pub fn rope_bridge(data: &str) {
  let head_movements = parse_movements(data);
  // println!("{:?}", head_movements);
  let tail_positions = follow_head(&head_movements);
  println!("Part One. Tail positions {}", tail_positions);
  println!("Part Two. Last knot positions {}", get_latest_knot_positions(&head_movements));
}

fn get_latest_knot_positions(movements: &Vec<HeadMovement>) -> u32 {
  let mut head_pos:Pos2D = (0, 0);
  let mut tail_pos:Vec<Pos2D> = Vec::new();
  let mut last_knot_positions:HashSet<Pos2D> = HashSet::new();
  // create tail
  for _i in 0..9 {
    tail_pos.push(head_pos.clone());
  }
  // add starting pos to visited
  last_knot_positions.insert((0, 0));
  assert_eq!(tail_pos.len(), 9);

  for mov in movements {
    head_pos = apply_movement(&head_pos, mov);
    for i in 0..tail_pos.len() {
      let knot_pos = tail_pos[i];
      if i == 0 {
        // first follow head
        tail_pos[i] = follow(&head_pos, &knot_pos);
      }
      else {
        // follow previous
        tail_pos[i] = follow(&tail_pos[i-1], &knot_pos)
      }
      if i == tail_pos.len() - 1 {
        // visit position only for last knot
        last_knot_positions.insert(tail_pos[i].clone());
      }
    }
  }

  last_knot_positions.len() as u32
}

fn follow_head(movements: &Vec<HeadMovement>) -> u32 {
  let mut head_pos:Pos2D = (0, 0);
  let mut tail_pos:Pos2D = (0, 0);
  // we can create a set where keys are tuples, pretty nice!
  let mut positions: HashSet<Pos2D> = HashSet::new();
  positions.insert(head_pos);  
  for mov in movements {
    // println!("Head pos {:?}. Move {:?}", head_pos, mov);
    // move head
    head_pos = apply_movement(&head_pos, mov);
    // move tail if needed
    tail_pos = follow(&head_pos, &tail_pos);
    positions.insert(tail_pos);              
  }
  positions.len() as u32
}

fn apply_movement(pos: &Pos2D, mov:&HeadMovement) -> Pos2D {
  let mut new_pos = pos.clone();
  match mov {
    HeadMovement::Right => new_pos.0 = new_pos.0 + 1,
    HeadMovement::Left => new_pos.0 = new_pos.0 - 1,
    HeadMovement::Up => new_pos.1 = new_pos.1 - 1,
    HeadMovement::Down => new_pos.1 = new_pos.1 + 1,
  }
  new_pos
}

fn follow(source: &Pos2D, follower: &Pos2D) -> Pos2D {
  let mut new_pos = follower.clone();
  if !is_adjacent(&source, &follower) {
    if source.0 < follower.0 {
      // move left
      new_pos.0 -= 1;
    }
    else if source.0 > follower.0 {
      // move right
      new_pos.0 += 1;
    }
    if source.1 < follower.1 {
      // move up
      new_pos.1 -= 1;
    }
    else if source.1 > follower.1 {
      // move down
      new_pos.1 += 1;
    }
    // println!("Move tail, new pos {:?}", tail_pos);
    // record position
    // positions.insert(tail_pos);    
  }
  new_pos
}

fn is_adjacent(a:&Pos2D, b:&Pos2D) -> bool {
  (a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1
}

fn parse_movements(data: &str) -> Vec<HeadMovement> {
  let mut movements = Vec::new();
  for line in data.split_terminator("\n").into_iter() {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let dir = parts[0].chars().nth(0).expect("Invalid movement");
    let amount = i32::from_str_radix(parts[1], 10).expect("Invalid amount");
    match (dir, amount) {
        ('R', amount) => { 
          movements.append(repeat_n(HeadMovement::Right, amount).as_mut()) 
        },
        ('U', amount) => movements.append(repeat_n(HeadMovement::Up, amount).as_mut()),
        ('L', amount) => movements.append(repeat_n(HeadMovement::Left, amount).as_mut()),
        ('D', amount) => movements.append(repeat_n(HeadMovement::Down, amount).as_mut()),
        _ => { }

    }
  }
  movements
}

fn repeat_n (mov:HeadMovement, times:i32) -> Vec<HeadMovement> {
  let mut result: Vec<HeadMovement> = Vec::new();
  for _x in 0..times {
    result.push(mov);
  }
  result
}