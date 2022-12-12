#![allow(dead_code)]
type TreeGrid = Vec<Vec<u32>>;

pub fn treetop_tree_house (data:&str) {
  let grid = parse_grid(data);
  // println!("Grid is {:?}", grid);
  println!("Part One. Visible trees: {}", get_visible_tree_count(&grid));
  println!("Part Two. Best tree score is {}", get_highest_scenic_score(&grid));
}

fn get_visible_tree_count (grid: &TreeGrid) -> u32 {
  let mut visible_total = 0;
  let mut visiblity_grid: Vec<Vec<char>> = Vec::new();
  for x in 0..grid.len() {
    visiblity_grid.push(Vec::new());
    for y in 0..grid[x].len() {         
      if is_invisible(x, y, grid) {
        visiblity_grid[x].push('I');        
      }      
      else {
        visiblity_grid[x].push('V');
        visible_total += 1;
      }
    }
  }
  // Just for debugging
  // pretty_print_grid(visiblity_grid);
  visible_total
}

fn get_highest_scenic_score (grid: &TreeGrid) -> u32 {
  let mut top_score: u32 = 0;
  let mut score_grid: TreeGrid = Vec::new();
  for x in 0..grid.len() {
    score_grid.push(Vec::new());
    for y in 0..grid[x].len() {
      let tree_score = get_tree_scenic_score(x, y, grid);      
      if tree_score > top_score {
        top_score = tree_score;
      }
      score_grid[x].push(tree_score);
    }
  }
  // pretty_print_grid(grid);
  // println!("--------------");
  // pretty_print_grid(&score_grid);
  top_score
}

fn get_tree_scenic_score (x:usize, y:usize, grid: &TreeGrid) -> u32 {
  let tree_height = grid[x][y];
  // the trick here was to `rev`erse the the range so it calculates inside out
  let (left_score, _left_blocked) = grid[x][0..y].iter().rev().fold((0, false), |acc, h| {
    let (acc_height, blocked) = acc;
    if blocked { acc }
    else if h >= &tree_height {
      (acc_height+1, true)
    }
    else { (acc_height+1, false) }
  });
  let (right_score, _right_blocked) = grid[x][y+1..].iter().fold((0, false), |acc, h| {
    let (acc_height, blocked) = acc;
    if blocked { acc }
    else if h >= &tree_height {
      (acc_height+1, true)
    }
    else { (acc_height+1, false) }    
  });
  // same here, reverse
  let (top_score, _top_blocked) = grid[0..x].iter().rev().fold((0, false), |acc, col| {
    let (acc_height, blocked) = acc;
    if blocked { acc }
    else if col[y] >= tree_height {
      (acc_height+1, true)
    }
    else { (acc_height+1, false) }
  });
  let (bottom_score, _bottom_blocked) = grid[x+1..].iter().fold((0, false), |acc, col| {
    let (acc_height, blocked) = acc;
    if blocked { acc }
    else if col[y] >= tree_height {
      (acc_height+1, true)
    }
    else { (acc_height+1, false) }    
  });  
  left_score * right_score * top_score * bottom_score
}

fn is_invisible (x: usize, y:usize, grid: &TreeGrid) -> bool {
  let grid_size = grid.len();
  let tree_height = grid[x][y];
  if x == 0 || x == grid_size - 1 { false }
  else if y == 0 || y == grid_size - 1 { false }
  else {    
    let inv_left = grid[x][0..y].iter().find(|h| **h >= tree_height);
    let inv_right = grid[x][y+1..].iter().find(|h| **h >= tree_height);
    let inv_above = grid[0..x].iter().find(|h| h[y] >= tree_height);
    let inv_below = grid[x+1..].iter().find(|h| h[y] >= tree_height);    
    inv_left.is_some() && inv_right.is_some() && inv_above.is_some() && inv_below.is_some()
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


fn pretty_print_grid<T> (grid: &Vec<Vec<T>>) where T: std::fmt::Display {
  for x in 0..grid.len() {
    let mut row = String::from("");
    for y in 0..grid.len() {
      row.push_str(&grid[x][y].to_string());
    }
    println!("{}", row);
  }
}