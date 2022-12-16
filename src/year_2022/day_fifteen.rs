// https://adventofcode.com/2022/day/15
use std::collections::{HashSet};
use regex::Regex;

pub fn beacon_exclusion_zone (data:&str) {
    let map = parse_data(data);
    // part 1
    get_positions_beacon_not_present_in_row(&map, 2000000);
}

fn get_positions_beacon_not_present_in_row(map: &Map, row_y: i32) {
    // TODO: 
    //  - keep minX, manX
    //  - keep minY, maxY
    // use sensor info distance to store all "covered" points in a HashSet
    // for a row at y, start at minX until minY and add
    // all positions which are covered above
    let mut covered = 0;
    for x in map.min_x..=map.max_x {
        let p = Point { x, y: row_y };
        if is_not_empty(map, p) { continue };
        for sensor_info in map.sensors.iter() {
            if is_covered_by_sensor(&sensor_info, p) {
                covered += 1;
                break;
            }
        }
    }
    println!("Total covered at row {}: {}", row_y, covered);
}

fn is_covered_by_sensor(sensor: &SensorInfo, at: Point) -> bool {
    // it's in the radius?
    manhattan_distance(&sensor.sensor_at, &at) <= sensor.distance
}

fn is_not_empty(map: &Map, at: Point) -> bool {
    map.beacons.contains(&at) || map.sensor_locations.contains(&at)
}

fn parse_data(data:&str) -> Map {
    let mut out = Map { 
        sensors: Vec::new(),
        sensor_locations: HashSet::new(),
        beacons: HashSet::new(),
        min_x: 0,
        max_x: 0,
        min_y: 0,
        max_y: 0
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
        out.sensor_locations.insert(sensor_at);    
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
        out.beacons.insert(beacon_at);
        let line_min_x = sensor_at.x.min(beacon_at.x);
        if line_min_x < out.min_x {
            out.min_x = line_min_x;
        }
        let line_max_x = sensor_at.x.max(beacon_at.x);
        if line_max_x > out.max_x {
            out.max_x = line_max_x;
        }
        let line_min_y = sensor_at.y.min(beacon_at.y);
        if line_min_y < out.min_y {
            out.min_y = line_min_y;
        }
        let line_max_y = sensor_at.y.max(beacon_at.y);
        if line_max_y > out.max_y {
            out.max_y = line_max_y;
        }

    }
    out
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    (p2.x - p1.x).abs() + (p2.y - p1.y).abs()
}

#[derive(Debug)]
struct Map {
    // all parsed sensor info
    sensors: Vec<SensorInfo>,    
    sensor_locations: HashSet<Point>,
    // all beacons present just in case
    beacons: HashSet<Point>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32
    
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
