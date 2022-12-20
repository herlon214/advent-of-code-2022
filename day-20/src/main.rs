use std::collections::VecDeque;

fn skip_take<'a, T>(n: usize, zero: T) -> &'a (usize, i64)
where
    T: Iterator<Item = &'a (usize, i64)>,
{
    let result = zero.skip(n).take(1).last().unwrap();

    result
}

fn sort_list(numbers: &mut VecDeque<(usize, i64)>, count: usize) {
    for _ in 0..count {
        for i in 0..numbers.len() {
            // Find the index
            let idx = numbers
                .iter()
                .enumerate()
                .find_map(|(pos, (j, _))| (i == *j).then_some(pos))
                .unwrap();

            // Bring the number to the front
            numbers.rotate_left(idx);

            // Remove the number
            let (j, v) = numbers.pop_front().unwrap();

            // Calculate the new position
            let d = v.rem_euclid(numbers.len() as i64) as usize;

            // Move to that position
            numbers.rotate_left(d);

            // Add the number
            numbers.push_front((j, v));
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let numbers: Vec<i64> = input.lines().map(|it| it.parse().unwrap()).collect();

    // Part 1
    let mut list: VecDeque<(usize, i64)> = VecDeque::from(numbers.clone())
        .into_iter()
        .enumerate()
        .collect();

    sort_list(&mut list, 1);
    let zero = list.iter().cycle().skip_while(|it| it.1 != 0);

    let item_1000 = skip_take(1000, zero.clone());
    let item_2000 = skip_take(2000, zero.clone());
    let item_3000 = skip_take(3000, zero.clone());
    let sum: i64 = vec![item_1000, item_2000, item_3000]
        .iter()
        .map(|it| it.1)
        .sum();

    println!("Part 1: {sum}");

    // Part 2
    let numbers: Vec<i64> = numbers.iter().map(|it| *it as i64 * 811589153).collect();
    let mut list: VecDeque<(usize, i64)> = VecDeque::from(numbers.clone())
        .into_iter()
        .enumerate()
        .collect();
    sort_list(&mut list, 10);
    let zero = list.iter().cycle().skip_while(|it| it.1 != 0);

    let item_1000 = skip_take(1000, zero.clone());
    let item_2000 = skip_take(2000, zero.clone());
    let item_3000 = skip_take(3000, zero.clone());
    let sum: i64 = vec![item_1000, item_2000, item_3000]
        .iter()
        .map(|it| it.1)
        .sum();

    println!("Part 2: {sum}");
}
