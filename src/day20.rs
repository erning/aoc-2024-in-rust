use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

type Pos = (i32, i32);
type Grid = Vec<Vec<char>>;

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse_input(input: &str) -> Grid {
    input.trim().lines().map(|s| s.chars().collect()).collect()
}

fn find_char(grid: &Grid, ch: char) -> Option<Pos> {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &ch {
                return Some((x as i32, y as i32));
            }
        }
    }
    None
}

fn steps_to_position(grid: &Grid, position: Pos) -> HashMap<Pos, usize> {
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut visited: HashMap<Pos, usize> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(usize, Pos)>> = BinaryHeap::new();
    queue.push(Reverse((0, position)));

    while let Some(Reverse((step, p))) = queue.pop() {
        match grid[p.1 as usize][p.0 as usize] {
            'S' => {}
            '#' => continue,
            _ => {}
        }
        if visited.contains_key(&p) {
            continue;
        }
        visited.insert(p, step);
        for (dx, dy) in DIRS {
            let (x, y) = (p.0 + dx, p.1 + dy);
            if x < 0 || x >= w || y < 0 || y >= h {
                continue;
            }
            queue.push(Reverse((step + 1, (x, y))));
        }
    }

    visited
}

fn count_cheat<F>(grid: &Grid, f: F) -> usize
where
    F: Fn(usize, usize) -> bool,
{
    let start = find_char(grid, 'S').unwrap();
    let end = find_char(grid, 'E').unwrap();
    let steps_to_start = steps_to_position(grid, start);
    let steps_to_end = steps_to_position(grid, end);
    let min_steps = steps_to_end.get(&start).unwrap();

    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, ch)| **ch == '#')
                .map(|(x, _)| (x as i32, y as i32))
                .collect::<Vec<Pos>>()
        })
        .map(|(x, y)| {
            DIRS.iter()
                .map(|(dx, dy)| (x + dx, y + dy))
                .flat_map(|ps| {
                    DIRS.iter()
                        .map(|(dx, dy)| (x + dx, y + dy))
                        .map(|pe| (ps, pe))
                        .filter(|(ps, pe)| ps != pe)
                        .filter_map(|(ps, pe)| {
                            match (
                                steps_to_start.get(&ps),
                                steps_to_end.get(&pe),
                            ) {
                                (Some(a), Some(b)) => Some(a + b + 2),
                                _ => None,
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .filter(|v| f(*v, *min_steps))
                .count()
        })
        .sum()
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    count_cheat(&grid, |step, min_step| step <= min_step - 100)
}

pub fn part_two(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(20);
        let grid = parse_input(&input);
        assert_eq!(count_cheat(&grid, |step, m| step == m - 2), 14);
        assert_eq!(count_cheat(&grid, |step, m| step == m - 4), 14);
        assert_eq!(count_cheat(&grid, |step, m| step == m - 6), 2);
        assert_eq!(count_cheat(&grid, |step, m| step == m - 8), 4);
        assert_eq!(count_cheat(&grid, |step, m| step == m - 10), 2);
        assert_eq!(count_cheat(&grid, |step, m| step == m - 12), 3);
        assert_eq!(count_cheat(&grid, |step, m| step == m - 20), 1);
        assert_eq!(count_cheat(&grid, |step, m| step == m - 36), 1);
        assert_eq!(count_cheat(&grid, |step, m| step == m - 38), 1);
        assert_eq!(count_cheat(&grid, |step, m| step == m - 40), 1);
        assert_eq!(count_cheat(&grid, |step, m| step == m - 64), 1);
    }
}
