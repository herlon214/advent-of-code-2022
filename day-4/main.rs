use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone)]
struct Assignment {
    start: usize,
    end: usize,
}

impl Assignment {
    pub fn new(input: &str) -> Self {
        let parts: Vec<usize> = input
            .split('-')
            .map(|it| it.parse::<usize>().unwrap())
            .collect();

        Self {
            start: parts.get(0).unwrap().clone(),
            end: parts.get(1).unwrap().clone(),
        }
    }
}

struct Pair(Assignment, Assignment);

impl Pair {
    pub fn new(input: &str) -> Self {
        let assignments: Vec<Assignment> = input.split(',').map(|it| Assignment::new(it)).collect();

        Self(
            assignments.get(0).unwrap().clone(),
            assignments.get(1).unwrap().clone(),
        )
    }

    pub fn overlap_any(&self) -> bool {
        // (A overlaps any of B) OR (B overlaps any of A)
        if (self.0.end >= self.1.start && self.0.end <= self.1.end)
            || (self.1.end >= self.0.start && self.1.end <= self.0.end)
        {
            return true;
        }

        false
    }

    pub fn overlap_full(&self) -> bool {
        // (A fully contains B) OR (B fully contains A)
        if (self.0.start <= self.1.start && self.0.end >= self.1.end)
            || (self.1.start <= self.0.start && self.1.end >= self.0.end)
        {
            return true;
        }

        false
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut total_full_overlaps = 0;
    let mut total_any_overlaps = 0;

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let pair = Pair::new(&content);

                // Part 1
                if pair.overlap_full() {
                    total_full_overlaps += 1;
                }

                // Part 2
                if pair.overlap_any() {
                    total_any_overlaps += 1;
                }
            }
            Err(e) => panic!("Failed to read line: {}", e.to_string()),
        }
    }

    println!("Total pairs that fully overlaps: {}", total_full_overlaps);
    println!("Total pairs that overlaps any: {}", total_any_overlaps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_assigment() {
        let input = "2-4";
        let result = Assignment::new(&input);

        assert_eq!(result.start, 2);
        assert_eq!(result.end, 4);
    }

    #[test]
    fn parse_pair() {
        let input = "2-4,6-8";
        let result = Pair::new(&input);

        assert_eq!(result.0.start, 2);
        assert_eq!(result.0.end, 4);
        assert_eq!(result.1.start, 6);
        assert_eq!(result.1.end, 8);
    }
}
