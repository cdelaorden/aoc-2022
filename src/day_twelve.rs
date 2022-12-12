pub fn hill_climbing_algorithm (data: &str) {
  let map = parse_map(&data);
  println!("Map {:#?}", map);
}

#[derive(Debug)]
struct Map {
  grid: Vec<Vec<u32>>,
  start_pos: (u32, u32),
  end_pos: (u32, u32)
}


fn parse_map (data: &str) -> Map {
  let mut map = Map {
    grid: vec![],
    start_pos: (0,0),
    end_pos: (0,0)
  };
  data
    .split_terminator("\n")
    .enumerate()
    .for_each(|(x, line)|{
      let mut row = vec![];
      line.chars().enumerate().
      for_each(|(y, c)|{
        match c {
          'S' => {
            map.start_pos = (x.try_into().unwrap(), y.try_into().unwrap());
            row.push('a' as u32);
          }
          'E' => {
            map.end_pos = (x.try_into().unwrap(), y.try_into().unwrap());
            row.push('z' as u32);
          }
          _ => row.push(c as u32)
        }
      });
      map.grid.push(row);      
    });    
    map
}