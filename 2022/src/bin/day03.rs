pub fn part_one(input: &str) {
    let mut total_priority = 0;
    for line in input.lines() {
        let sack_halves = split_rucksack(line);
        let shared_item = get_shared_char_two_bags(sack_halves).expect("Error: no shared charater");
        let priority = char_to_priority(shared_item).expect("Error: invalid item");
        total_priority += priority;
    }
    println!("{}", total_priority);
}

pub fn part_two(input: &str) {
    let mut total_priority: u32 = 0;
    let mut input2 = input.clone().lines();

    for _ in input.lines().enumerate().step_by(3) {
        let sack_one: &str = input2.next().expect("Error: sack one missing");
        let sack_two: &str = input2.next().expect("Error: sack two missing");
        let sack_three: &str = input2.next().expect("Error: sack three missing");
        let shared_item = get_shared_char_three_bags((sack_one, sack_two, sack_three)).unwrap();
        let priority = char_to_priority(shared_item).unwrap();
        total_priority += priority;
    }
    println!("{}", total_priority);
}

fn char_to_priority(ch: char) -> Option<u32> {
    assert!(ch.is_ascii());
    if ch.is_lowercase() {
        Some((ch as u32) - ('a' as u32) + 1)
    } else if ch.is_uppercase() {
        Some((ch as u32) - ('A' as u32) + 27)
    } else {
        None
    }
}

fn split_rucksack(sack: &str) -> (&str, &str) {
    let num_sack_items = sack.chars().count();
    let midpoint = num_sack_items / 2;
    (&sack[0..midpoint], &sack[midpoint..])
}

fn get_shared_char_three_bags(sack: (&str, &str, &str)) -> Option<char> {
    for sack0_item in sack.0.chars() {
        if sack.1.contains(sack0_item) && sack.2.contains(sack0_item) {
            return Some(sack0_item);
        }
    }
    None
}

fn get_shared_char_two_bags(sack: (&str, &str)) -> Option<char> {
    for sack0_item in sack.0.chars() {
        if sack.1.contains(sack0_item) {
            return Some(sack0_item);
        }
    }
    None
}

fn main() {
    let input = aoc_2022::read_file("inputs", 3);
    part_one(&input);
    part_two(&input);
}
