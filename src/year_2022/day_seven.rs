use std::{collections::HashMap};

// https://adventofcode.com/2022/day/7
#[derive(Debug)]
enum ParsedLine {  
  CdToRoot,
  CdIn(String),
  CdOut,
  List,
  Dir(String),
  File(String, u32)
}

const TOTAL_SPACE: u32 = 70_000_000;
const MIN_SPACE: u32 = 30_000_000;

pub fn no_space_left (data: &str) {
  let lines: Vec<&str> = data.split_terminator('\n').collect();
  let parsed = parse_lines(lines);
  // println!("# lines {:?}", parsed);
  let tree = walk_tree(parsed);  
  // part one
  calculate_candidates_for_deletion(&tree);
  // part two
  calculate_dir_to_remove(&tree);
}

fn calculate_candidates_for_deletion(tree: &HashMap<String, u32>) {
  let mut total = 0u32;
  // println!("Full tree {:#?}", tree);
  for (_dir, size) in tree.iter() {
    if size <= &100_000 {
      total += size;
    } 
  }
  println!("Part One. Total for delete {}", total)
}

fn calculate_dir_to_remove(tree: &HashMap<String, u32>) {
  let mut sizes_sorted: Vec<u32> = Vec::new();
  let used_size = tree.get("/").expect("Root size not found");
  let unused_size = TOTAL_SPACE - used_size;
  let space_to_free = MIN_SPACE - unused_size;
  // println!("Space needed to free {}", space_to_free);
  for (dir, size) in tree.iter() {
    if dir == "/" { continue };
    sizes_sorted.push(*size);
  }
  sizes_sorted.sort();
  // println!("Sorted sizes {:?}", sizes_sorted);
  for dir_size in sizes_sorted {
    if dir_size >= space_to_free {
      println!("Part Two. Dir to remove size {}", dir_size);
      break;
    }
  }
}

fn walk_tree (tree: Vec<ParsedLine>) -> HashMap<String, u32>{
  let mut result:HashMap<String, u32> = HashMap::new();
  let mut curr_path:Vec<String> = Vec::new();
  for entry in tree {
    match entry {
      ParsedLine::CdToRoot => {
        // curr_path.push(String::from("/"));
        curr_path.clear();
      }
      ParsedLine::CdIn(path) => {      
        // println!("Entering dir {}", path);          
        curr_path.push(String::from(path));
      }
      ParsedLine::CdOut => {
        // println!("Exitting dir {:?}", &curr_path.last());
        curr_path.pop();
      }
      ParsedLine::Dir(_dir_name) => {

      }
      ParsedLine::File(_file_name, file_size) => {
        let mut path_copy = curr_path.clone();
        while path_copy.len() > 0 {
          let full_path = path_copy.join("/");
          let path_info = result.entry(full_path).or_insert(0);
          *path_info += file_size;
          path_copy.pop();
        }        
        // add to root
        let root_dir = result.entry(String::from("/")).or_insert(0);
        *root_dir += file_size;
      }
      ParsedLine::List => {}
    }
  }

  result
}

fn parse_lines(input: Vec<&str>) -> Vec<ParsedLine> {
  let mut result: Vec<ParsedLine> = Vec::new();
  for line in input {
    if line.chars().nth(0) == Some('$') {
      result.push(parse_command(&line[1..]));
    }
    else {
      result.push(parse_entry(line));
    }
  }
  result
}

fn parse_command (data: &str) -> ParsedLine {
  let command_string = String::from(data.trim());
  if command_string[0..2].eq("cd"){
    let target = &command_string[3..];
    if target.trim().eq("/") {
      ParsedLine::CdToRoot
    }
    else if target.eq("..") {
      ParsedLine::CdOut
    }
    else {
      ParsedLine::CdIn(String::from(&command_string[3..]))
    }
  }
  else {
    // must be an ls
    ParsedLine::List
  }
}

fn parse_entry (data: &str) -> ParsedLine {
  let entry_parts: Vec<&str> = data.split_ascii_whitespace().collect();
  if entry_parts.len() != 2 { panic!("Unknown entry") }
  if entry_parts[0] == "dir" {
    ParsedLine::Dir(String::from(entry_parts[1]))
  }
  else {
    ParsedLine::File(String::from(entry_parts[1]), u32::from_str_radix(entry_parts[0], 10).expect("Error parsing size"))
  }
  
}