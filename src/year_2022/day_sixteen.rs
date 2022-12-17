use std::collections::HashMap;
use regex::Regex;

pub fn proboscidea_volcanum(data:&str) {
    let mut cave = parse(data);
    println!("Cave {:?}", cave)
}

#[derive(Debug, Hash, Clone)]
struct Valve {
    id: String,
    flow_rate: u32,
    tunnels: Vec<String>
}

#[derive(Debug)]
struct Cave {
    valves: HashMap<String, Valve>
}

fn parse(data:&str) -> Cave {
    let mut out = Cave {
        valves: HashMap::new()
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