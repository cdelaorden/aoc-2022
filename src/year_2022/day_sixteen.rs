use regex::Regex;
use std::collections::{BinaryHeap, HashMap, HashSet};

// A candidate is the list of valves and the total flow produced in the end AND time left
type PathCandidate<'a> = (Vec<&'a str>, i32);

pub fn proboscidea_volcanum(data: &str) {
    let mut cave = parse(data);
    make_distance_matrix(&mut cave);
    // println!("{:?}", cave);
    part_one_most_pressure(&cave);
}

fn part_one_most_pressure(cave: &Cave) {
    // generate all possible paths
    let nodes_with_flow = cave
        .valves
        .iter()
        .filter(|v| v.1.flow_rate > 0)
        .map(|(id, _)| &id[..] )
        .collect();
    let start_s = String::from("AA");
    let _start = cave.valves.get(&start_s).unwrap();

    let best_path = find_best_path(
        cave,
        &nodes_with_flow,
        &start_s,
        30,
        &vec![],
    );
    // select best
    println!("Best path {:?}", best_path);
}

fn find_best_path<'a>(
    cave: &Cave,
    to_visit: &Vec<&'a str>,
    start: &'a str,
    minutes: i32,
    path: &Vec<&'a str>,
) -> PathCandidate<'a> {
    // println!("find best path from {} with {} min left. to-visit: {:?}, path: {:?}",
    //     start,
    //     minutes,
    //     to_visit,
    //     path
    // );
    
    // store all generated paths as Vec of valve ids
    let mut candidates: Vec<PathCandidate> = Vec::new();
    let time_left = minutes;

    for v in to_visit {
        let valve = cave.valves.get(*v).unwrap();
        let distance = cave.distance_matrix[start][*v];
        if distance >= time_left {
            continue;
        }
        let min_left = minutes - distance - 1;
        let flow = valve.flow_rate as i32 * min_left;
        let remaining = &to_visit
            .iter()
            .filter(|x| *x != v)
            .map(|v| v.clone())
            .collect();

        let mut next_path = path.clone();
        next_path.push(*v);
        let full_path = find_best_path(
            cave, 
            remaining, 
            &v[..], 
            min_left, 
            &next_path
        );
        // println!("Recursive with {} at distance {} with flow {} and min left {}", v, distance, flow, min_left);
        // println!("Result {:?}", full_path);
        // let mut finished_path = path.clone();
        // finished_path.extend(full_path.0);
        candidates.push((full_path.0, full_path.1 + flow ));
    }
    let mut best_path = (path.clone(), 0);
    for candidate in candidates {
        // println!("Checking candidate {:#?}", candidate);        
        if candidate.1 > best_path.1 {            
            // candidate.0.insert(0, &preprend_path);
            best_path = candidate;
        }
    }
    
    // if path.len() == 0 {
    //     // prepend start
    //     best_path.0.insert(0, start);
    // }
    best_path

    // from all paths, get the best one in terms of flow per minute
}

#[derive(Debug, Hash, Clone)]
struct Valve {
    id: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

#[derive(Debug)]
struct Cave {
    valves: HashMap<String, Valve>,
    distance_matrix: HashMap<String, HashMap<String, i32>>,
}
type Visit = (String, i32);

fn make_distance_matrix(cave: &mut Cave) {
    for start in cave.valves.keys() {
        let distances_from = cave
            .distance_matrix
            .entry(start.clone())
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
                let improve_distance = distances_from
                    .get(neighbour)
                    .map_or(true, |&d| d > new_dist);
                if improve_distance {
                    distances_from.insert(neighbour.clone(), new_dist);
                    to_visit.push((neighbour.clone(), new_dist));
                }
            }
        }
    }
}

fn parse(data: &str) -> Cave {
    let mut out = Cave {
        valves: HashMap::new(),
        distance_matrix: HashMap::new(),
    };
    let valve_ids_regex = Regex::new(r"([A-Z]{2})").unwrap();
    let flow_rate_regex = Regex::new(r"flow rate=(\d+)").unwrap();
    for line in data.lines().into_iter() {
        let mut line_s = line.split(";").into_iter();
        let def = line_s.next().expect("Definition missing");
        let valve_id = &def.strip_prefix("Valve ").unwrap()[0..2];
        let flow_rate = flow_rate_regex.captures(line).unwrap()[1]
            .parse::<u32>()
            .unwrap();
        let connections = line_s.next().expect("Tunnel defs missing");
        out.valves.insert(
            valve_id.to_string(),
            Valve {
                id: valve_id.to_owned(),
                flow_rate,
                tunnels: valve_ids_regex
                    .find_iter(connections)
                    .into_iter()
                    .map(|m| connections[m.start()..m.end()].to_string())
                    .collect(),
            },
        );
    }
    out
}
