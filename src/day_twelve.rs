use std::collections::{HashSet, VecDeque};

pub fn hill_climbing_algorithm (data: &str) {
  let map = parse(&data);
  // println!("Map {:#?}", map);
  // // part 1
  let min_distance = find_path(&map);
  println!("Min distance is {:?}", min_distance);
}

fn find_path(
  map: &HeightMap

) -> Option<usize> {
  let mut visited: HashSet<Point> = HashSet::new();
  
  let mut queue = VecDeque::new();
  let current = map.start;
  queue.push_back((current, Vec::new()));

  let is_allowed = |to_height:u8, from_height:u8| {
    to_height <= from_height + 1
  };

  while !queue.is_empty() {
    let (current, history) = queue.pop_front().unwrap();
    if (current == map.end) {
      return Some(history.len());
    }
    // println!("Checking {:?} with history {:?}", current, history);

    let current_height = map.data[current.y][current.x];

    let mut check_neighbour = |x:usize, y:usize| {
      if x >= map.data[0].len()
        || y >= map.data.len()
        || !is_allowed(map.data[y][x], current_height)
        || visited.contains(&Point { x, y })
      {
        return;  
      }
      visited.insert(Point::new(x, y));
      let mut new_history = history.clone();
      new_history.push(current);
      queue.push_back((Point::new(x, y), new_history));
    };

    check_neighbour(current.x + 1, current.y);
    check_neighbour(current.x.wrapping_sub(1), current.y);
    check_neighbour(current.x, current.y + 1);
    check_neighbour(current.x, current.y.wrapping_sub(1));
  }

  None
}

#[derive(Debug, Clone)]
struct HeightMap {
  data: Vec<Vec<u8>>,
  start: Point,
  end: Point
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
  x: usize,
  y: usize
}
impl Point {
  fn new(x: usize, y: usize) -> Self {
      Self { x, y }
  }
}

fn parse(data:&str) -> HeightMap {
  let mut out = Vec::new();
  let mut start = Point::new(0, 0);
  let mut end = Point::new(0, 0);

  for i in data.lines() {
    let mut row = Vec::new();
    for j in i.chars() {
      match j {
        'S' => {
          row.push(0);
          start = Point::new(row.len() - 1, out.len());
        },
        'E' => {
          row.push(25);
          end = Point::new(row.len() - 1, out.len());
        },
        _ => row.push(j as u8 - 97)
      }
    }
    out.push(row);
  }
  HeightMap { data: out, start: start, end: end }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn it_parses_height_map () {
    let m = parse("Sabqponm\n\
    abcryxxl\n\
    accszExk\n\
    acctuvwj\n\
    abdefghi");
    // S is 0 (a)
    assert_eq!(m.data[0][0], 0);
    // E is 25 (z)
    assert_eq!(m.data[2][4], 'z' as u8 - 97);
  }
}