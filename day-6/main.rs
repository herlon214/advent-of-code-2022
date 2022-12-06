use std::collections::HashSet;

enum MarkerType {
    StartOfPacket = 4,
    Message = 14,
}

fn identify_marker(input: &str, marker_type: MarkerType) -> usize {
    let mut start = 0;
    let mut current = start;
    let mut set: HashSet<&char> = HashSet::new();
    let characters: Vec<char> = input.chars().collect();
    let distinct_characters = marker_type as usize;

    while current < characters.len() {
        let character = characters.get(current).unwrap();

        // Check if character already exist
        if set.get(&character).is_some() {
            start += 1;
            current = start;

            // Reset set
            set = HashSet::new();

            continue;
        } else {
            set.insert(character);
            // Check current length
            if set.len() == distinct_characters {
                break;
            }
        }

        current += 1;
    }

    current + 1
}

fn main() {
    let input = include_str!("input");

    // Part 1
    println!(
        "First start-of-packet marker: {}",
        identify_marker(input, MarkerType::StartOfPacket)
    );

    // Part 2
    println!(
        "First start-of-message marker: {}",
        identify_marker(input, MarkerType::Message)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_of_packet_markers() {
        assert_eq!(
            identify_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", MarkerType::StartOfPacket),
            5
        );
        assert_eq!(
            identify_marker("nppdvjthqldpwncqszvftbrmjlhg", MarkerType::StartOfPacket),
            6
        );
        assert_eq!(
            identify_marker(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                MarkerType::StartOfPacket
            ),
            10
        );
        assert_eq!(
            identify_marker(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                MarkerType::StartOfPacket
            ),
            11
        );
    }

    #[test]
    fn message_markers() {
        assert_eq!(
            identify_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", MarkerType::Message),
            19
        );
        assert_eq!(
            identify_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", MarkerType::Message),
            23
        );
        assert_eq!(
            identify_marker("nppdvjthqldpwncqszvftbrmjlhg", MarkerType::Message),
            23
        );
        assert_eq!(
            identify_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", MarkerType::Message),
            29
        );
        assert_eq!(
            identify_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", MarkerType::Message),
            26
        );
    }
}
