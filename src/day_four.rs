type Section = (u32, u32);


pub fn camp_cleanup(data:String) -> () {
  let pairs: Vec<(Section, Section)> = data.split_terminator("\n")
  .map(|line| {
    let sections: Vec<&str> = line.split(",").collect();
    let elf1_sections = parse_section(sections[0]);
    let elf2_sections = parse_section(sections[1]);
    return (elf1_sections, elf2_sections);

  }).collect();
  let total: i32 = pairs.iter().fold(0, |acc, section_pair|{
    if is_contained(section_pair.0, section_pair.1) || is_contained(section_pair.1, section_pair.0) {
      acc + 1
    }
    else {
      acc
    }
  });
  println!("Total fully contained {}", total);
}

fn parse_section (x: &str) -> (u32, u32) {
  let numbers: Vec<u32> = x.split("-").map(|x| u32::from_str_radix(x, 10).expect("Invalid section number!"))
    .collect();
  return (numbers[0], numbers[1]);
}

fn is_contained (s: Section, in_section: Section) -> bool {
  return s.0 >= in_section.0 && s.1 <= in_section.1
}