type TreeGrid = Vec<Vec<u32>>;

pub fn treetop_tree_house (data:&str) {
  let grid = parse_grid(data);
  // println!("Grid is {:?}", grid);
  println!("Part one. Visible trees: {}", get_visible_tree_count(&grid));
}

fn get_visible_tree_count (grid: &TreeGrid) -> u32 {
  let mut visible_total = 0;
  let mut visiblity_grid: Vec<Vec<char>> = Vec::new();
  for x in 0..grid.len() {
    visiblity_grid.push(Vec::new());
    for y in 0..grid[x].len() {         
      if is_invisible(x, y, grid) {
        println!("Tree at {} {} height {} is INVISIBLE", x, y, grid[x][y]);
        visiblity_grid[x].push('I');        
      }      
      else {
        println!("Tree at {} {} height {} is VISIBLE", x, y, grid[x][y]);
        visiblity_grid[x].push('V');
        visible_total += 1;
      }
    }
  }
  // println!("Visiblity {:#?}", visiblity_grid);
  pretty_print_grid(visiblity_grid);
  visible_total
}

fn is_invisible (x: usize, y:usize, grid: &TreeGrid) -> bool {
  let grid_size = grid.len();
  let tree_height = grid[x][y];
  if x == 0 || x == grid_size - 1 { false }
  else if y == 0 || y == grid_size - 1 { false }
  else {    
    let inv_left = grid[x][0..y].iter().find(|h| h >= tree_height);
    inv_left.is_some()
  }

}

fn parse_grid(data:&str) -> Vec<Vec<u32>> {
  let mut grid: Vec<Vec<u32>> = Vec::new();
  for (x, line) in data.split_terminator("\n").enumerate() {
    grid.push(Vec::new());
    for (_y,  height_char) in line.chars().enumerate() {
      let height = height_char.to_digit(10).expect("Invalid tree height");
      grid[x].push(height);
    }
  }
  grid
}

fn pretty_print_grid<T> (grid: Vec<Vec<T>>) where T: std::fmt::Display {
  for x in 0..grid.len() {
    let mut row = String::from("");
    for y in 0..grid.len() {
      row.push_str(&grid[x][y].to_string());
    }
    println!("{}", row);
  }
}