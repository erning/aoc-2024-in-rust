use std::collections::HashSet;

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (HashSet<(i32, i32)>, i32, i32, (i32, i32)) {
    let mut start = (0, 0);
    let mut height = 0;
    let mut width = 0;
    let obstructions = input
        .trim()
        .lines()
        .enumerate()
        .filter(|(y, _)| {
            height = height.max(*y as i32);
            true
        })
        .flat_map(|(y, s)| {
            s.bytes()
                .enumerate()
                .filter(|(x, _)| {
                    width = width.max(*x as i32);
                    true
                })
                .filter(|&(x, c)| match c {
                    b'#' => true,
                    b'^' => {
                        start = (x as i32, y as i32);
                        false
                    }
                    _ => false,
                })
                .map(|(x, _)| (x as i32, y as i32))
                .collect::<Vec<(i32, i32)>>()
        })
        .collect();
    (obstructions, width + 1, height + 1, start)
}

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn search(
    obstructions: &HashSet<(i32, i32)>,
    width: i32,
    height: i32,
    start: (i32, i32),
) -> (bool, HashSet<(i32, i32, usize)>) {
    let mut d = 0;
    let (mut x, mut y) = start;
    let mut visited: HashSet<(i32, i32, usize)> = HashSet::new();
    let mut is_loop = false;
    while x >= 0 && x < width && y >= 0 && y < height {
        if visited.contains(&(x, y, d)) {
            is_loop = true;
            break;
        }
        visited.insert((x, y, d));
        let delta = DIRECTIONS[d];
        let p = (x + delta.0, y + delta.1);
        if obstructions.contains(&p) {
            d = (d + 1) % 4;
            continue;
        }
        (x, y) = p;
    }
    (is_loop, visited)
}

pub fn part_one(input: &str) -> usize {
    let (obstructions, width, height, start) = parse_input(input);
    let (_, visited) = search(&obstructions, width, height, start);
    visited
        .iter()
        .map(|&(x, y, _)| (x, y))
        .collect::<HashSet<(i32, i32)>>()
        .len()
}

pub fn part_two(input: &str) -> usize {
    let (mut obstructions, width, height, start) = parse_input(input);
    let (is_loop, visited) = search(&obstructions, width, height, start);
    (if is_loop { 1 } else { 0 })
        + visited
            .iter()
            .map(|&(x, y, _)| (x, y))
            .collect::<HashSet<(i32, i32)>>()
            .iter()
            .filter(|&position| {
                if obstructions.contains(position) {
                    return false;
                }
                obstructions.insert(*position);
                let (is_loop, _) =
                    search(&obstructions, width, height, start);
                obstructions.remove(position);
                is_loop
            })
            .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(6);
        assert_eq!(part_one(&input), 41);
        assert_eq!(part_two(&input), 6);
    }
}
