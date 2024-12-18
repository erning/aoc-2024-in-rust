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

fn shortest_path(positions: &[Pos], start: Pos, end: Pos) -> Vec<Pos> {
    let corrupted: HashSet<Pos> = positions.iter().cloned().collect();
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push(Reverse((0, vec![start])));
    while let Some(Reverse((steps, path))) = queue.pop() {
        let &node = path.last().unwrap();
        if node == end {
            return path;
        }
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        for (dx, dy) in DIRS {
            let (x, y) = (node.0 + dx, node.1 + dy);
            if x < 0 || x > end.0 || y < 0 || y > end.1 {
                continue;
            }
            if corrupted.contains(&(x, y)) {
                continue;
            }
            let mut next = path.clone();
            next.push((x, y));
            queue.push(Reverse((steps + 1, next)));
        }
    }
    Vec::new()
}

pub fn part_one(input: &str) -> usize {
    let positions = parse_input(input);
    shortest_path(&positions[..1024], (0, 0), (70, 70)).len() - 1
}

fn privent_coordinate(positions: &[Pos], start: Pos, end: Pos) -> String {
    match (1..=positions.len())
        .collect::<Vec<usize>>()
        .binary_search_by(|&i| {
            let path = shortest_path(&positions[..i], start, end);
            if path.is_empty() {
                Ordering::Greater
            } else {
                Ordering::Less
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
        let steps = shortest_path(&positions[..12], (0, 0), (6, 6)).len() - 1;
        assert_eq!(steps, 22);
        let p = privent_coordinate(&positions, (0, 0), (6, 6));
        assert_eq!(p, "6,1");
    }
}
