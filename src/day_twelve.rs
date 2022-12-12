use std::collections::{HashSet, VecDeque};

pub fn hill_climbing_algorithm (data: &str) {
  let mut map = parse(&data);
  // part 1
  let min_distance = find_path(
    &map, 
    |p| { map.end == p },
    false
  );
  println!("Part One. Min distance is {:?}", min_distance);
  // part 2 start from end, and define "ending" as any point with height 0
  // since the find_path gets the shortest path already
  // it also means that the height must be inverted!
  map.start = map.end;
  let min_distance_to_an_a = find_path(
    &map, 
    |p| { map.data[p.y][p.x] == 0},
    true
  );
  println!("Part Two. Min distance from any a {:?}", min_distance_to_an_a);

}

fn find_path(
  map: &HeightMap, 
  is_solution: impl Fn(Point) -> bool,
  invert_height_check: bool
) -> Option<usize> {
  // keeps track of already visited points
  let mut visited: HashSet<Point> = HashSet::new();
  // stores pending points to visit
  let mut queue = VecDeque::new();
  let current = map.start;
  queue.push_back((current, Vec::new()));

  // checks height
  let is_allowed = |to_height:u8, from_height:u8| {
    if invert_height_check {
      from_height <= to_height + 1
    }
    else {
      to_height <= from_height + 1
    }
  };
  // loop while there are points to check in the queue
  while !queue.is_empty() {
    let (current, history) = queue.pop_front().unwrap();
    if is_solution(current) {
      // println!("Got solution {:?}", history);
      return Some(history.len());
    }

    let current_height = map.data[current.y][current.x];
    // closure to be able to use visited Set, removes points
    // outside the map. it has to be mutable because it's re-defined on each loop
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
// without Hash trait it cannot be added to HashSet
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