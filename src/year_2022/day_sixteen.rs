use std::collections::{HashMap, HashSet, BinaryHeap};
use regex::Regex;

pub fn proboscidea_volcanum(data:&str) {
    let mut cave = parse(data);
    make_distance_matrix(&mut cave);
    println!("{:?}", cave);
    part_one_most_pressure(&cave);
}

fn part_one_most_pressure(cave: &Cave) {
    // generate all possible paths
    generate_path(cave);
    // select best
}

fn generate_path(cave:&Cave) {
    let mut total_release = 0;
    let mut min_left = 30;
    let mut release_per_minute = 0;    
    let mut from = String::from("AA");
    let mut paths = Vec::new();
    let mut open_valves: HashSet<String> = HashSet::new();
    loop {        
        if min_left == 0 {
            break
        }
        total_release += release_per_minute;
        // sort target caves by release_flow
        let valve = cave.valves.get(&from).unwrap();
        if valve.flow_rate > 0 && !open_valves.contains(&from) {
            println!("Opening {} for {} per minute", &from, valve.flow_rate);
            release_per_minute += valve.flow_rate;
            min_left = min_left - 1;
            open_valves.insert(from.clone());
        }
        let mut targets: Vec<String> = valve.tunnels
            .iter()
            .filter(|&t| open_valves.contains(t) == false)
            .cloned()
            .collect();
            
        if targets.len() == 0 {
            //out of targets
            break;
        }
        {
            targets.sort_by(|id1, id2| {
                    cave.valves.get(id1).unwrap().flow_rate.cmp(&cave.valves.get(id2).unwrap().flow_rate)
                });
            targets.reverse();        
            min_left -= 1;
            from = targets[0].clone();            
            paths.push(String::from(targets[0].clone()));
        }
    }
    println!("From AA, best is {:?} with total release {}", paths, total_release);
}

#[derive(Debug, Hash, Clone)]
struct Valve {
    id: String,
    flow_rate: u32,
    tunnels: Vec<String>
}

#[derive(Debug)]
struct Cave {
    valves: HashMap<String, Valve>,
    distance_matrix: HashMap<String, HashMap<String, i32>>
}
type Visit = (String, i32);

fn make_distance_matrix(cave:&mut Cave) {
    for start in cave.valves.keys() {
        let distances_from = cave.distance_matrix.entry(start.clone())
            .or_insert(HashMap::new());
        let mut to_visit: BinaryHeap<Visit> = BinaryHeap::new();
        let mut visited: HashSet<String> = HashSet::new();

        to_visit.push((start.clone(), 0));

        while let Some((valve, distance)) = to_visit.pop() {
            // already visited
            if !visited.insert(valve.clone()) {
                continue;
            }
            let neighbours = &cave.valves.get(&valve).unwrap().tunnels;
            for neighbour in neighbours {
                let new_dist = distance + 1;
                let is_better_distance = distances_from.get(neighbour)
                    .map_or(true, |&d| d > new_dist);
                if is_better_distance {
                    distances_from.insert(neighbour.clone(), new_dist);
                    to_visit.push((neighbour.clone(), new_dist));
                }
            }
        }
    }    
}


fn parse(data:&str) -> Cave {
    let mut out = Cave {
        valves: HashMap::new(),
        distance_matrix: HashMap::new()
    };
    let valve_ids_regex = Regex::new(r"([A-Z]{2})").unwrap();
    let flow_rate_regex = Regex::new(r"flow rate=(\d+)").unwrap();
    for line in data.lines().into_iter() {
        let mut line_s = line.split(";").into_iter();   
        let def = line_s.next().expect("Definition missing");
        let valve_id = &def.strip_prefix("Valve ").unwrap()[0..2];
        let flow_rate = flow_rate_regex.captures(line).unwrap()[1].parse::<u32>().unwrap();
        let connections = line_s.next().expect("Tunnel defs missing");
        out.valves.insert(valve_id.to_string(), 
            Valve {
                id: valve_id.to_owned(),
                flow_rate,
                tunnels: valve_ids_regex
                    .find_iter(connections)
                    .into_iter()
                    .map(|m| connections[m.start()..m.end()].to_string())
                    .collect()                    
            });        
    }
    out
}