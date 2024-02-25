use petgraph::{
    algo::floyd_warshall,
    graph::{NodeIndex, UnGraph},
    Graph,
};
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

#[derive(Debug)]
struct Part1State {
    pos: NodeIndex,
    opened: HashSet<NodeIndex>,
    mins_left: i32,
    released: i32,
}


pub fn part_one(input: &str) -> Option<i32> {
    let (indices, graph) = get_graph(input);
    let dists = floyd_warshall(&graph, |_| 1).ok().unwrap();

    let mut to_visit = vec![Part1State {
        pos: indices.get("AA").unwrap().0,
        opened: HashSet::new(),
        mins_left: 30,
        released: 0,
    }];
    to_visit
        .get_mut(0)
        .unwrap()
        .opened
        .insert(indices.get("AA").unwrap().0);
    let mut max_pressure = 0;

    while !to_visit.is_empty() {
        let curr = to_visit.pop().unwrap();
        if curr.opened.len() == graph.node_count() {
            max_pressure = max(curr.released, max_pressure);
            continue;
        }
        let mut added = false;
        for n in graph.node_indices() {
            let pressure = graph.node_weight(n).unwrap();
            let cost = 1 + dists.get(&(curr.pos, n)).unwrap();
            if n == curr.pos || curr.opened.contains(&n) || *pressure == 0 || cost > curr.mins_left
            {
                continue;
            }

            let mut new_open = Part1State {
                pos: n,
                opened: curr.opened.clone(),
                mins_left: curr.mins_left - cost,
                released: curr.released + pressure * (curr.mins_left - cost),
            };
            new_open.opened.insert(n);
            to_visit.push(new_open);
            added = true;
        }
        if !added {
            max_pressure = max(curr.released, max_pressure);
        }
    }
    Some(max_pressure)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (indices, graph) = get_graph(input);
    let dists = floyd_warshall(&graph, |_| 1).ok().unwrap();

    let valve_sets = get_valve_sets(indices.get("AA").unwrap().0, graph, dists);
    println!("{:?}", valve_sets);
    println!("{}", valve_sets.len());
    None
}

fn get_valve_sets(
    start: NodeIndex,
    graph: Graph<i32, (), petgraph::Undirected>, 
    dists: HashMap<(NodeIndex, NodeIndex), i32>
) -> Vec<(HashSet<NodeIndex>, i32)> {
    let mut valve_sets: Vec<(HashSet<NodeIndex>, i32)> = Vec::new();
    let mut first = (HashSet::new(), start, 0, 26);
    first.0.insert(start);
    let mut to_visit = vec![first];

    while !to_visit.is_empty() {
        let curr = to_visit.pop().unwrap();
        add_valve_set(&mut valve_sets, curr.0.clone(), curr.2);
        valve_sets.push((curr.0.clone(), curr.2));
        if curr.3 == 0 {
            continue;
        }
        for n in graph.node_indices() {
            let pressure = *graph.node_weight(n).unwrap();
            let cost = 1 + dists.get(&(curr.1, n)).unwrap();
            let new_time = curr.3 - cost;
            let new_released = curr.2 + pressure * new_time;
            if n == curr.1 || pressure == 0 || curr.0.contains(&n) || new_time < 0 {
                continue;
            }
            let mut new_state = (curr.0.clone(), n, new_released, new_time);
            new_state.0.insert(n);
            add_valve_set(&mut valve_sets, new_state.0.clone(), new_released);
            to_visit.push(new_state);
        }
    }
    valve_sets
}

fn add_valve_set (
    valve_sets: &mut Vec<(HashSet<NodeIndex>, i32)>, 
    curr_set: HashSet<NodeIndex>, 
    released: i32
) -> () {
    for idx in 0..valve_sets.len() {
        let existing_set = valve_sets.get_mut(idx).unwrap();
        if existing_set.0 == curr_set {
            if (released > existing_set.1) {
                existing_set.1 = released;
                return;
            }
        }
    }
    valve_sets.push((curr_set, released));
}

fn get_graph(
    input: &str,
) -> (
    HashMap<String, (NodeIndex, Vec<String>)>,
    Graph<i32, (), petgraph::Undirected>,
) {
    let mut indices: HashMap<String, (NodeIndex, Vec<String>)> = HashMap::new();
    let mut graph = UnGraph::<i32, ()>::new_undirected();
    for line in input.lines() {
        let mut line = line.split_ascii_whitespace();
        line.next();
        let name = line.next().unwrap().to_string();
        line.next();
        line.next();
        let rate: i32 = line
            .next()
            .unwrap()
            .matches(char::is_numeric)
            .fold("".to_string(), |acc, ch| acc + ch)
            .parse()
            .unwrap();
        line.next();
        line.next();
        line.next();
        line.next();
        let mut neighbors = Vec::new();
        while let Some(valve_name) = line.next() {
            neighbors.push(valve_name.replace(",", ""));
        }
        indices.insert(name, (graph.add_node(rate), neighbors));
    }
    for v_name in indices.keys() {
        let vertex = indices.get(v_name).unwrap();
        for neighbor in vertex.1.clone() {
            graph.add_edge(vertex.0, indices.get(&neighbor).unwrap().0, ());
        }
    }
    (indices, graph)
}

fn main() {
    let input = aoc_2022::read_file("inputs", 16);
    // let res = part_one(&input).unwrap();
    // println!("{}", res);
    let res = part_two(&input).unwrap();
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc_2022::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651)); // fill in
    }
    #[test]
    fn test_part_two() {
        let input = aoc_2022::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707)); // fill in
    }
}
