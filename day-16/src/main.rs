#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valve {
    name: String,
    flow_rate: i32,
    links: Vec<String>,
}

impl Valve {}

impl From<&str> for Valve {
    fn from(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? ([A-Z, ]+)"
            )
            .unwrap();
        }

        let captures = RE.captures(input).unwrap();

        Valve {
            name: captures.get(1).unwrap().as_str().to_string(),
            flow_rate: captures.get(2).unwrap().as_str().parse().unwrap(),
            links: captures
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|it| it.to_string())
                .collect::<Vec<String>>(),
        }
    }
}

fn main() {
    let input = include_str!("../example");
    let mut valves: Vec<Valve> = input.lines().map(|it| Valve::from(it)).collect();
    let max_time = 30;

    valves.sort_unstable_by(|a, b| b.flow_rate.cmp(&a.flow_rate));

    let valves_idx: HashMap<&String, usize> = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (&v.name, i))
        .collect();

    let good_valves = valves.iter().filter(|valve| valve.flow_rate > 0).count();
    let total_valves = valves.len();

    let mut adj = vec![vec![0usize; 0]; total_valves];
    let mut flow = vec![0i32; total_valves];

    for valve in valves.iter() {
        let i = valves_idx[&valve.name];
        flow[i] = valve.flow_rate;
        for w in valve.links.iter() {
            adj[i].push(valves_idx[w]);
        }
    }

    let aa = valves_idx[&"AA".to_string()];

    let mm = 1 << good_valves;
    let mut values = vec![vec![vec![0; mm]; total_valves]; max_time];

    for t in 1..max_time {
        for i in 0..total_valves {
            let ii = 1 << i;
            for x in 0..mm {
                let mut current = values[t][i][x];
                if ii & x != 0 && t >= 2 {
                    current = current.max(values[t - 1][i][x - ii] + flow[i] * t as i32);
                }

                for &j in adj[i].iter() {
                    current = current.max(values[t - 1][j][x]);
                }

                values[t][i][x] = current;
            }
        }
    }

    // Part 1
    println!("Part 1: {}", values[29][aa][mm - 1]);

    // Part 2
    let mut best = 0;
    for x in 0..mm / 2 {
        let y = mm - 1 - x;
        best = best.max(values[25][aa][x] + values[25][aa][y]);
    }

    println!("Part 2: {}", best);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valve() {
        let input: Valve = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".into();
        assert_eq!(
            input,
            Valve {
                name: "AA".to_string(),
                flow_rate: 0,
                links: vec!["DD".to_string(), "II".to_string(), "BB".to_string()],
            }
        )
    }
}
