
struct Move {
    direction: (i32, i32),
    num_steps: i32,
}

pub fn part_one(input: &str) -> Option<i32> {
    let moves = get_move_list(input);
    let mut current_position = vec![(0,0), (0,0)];
    run_moves(&mut current_position, moves)
}

fn run_moves(current_position: &mut Vec<(i32, i32)>, moves: Vec<Move>) -> Option<i32> {
    let mut locations: Vec<(i32, i32)> = vec![(0, 0)];
    for m in moves {
        let mut new_locations = get_locations_from_move(current_position, m);
        locations.append(&mut new_locations);
    }
    locations.sort();
    locations.dedup();
    Some(locations.len() as i32)
}

fn get_locations_from_move(pos: &mut Vec<(i32, i32)>, m: Move) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::new();
    for _ in 0..m.num_steps {
        pos.get_mut(0).unwrap().0 += m.direction.0;
        pos.get_mut(0).unwrap().1 += m.direction.1;
        let new_result: Option<(i32, i32)> = update_rope(pos);
        match new_result {
            Some(p) => result.push(p),
            None => (),
        }
    }
    result
}

fn update_rope(pos: &mut Vec<(i32, i32)>) -> Option<(i32, i32)> {
    let tmp = pos.len();
    let mut head_knot = &mut pos.get(0).unwrap().clone();
    let mut count = 0;
    for x in pos {//.iter().enumerate() {
        if count == 0 {
            count += 1;
            continue;
        }
        if count == tmp - 1 {
            return update_knot(&head_knot, x);
        }
        update_knot(&head_knot, x);
        head_knot = x;
        count += 1;
    }
    None
}

fn update_knot(start: &(i32, i32), end: &mut (i32, i32)) -> Option<(i32, i32)> {
    let diffs = get_position_diff(*start, *end);
    if diffs.0.abs() <= 1 && diffs.1.abs() <= 1 {
        return None;
    }
    if diffs.0 == 0 {
        end.1 = end.1 + diffs.1 / 2;
    } else if diffs.1 == 0 {
        end.0 = end.0 + diffs.0 / 2;
    } else {
        let pos_change = (diffs.0 / diffs.0.abs(), diffs.1 / diffs.1.abs());
        *end = (end.0 + pos_change.0, end.1 + pos_change.1);
    }
    Some(*end)
}

fn get_position_diff(start: (i32, i32), end: (i32, i32)) -> (i32, i32) {
    ((start.0 - end.0), (start.1 - end.1))
}

pub fn part_two(input: &str) -> Option<i32> {
    let moves = get_move_list(input);
    let mut current_position = vec![(0,0); 10];
    run_moves(&mut current_position, moves)
}

fn get_move_list(input: &str) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for line in input.lines() {
        let mut line = line.split_ascii_whitespace();
        let dir = direction_from_letter(line.next().unwrap()).unwrap();
        let steps: i32 = line.next().unwrap().parse().unwrap();
        let new_move = Move { direction: dir, num_steps: steps };
        moves.push(new_move);
    }
    moves
}

fn direction_from_letter(letter: &str) -> Option<(i32, i32)> {
    let letter = letter.chars().next().unwrap();
    if letter == 'R' {
        return Some((0, 1));
    } else if letter == 'U' {
        return Some((1, 0))
    } else if letter == 'D' {
        return Some((-1, 0));
    } else if letter == 'L' {
        return Some((0, -1))
    }
    None
}

fn main() {
    let input = aoc_2022::read_file("inputs", 9);
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
        let input = aoc_2022::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13)); // fill in
    }
    #[test]
    fn test_part_two() {
        let input = aoc_2022::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1)); // fill in
    }
}
