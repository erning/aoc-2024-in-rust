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
                .into_iter()
                .map(|v| v.parse::<i8>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .collect()
}

fn shortest_path(positions: &[Pos], start: Pos, end: Pos) -> Option<usize> {
    let corrupted: HashSet<Pos> = positions.into_iter().cloned().collect();
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
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
            let (x, y) = (node.0 + dx, node.1 + dy);
            if x < 0 || x > end.0 || y < 0 || y > end.1 {
                continue;
            }
            let next = (x, y);
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
    shortest_path(&positions[..1024], (0, 0), (70, 70)).unwrap()
}

pub fn part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(18);
        let positions = parse_input(&input);
        assert_eq!(shortest_path(&positions[..12], (0, 0), (6, 6)), Some(22));
        //assert_eq!(part_two(&input), "6,1");
    }
}
