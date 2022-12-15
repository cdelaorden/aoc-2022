// https://adventofcode.com/2022/day/14
use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
};

pub fn regolith_reservoir(data: &str) {
    // part one
    calculate_sand_units_before_overflow(data);
    // part two
    calculate_sand_units_until_blocked(data);
}

fn calculate_sand_units_before_overflow(data: &str) {
    let mut cave = generate_cave(data, false);
    let mut sand_units = 0;
    // println!("{}", cave);
    loop {
        let rest_point = spawn_sand(&mut cave);
        if rest_point.is_none() {
            break;
        }
        sand_units += 1;
    }
    // clear screen, cursor at 1 1
    // print!("\x1B[2J\x1B[1;1H");
    print!("{}", cave);
    println!("Part One. Spawned sand: {}", sand_units);
}

fn calculate_sand_units_until_blocked(data: &str) {
    let mut cave = generate_cave(data, true);
    // extend floor
    add_infinite_floor(&mut cave);
    let mut sand_units = 0;
    // println!("{:?}", cave);
    loop {
        let rest_point = spawn_sand(&mut cave);
        // println!("Sand {} rest point {:?}", sand_units, rest_point);
        if rest_point.is_none() {
            break;
        }
        sand_units += 1;
    }
    // print!("\x1B[2J\x1B[1;1H");
    print!("{}", cave);
    println!("Part Two. Source blocked after {}", sand_units);
}

fn spawn_sand(cave: &mut Cave) -> Option<Point> {
    let mut curr = cave.source.clone();
    if !is_free_at(cave, &curr) {
        println!("Source blocked!");
        return None;
    }
    loop {
        if is_out_bounds(cave, &curr) {
            println!("OOB");
            break None;
        }
        // keep going down if there are no obstacles
        if is_free_at(
            cave,
            &Point {
                x: curr.x,
                y: curr.y + 1,
            },
        ) {
            curr.y += 1;
        } else if is_free_at(
            cave,
            &Point {
                x: curr.x - 1,
                y: curr.y + 1,
            },
        ) {
            curr.y += 1;
            curr.x -= 1;
        } else if is_free_at(
            cave,
            &Point {
                x: curr.x + 1,
                y: curr.y + 1,
            },
        ) {
            curr.y += 1;
            curr.x += 1;
        } else {
            if is_out_bounds(cave, &curr) {
                break None;
            } else {
                fill_point(cave, &curr, Fill::Sand);
                extend_infinite_floor(cave, &curr);
                break Some(curr);
            }
        }
    }
}

fn generate_cave(data: &str, with_floor: bool) -> Cave {
    let mut out = Cave::new(Point { x: 500, y: 0 }, with_floor);

    let mut lines = data.lines().into_iter();
    while let Some(line) = lines.next() {
        let points: Vec<Point> = line.split(" -> ").map(parse_point).collect();
        fill_rock_path(&mut out, points);
    }
    out
}

fn fill_rock_path(cave: &mut Cave, points: Vec<Point>) -> () {
    let points_iter = points.iter();
    let mut pen_pos: Option<&Point> = None;
    for point in points_iter {
        // generate path and fill each point
        if let Some(origin) = pen_pos {
            if origin.y == point.y {
                for x in min(origin.x, point.x)..=max(origin.x, point.x) {
                    fill_point(cave, &Point { x, y: origin.y }, Fill::Rock);
                    pen_pos = Some(&point);
                }
            } else if origin.x == point.x {
                for y in min(origin.y, point.y)..=max(origin.y, point.y) {
                    fill_point(cave, &Point { x: origin.x, y }, Fill::Rock);
                    pen_pos = Some(&point);
                }
            } else {
                panic!("Got consecutive points with x or y in common");
            }
        } else {
            // fill_point(cave, point, Fill::Rock);
            pen_pos = Some(point);
        }
    }
}

fn add_infinite_floor(cave: &mut Cave) {
    if cave.with_floor {
        let floor_points = vec![
            Point {
                x: cave.min_x - 5,
                y: cave.max_y + 2,
            },
            Point {
                x: cave.max_x + 5,
                y: cave.max_y + 2,
            },
        ];
        fill_rock_path(cave, floor_points);
        cave.floor_y = cave.max_y + 2;
    }
}

fn extend_infinite_floor(cave: &mut Cave, at: &Point) {
    if !cave.with_floor {
        return;
    }
    if at.x == cave.min_x || at.x == cave.max_x {
        let floor_points = vec![
            Point {
                x: at.x - 1,
                y: cave.floor_y,
            },
            Point {
                x: at.x + 1,
                y: cave.floor_y,
            },
        ];
        fill_rock_path(cave, floor_points);
    }
}

fn fill_point(cave: &mut Cave, at: &Point, fill: Fill) {
    cave.occupied_points.insert(*at, fill);
    if cave.with_floor || fill == Fill::Rock {
        // println!("Filled point at {:?} with {:?}. Checking bounds.", at, fill);
        extend_bounds(cave, at);
    }
}

fn extend_bounds(cave: &mut Cave, at: &Point) {
    if at.x < cave.min_x {
        cave.min_x = at.x;
    }
    if at.x > cave.max_x {
        cave.max_x = at.x;
    }
    if at.y >= cave.max_y {
        cave.max_y = at.y;
    }
}

fn is_free_at(cave: &Cave, point: &Point) -> bool {
    return !cave.occupied_points.contains_key(point);
}

fn is_out_bounds(cave: &Cave, point: &Point) -> bool {
    if !cave.with_floor {
        point.x < cave.min_x || point.x > cave.max_x || point.y > cave.max_y
    } else {
        point.y > cave.max_y
    }
}

fn get_fill(cave: &Cave, at: Point) -> Option<Fill> {
    cave.occupied_points.get(&at).copied()
}

fn parse_point(point_str: &str) -> Point {
    let mut coords = point_str.split(",").into_iter();
    let x_coord = coords.next().expect("Missing point x coordinate");
    let y_coord = coords.next().expect("Missing point y coordinate");
    Point {
        x: i32::from_str_radix(x_coord, 10).unwrap(),
        y: i32::from_str_radix(y_coord, 10).unwrap(),
    }
}

#[derive(Debug, Clone)]
struct Cave {
    source: Point,
    occupied_points: HashMap<Point, Fill>,
    min_x: i32,
    max_x: i32,
    max_y: i32,
    with_floor: bool,
    floor_y: i32,
}
impl Cave {
    fn new(sand_source: Point, with_floor: bool) -> Self {
        Cave {
            source: sand_source,
            occupied_points: HashMap::new(),
            min_x: sand_source.x,
            max_x: sand_source.x,
            max_y: sand_source.y,
            with_floor,
            floor_y: sand_source.y + 1,
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.max_y {
            for x in self.min_x..=self.max_x {
                let point = Point { x, y };
                if x == self.min_x {
                    // print line number
                    write!(f, "{}\t", y)?;
                }
                if self.source == point {
                    write!(f, "+")?;
                }
                match get_fill(self, Point { x, y }) {
                    Some(Fill::Rock) => write!(f, "#")?,
                    Some(Fill::Sand) => write!(f, "o")?,
                    None => write!(f, ".")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fill {
    Rock,
    Sand,
}
