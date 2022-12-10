use priority_queue::PriorityQueue;

fn main() {
    let mut pq: PriorityQueue<usize, i64> = PriorityQueue::new();
    let input = include_str!("../input");

    let mut elf_idx = 1;
    let mut sum = 0;

    for line in input.lines() {
        if line == "" {
            // Add to queue
            pq.push(elf_idx, sum);

            sum = 0;
            elf_idx += 1;
        } else {
            let value = line.parse::<i64>().unwrap();
            sum += value;
        }
    }

    // Get top K elements
    let mut top_k = 3;
    let mut top_sum = 0;
    for top_elf in pq.into_sorted_iter() {
        if top_k <= 0 {
            break;
        }

        println!("Elf {} is carrying {} calories", top_elf.0, top_elf.1);
        top_sum += top_elf.1;
        top_k -= 1;
    }

    println!("----------------");
    println!("Total: {}", top_sum);
}
