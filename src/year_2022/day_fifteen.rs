// https://adventofcode.com/2022/day/15
use std::collections::{HashSet, HashMap};
use regex::Regex;

pub fn beacon_exclusion_zone (data:&str) {
    let mut map = parse_data(data);
    println!("{:?}", map);
}

#[derive(Debug)]
struct Map {
    // all parsed sensor info
    sensors: Vec<SensorInfo>,
    // list of sensor info indexed by y_pos (part 1)
    sensors_y: HashMap<i32, Vec<SensorInfo>>,
    // all beacons present just in case
    beacons: HashSet<Point>,
    
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct SensorInfo {    
    sensor_at: Point,
    beacon_at: Point,
    distance: i32
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32
}

fn parse_data(data:&str) -> Map {
    let mut out = Map { 
        sensors: Vec::new(),
        sensors_y: HashMap::new(),
        beacons: HashSet::new(),
    };
    let coords_re = Regex::new(r"x=(?P<x>-?\d+),\sy=(?P<y>-?\d+)").unwrap();
    let mut lines = data.lines().into_iter();
    while let Some(line) = lines.next() {
        let mut coords = coords_re.captures_iter(line);
        let sensor = coords.next().unwrap();
        let sensor_at = Point { 
            x: i32::from_str_radix(&sensor["x"], 10).expect("Invalid sensor x"),
            y: i32::from_str_radix(&sensor["y"], 10).expect("Invalid sensor y")
        };        
        let beacon = coords.next().expect("Missing beacon coords");
        let beacon_at = Point {
            x: i32::from_str_radix(&beacon["x"], 10).expect("Invalid beacon x"),
            y: i32::from_str_radix(&beacon["y"], 10).expect("Invalid beacon y")
        };
        let distance = manhattan_distance(&sensor_at, &beacon_at);
        let sensor_info = SensorInfo {
            beacon_at: beacon_at,
            sensor_at: sensor_at,
            distance
        };
        out.sensors.push(sensor_info);
        let sensor_at_y_pos = out.sensors_y.entry(sensor_at.y).or_default();
        sensor_at_y_pos.push(sensor_info);
        out.beacons.insert(beacon_at);
    }
    out
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    (p2.x - p1.x).abs() + (p2.y - p1.y).abs()
}