use core::num;

pub fn part_one(input: &str) -> Option<i32> {
    let mut num_sequence = get_numbers(input);
    let count = num_sequence.len() as i32;
    let mut decryption: Vec<i32> = num_sequence.iter().map(|p|p.0).collect();
    for i in 0..count {
        let old = num_sequence.get(i as usize).unwrap().clone();
        let new_idx = get_new_idx(old.1, old.0, count);
        // println!("{:?} :: {:?}", decryption, num_sequence);
        // println!("{:?} -> {}", old, new_idx);
        
        decryption.insert(new_idx as usize, old.0);
        if new_idx <= old.1 {
            let tmp = decryption.remove(old.1 as usize + 1);
            // println!("1: added {} at idx {}, removed {} at idx {}", old.0, new_idx, tmp, old.1 + 1);
            if old.0 != tmp {
                panic!("n={} but removed value is {:?}", old.0, tmp);
            }
        } else {
            let tmp = decryption.remove(old.1 as usize);
            // println!("2: added {} at idx {}, removed {} at idx {}", old.0, new_idx, tmp, old.1);
            if old.0 != tmp {
                panic!("n={} but removed value is {:?}", old.0, tmp);
            }
        }
        for j in 0..count {
            let tmp = num_sequence.get_mut(j as usize).unwrap();
            if tmp.0 == old.0 && tmp.1 == old.1 {
                tmp.1 = new_idx - 1;
            } else if new_idx < old.1 && tmp.1 >= new_idx && tmp.1 < old.1  {
                tmp.1 += 1;
            } else if new_idx > old.1 && tmp.1 > old.1 && tmp.1 < new_idx {
                tmp.1 -= 1;
            }
        }
    }
    let zero_idx = decryption.iter().position(|x| *x == 0).unwrap();
    let p1 = *decryption.get((zero_idx + 1000) % count as usize).unwrap();
    let p2 = *decryption.get((zero_idx + 2000) % count as usize).unwrap();
    let p3 = *decryption.get((zero_idx + 3000) % count as usize).unwrap();
    // println!("{:?} :: {:?}", decryption, num_sequence);
    // println!("{}, {}, {}", p1, p2, p3);
    Some(p1 + p2 + p3)
    // None
}

pub fn part_two(input: &str) -> Option<i32> {
    None
}

fn get_new_idx(old_idx: i32, n: i32, count: i32) -> i32 {
    let mut new = old_idx + n;
    if new > 0 {
        new += 1;
    }
    new.rem_euclid(count)
}

fn get_numbers(input: &str) -> Vec<(i32, i32)> {
    let mut nums = Vec::new();
    let mut counter = 0;
    for line in input.lines() {
        nums.push((line.parse::<i32>().unwrap(), counter));
        counter += 1;
    }
    nums
}

fn main() {
    let input = aoc_2022::read_file("inputs", 20);
    let res = part_one(&input).unwrap();
    println!("{}", res);
    let res = part_two(&input).unwrap();
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc_2022::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3)); // fill in
    }
    #[test]
    fn test_part_two() {
        let input = aoc_2022::read_file("examples", 20);
        assert_eq!(part_two(&input), None); // fill in
    }
}
