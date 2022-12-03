use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn calc_priority(input: char) -> usize {
    let mut tmp = input.clone();
    let uppercase = match tmp {
        'A'..='Z' => true,
        _ => false,
    };

    if uppercase {
        tmp = tmp.to_ascii_lowercase();
    }

    let priority = match tmp {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        _ => 0,
    };

    if uppercase {
        return 26 + priority;
    }

    priority
}

fn parse_line<'a>(input: &'a str) -> (&'a str, &'a str) {
    input.split_at(input.len() / 2)
}

fn intersect<'a>(first: &'a str, second: &'a str) -> char {
    let parts = first.split_terminator("").skip(1);
    for character in parts {
        if second.contains(character) {
            return character.chars().last().unwrap();
        }
    }

    panic!("repeated character not found")
}

fn identify_group(group: Vec<String>) -> char {
    let mut map: HashMap<&char, bool> = HashMap::new();
    let group_chars: Vec<Vec<char>> = group.iter().map(|it| it.chars().collect()).collect();

    for (i, chars) in group_chars.iter().enumerate() {
        // If it's the first group just fill the hashmap
        if i == 0 {
            for character in chars {
                map.insert(character, true);
            }
        } else {
            // Remove the items that aren't present
            for map_char in map.clone().keys() {
                if !chars.contains(&map_char) {
                    map.remove(map_char);
                }
            }
        }
    }

    map.into_keys().last().unwrap().clone()
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut repeated_total = 0;
    let mut group_total = 0;
    let mut group: Vec<String> = vec![];

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let (first, second) = parse_line(&content);
                let repeated = intersect(first, second);
                // Part 1
                repeated_total += calc_priority(repeated);

                // Part 2
                group.push(content.clone());
                if group.len() == 3 {
                    let group_badge = identify_group(group);
                    group_total += calc_priority(group_badge);

                    // Reset the group
                    group = vec![];
                }
            }
            Err(e) => panic!("Failed to read line: {}", e.to_string()),
        }
    }

    println!("Total sum of repeated items priority: {}", repeated_total);
    println!("Sum of group badges: {}", group_total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority() {
        assert_eq!(calc_priority('a'), 1);
        assert_eq!(calc_priority('z'), 26);
        assert_eq!(calc_priority('A'), 27);
        assert_eq!(calc_priority('Z'), 52);
    }

    #[test]
    fn split() {
        let line: String = "vJrwpWtwJgWrhcsFMMfFFhFp".into();
        let (first, second) = parse_line(&line);
        assert_eq!(first, "vJrwpWtwJgWr");
        assert_eq!(second, "hcsFMMfFFhFp");
    }

    #[test]
    fn repeated_character() {
        let line: String = "vJrwpWtwJgWrhcsFMMfFFhFp".into();
        let (first, second) = parse_line(&line);
        println!("{:?} and {:?}", first, second);

        assert_eq!(intersect(first, second), 'p');
    }

    #[test]
    fn extract_group_1() {
        let group: Vec<String> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
        ];

        assert_eq!(identify_group(group), 'r');
    }

    #[test]
    fn extract_group_2() {
        let group: Vec<String> = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];

        assert_eq!(identify_group(group), 'Z');
    }
}
