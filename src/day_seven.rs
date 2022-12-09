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

struct DirWithSize {
  path: String,
  total_size: u32
}

pub fn no_space_left (data: &str) {
  let lines: Vec<&str> = data.split_terminator('\n').collect();
  let parsed = parse_lines(lines);
  // println!("# lines {:?}", parsed);
  let tree = walk_tree(parsed);  
  // part one
  calculate_candidates_for_deletion(tree);
}

fn calculate_candidates_for_deletion(tree:HashMap<String, u32>) {
  // println!("Dir tree {:#?}", tree);
  let mut total = 0u32;
  for (_dir, size) in tree.iter() {
    if size <= &100_000 {
      // println!("Adding dir {} with size {}", dir, size);
      total += size;
    } 
  }
  println!("Total for delete {}", total)
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