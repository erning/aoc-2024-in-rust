use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

type Pos = (i8, i8);
const DIRS: [(i8, i8); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse_input(input: &str) -> Vec<(i8, i8)> {
    input
        .trim()
        .lines()
        .map(|s| {
            s.splitn(2, ',')
                .map(|v| v.parse::<i8>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .collect()
}

fn shortest_steps(positions: &[Pos], start: Pos, end: Pos) -> Option<usize> {
    let corrupted: HashSet<Pos> = positions.iter().cloned().collect();
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start)));
    while let Some(Reverse((steps, node))) = queue.pop() {
        if node == end {
            return Some(steps);
        }
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        for (dx, dy) in DIRS {
            let next = (node.0 + dx, node.1 + dy);
            if next.0 < 0 || next.0 > end.0 || next.1 < 0 || next.1 > end.1 {
                continue;
            }
            if corrupted.contains(&next) {
                continue;
            }
            queue.push(Reverse((steps + 1, next)));
        }
    }
    None
}

pub fn part_one(input: &str) -> usize {
    let positions = parse_input(input);
    shortest_steps(&positions[..1024], (0, 0), (70, 70)).unwrap()
}

fn privent_coordinate(positions: &[Pos], start: Pos, end: Pos) -> String {
    match (1..=positions.len())
        .collect::<Vec<usize>>()
        .binary_search_by(|&i| {
            match shortest_steps(&positions[..i], start, end) {
                None => Ordering::Greater,
                _ => Ordering::Less,
            }
        }) {
        Err(i) => format!("{},{}", positions[i].0, positions[i].1),
        _ => String::new(),
    }
}

pub fn part_two(input: &str) -> String {
    let positions = parse_input(input);
    privent_coordinate(&positions, (0, 0), (70, 70))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(18);
        let positions = parse_input(&input);
        let steps = shortest_steps(&positions[..12], (0, 0), (6, 6)).unwrap();
        assert_eq!(steps, 22);
        let p = privent_coordinate(&positions, (0, 0), (6, 6));
        assert_eq!(p, "6,1");
    }
}
