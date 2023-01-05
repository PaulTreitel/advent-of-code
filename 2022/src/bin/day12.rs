pub fn part_one(input: &str) -> Option<i32> {
    let (map, start) = get_elevation_map(input);
    // print_map(&map);
    let num_steps = bfs_map(map, &mut vec![(0, start)]);
    Some(num_steps)
}

fn valid_step(a: i32, b: i32) -> bool {
    b == a + 1 || b <= a
}

fn bfs_map(map: Vec<Vec<i32>>, queue: &mut Vec<(i32, (i32, i32))>) -> i32 {
    // let mut queue = vec![(0, start)];
    let mut visited: Vec<(i32, i32)> = Vec::new();
    let rows = map.len();
    let cols = map.get(0).unwrap().len();
    while  queue.len() > 0 {
        let (moves, pos) = queue.remove(0);
        if visited.contains(&pos) {
            continue;
        }
        visited.push(pos);
        let start_elevation = map.get(pos.0 as usize).unwrap().get(pos.1 as usize).unwrap();
        for (newrow, newcol) in [(pos.0 + 1, pos.1), (pos.0 - 1, pos.1), (pos.0, pos.1 + 1), (pos.0, pos.1 - 1)] {
            if (newrow < 0 || newrow >= rows as i32) || (newcol < 0 || newcol >= cols as i32) {
                continue;
            }
            let new_elevation = map.get(newrow as usize).unwrap().get(newcol as usize).unwrap();
            if valid_step(*start_elevation, *new_elevation) {
                if *new_elevation == 26 {
                    return moves + 1;
                }
                if !visited.contains(&(newrow, newcol)) {
                    queue.push((moves + 1, (newrow, newcol)));
                }
            }
        }
    }
    return -1;
}

pub fn part_two(input: &str) -> Option<i32> {
    let (map, _) = get_elevation_map(input);
    let mut starts: Vec<(i32, (i32, i32))> = Vec::new();
    for row_index in 0..map.len() {
        for col_index in 0..map.get(0).unwrap().len() {
            let elevation = map.get(row_index).unwrap().get(col_index).unwrap();
            if *elevation == 0 {
                starts.push((0, (row_index as i32, col_index as i32)));
            }
        }
    }
    let steps = bfs_map(map, &mut starts);
    Some(steps)
}

fn get_elevation_map(input: &str) -> (Vec<Vec<i32>>, (i32, i32)) {
    let mut map = Vec::<Vec<i32>>::new();
    let mut rowcount = 0;
    let mut start: (i32, i32) = (0, 0);
    for line in input.lines() {
        let mut colcount = 0;
        let mut map_row: Vec<i32> = Vec::new();
        let line = line.chars();
        for ch in line {
            map_row.push(get_height(ch));
            if ch == 'S' {
                start = (rowcount, colcount);
                // println!("start found at {:?}", start);
            } else if ch == 'E' {
                // println!("end found at ({}, {})", rowcount, colcount);
            }
            colcount += 1;
        }
        map.push(map_row);
        rowcount += 1;
    }
    (map, start)
}

fn get_height(ch: char) -> i32 {
    if ch == 'S' {
        return 0;
    } else if ch == 'E' {
        return 26;
    }
    ch as i32 - 'a' as i32
}

fn main() {
    let input = aoc_2022::read_file("inputs", 12);
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
        let input = aoc_2022::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31)); // fill in
    }
    #[test]
    fn test_part_two() {
        let input = aoc_2022::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29)); // fill in
    }
}
