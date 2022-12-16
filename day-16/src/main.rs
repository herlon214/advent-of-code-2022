#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valve {
    name: String,
    flow_rate: i32,
    links: Vec<String>,
    link_idx: Option<usize>,
}

impl Valve {
    fn next_link(&mut self) -> String {
        let current = self.link_idx.unwrap_or(0);
        let target = self.links.get(current).unwrap();
        self.link_idx = Some((current + 1) % self.links.len());

        target.clone()
    }
}

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
            link_idx: None,
        }
    }
}

fn main() {
    let input = include_str!("../example");
    let valves: HashMap<String, Valve> = input
        .lines()
        .map(|it| {
            let valve = Valve::from(it);

            (it.to_string(), valve)
        })
        .collect();

    dbg!(&valves);
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
                link_idx: None,
            }
        )
    }
}
