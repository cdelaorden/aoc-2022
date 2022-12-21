use regex::Regex;
use std::collections::{BinaryHeap, HashMap, HashSet};

// A candidate is the list of valves and the total flow produced in the end AND time left
type PathCandidate<'a> = (Vec<&'a str>, i32);

pub fn proboscidea_volcanum(data: &str) {
    let mut cave = parse(data);
    make_distance_matrix(&mut cave);
    // println!("{:#?}", cave.valves);
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
    let mut cached_paths : HashMap<&str, PathCandidate>= HashMap::new();
    let best_path = find_best_path(
        cave,
        &nodes_with_flow,
        &start_s,
        30,
        &vec![],
        &mut cached_paths
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
    cache: &mut HashMap<&str, PathCandidate>
) -> PathCandidate<'a> {
    // println!("find best path from {} with {} min left. to-visit: {:?}, curr_path: {:?}",
    //     start,
    //     minutes,
    //     to_visit,
    //     path
    // );
    
    // store all generated paths as Vec of valve ids
    let mut candidates: Vec<PathCandidate> = Vec::new();

    for v in to_visit {        
        let distance = cave.distance_matrix[start][*v];
        if distance >= minutes {
            continue;
        }
        let valve = cave.valves.get(*v).unwrap();
        let min_left = minutes - distance - 1;
        let flow = valve.flow_rate as i32 * min_left;
        let remaining = &to_visit
            .iter()
            .filter(|x| *x != v)
            .map(|v| *v)
            .collect();

        let mut next_path = path.clone();
        next_path.push(&v[..]);
        let full_path = find_best_path(
            cave, 
            remaining, 
            &v[..], 
            min_left, 
            &next_path,
            cache
        );
        
        candidates.push((full_path.0, full_path.1 + flow ));
    }
    let mut best_path = (path.clone(), 0);
    for candidate in candidates {
        // println!("Checking candidate {:#?}", candidate);        
        if candidate.1 > best_path.1 {            
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
                let is_better_distance = distances_from
                    .get(neighbour)
                    .map_or(true, |&d| d > new_dist);
                if is_better_distance {
                    distances_from.insert(neighbour.clone(), new_dist);
                    to_visit.push((neighbour.clone(), new_dist));
                }
            }
        }
    }
}

#[test]
fn distance_matrix_returns_correct_distances(){
    let mut cave = parse("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II");
    make_distance_matrix(&mut cave);
    assert_eq!(cave.distance_matrix["AA"]["DD"], 1);
    assert_eq!(cave.distance_matrix["AA"]["EE"], 2);
    assert_eq!(cave.distance_matrix["AA"]["FF"], 3);
    assert_eq!(cave.distance_matrix["FF"]["HH"], 2);
    assert_eq!(cave.distance_matrix["AA"]["HH"], 5);
}

fn parse(data: &str) -> Cave {
    let mut out = Cave {
        valves: HashMap::new(),
        distance_matrix: HashMap::new(),
    };
    let valve_ids_regex = Regex::new(r"([A-Z]{2})").unwrap();
    let flow_rate_regex = Regex::new(r"flow rate=(\d+)").unwrap();
    for line in data.lines().into_iter() {
        if line.trim().is_empty() { continue }
        let mut line_s = line.trim().split(";").into_iter();
        let def = line_s.next().expect("Definition missing");
        let valve_id = &def.strip_prefix("Valve ").expect("Line does not start with Valve ")[0..2];
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
