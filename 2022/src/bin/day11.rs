
struct Monkey {
    items: Vec<u64>,
    update_worry: fn(u64) -> u64,
    test_num: u64,
    throw_true: usize,
    throw_false: usize,
}

pub fn part_one(_input: &str, real: bool) -> Option<u64> {
    let mut monkeys = get_monkeys(real);
    let mut num_inspections: Vec<u64> = vec![0; monkeys.len()];
    let update_relief = |x: u64| x / 3;
    for _ in 0..20 {
        for monk_idx in 0..monkeys.len() {
            let monk = monkeys.get_mut(monk_idx).unwrap();
            let throws = monkey_turn(monk, update_relief);
            *num_inspections.get_mut(monk_idx).unwrap() += throws.len() as u64;
            for (item, to_monk) in throws {
                monkeys.get_mut(to_monk).unwrap().items.push(item);
            }
        }
    }
    num_inspections.sort();
    num_inspections.reverse();
    let result = num_inspections.get(0).unwrap() * num_inspections.get(1).unwrap();
    Some(result)
}

fn monkey_turn(m: &mut Monkey, update_relief: fn(u64) -> u64) -> Vec<(u64, usize)> {
    let mut result: Vec<(u64, usize)> = Vec::new();
    while m.items.len() > 0 {
        let mut item = m.items.pop().unwrap();
        item = (m.update_worry)(item);
        item = (update_relief)(item);
        
        if item % m.test_num == 0 {
            result.push((item, m.throw_true));
        } else {
            result.push((item, m.throw_false));
        }
    }
    result
}

pub fn part_two(_input: &str, real: bool) -> Option<u64> {
    let mut monkeys = get_monkeys(real);
    let mut num_inspections: Vec<u64> = vec![0; monkeys.len()];
    let update_relief = |x: u64| x;
    let max_wrap: u64 = monkeys.iter().map(|m| m.test_num).product();
    for _ in 0..10_000 {
        for monk_idx in 0..monkeys.len() {
            let throws = monkey_turn(monkeys.get_mut(monk_idx).unwrap(), update_relief);
            *num_inspections.get_mut(monk_idx).unwrap() += throws.len() as u64;
            for (item, to_monk) in throws {
                if item > max_wrap {
                    monkeys.get_mut(to_monk).unwrap().items.push(item % max_wrap);
                } else {
                    monkeys.get_mut(to_monk).unwrap().items.push(item);
                }
            }
        }
    }
    num_inspections.sort();
    num_inspections.reverse();
    let result = num_inspections.get(0).unwrap() * num_inspections.get(1).unwrap();
    Some(result)
}

fn get_monkeys(real_input:bool) -> Vec<Monkey> {
    let mut result: Vec<Monkey> = Vec::new();
    if real_input {
        result.push(Monkey{
            items: vec![66, 79],
            update_worry: |x| x * 11,
            test_num: 7,
            throw_true: 6,
            throw_false: 7,
        });
        result.push(Monkey{
            items: vec![84, 94, 94, 81, 98, 75],
            update_worry: |x| x * 17,
            test_num: 13,
            throw_true: 5,
            throw_false: 2,
        });
        result.push(Monkey{
            items: vec![85, 79, 59, 64, 79, 95, 67],
            update_worry: |x| x + 8,
            test_num: 5,
            throw_true: 4,
            throw_false: 5,
        });
        result.push(Monkey{
            items: vec![70],
            update_worry: |x| x + 3,
            test_num: 19,
            throw_true: 6,
            throw_false: 0,
        });
        result.push(Monkey{
            items: vec![57, 69, 78, 78],
            update_worry: |x| x + 4,
            test_num: 2,
            throw_true: 0,
            throw_false: 3,
        });
        result.push(Monkey{
            items: vec![65, 92, 60, 74, 72],
            update_worry: |x| x + 7,
            test_num: 11,
            throw_true: 3,
            throw_false: 4,
        });
        result.push(Monkey{
            items: vec![77, 91, 91],
            update_worry: |x| x * x,
            test_num: 17,
            throw_true: 1,
            throw_false: 7,
        });
        result.push(Monkey{
            items: vec![76, 58, 57, 55, 67, 77, 54, 99],
            update_worry: |x| x + 6,
            test_num: 3,
            throw_true: 2,
            throw_false: 1,
        });
    } else {
        result.push(Monkey{
            items: vec![79, 98],
            update_worry: |x| x * 19,
            test_num: 23,
            throw_true: 2,
            throw_false: 3,
        });
        result.push(Monkey{
            items: vec![54, 65, 75, 74],
            update_worry: |x| x + 6,
            test_num: 19,
            throw_true: 2,
            throw_false: 0,
        });
        result.push(Monkey{
            items: vec![79, 60, 97],
            update_worry: |x| x * x,
            test_num: 13,
            throw_true: 1,
            throw_false: 3,
        });
        result.push(Monkey{
            items: vec![74],
            update_worry: |x| x + 3,
            test_num: 17,
            throw_true: 0,
            throw_false: 1,
        });
    }
    result
}

fn main() {
    let input = aoc_2022::read_file("inputs", 11);
    let res = part_one(&input, true).unwrap();
    println!("{}", res);
    let res = part_two(&input, true).unwrap();
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc_2022::read_file("examples", 11);
        assert_eq!(part_one(&input, false), Some(10605)); // fill in
    }
    #[test]
    fn test_part_two() {
        let input = aoc_2022::read_file("examples", 11);
        assert_eq!(part_two(&input, false), Some(2713310158)); // fill in
    }
}
