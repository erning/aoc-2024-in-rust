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
) -> HashSet<(i32, i32)> {
    let mut d = 0;
    let (mut x, mut y) = start;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    while x >= 0 && x < width && y >= 0 && y < height {
        visited.insert((x, y));
        let delta = DIRECTIONS[d];
        let p = (x + delta.0, y + delta.1);
        if obstructions.contains(&p) {
            d = (d + 1) % 4;
            continue;
        }
        (x, y) = p;
    }
    visited
}

pub fn part_one(input: &str) -> usize {
    let (obstructions, width, height, start) = parse_input(input);
    search(&obstructions, width, height, start).len()
}

pub fn part_two(input: &str) -> usize {
    let (obstructions, width, height, start) = parse_input(input);
    search(&obstructions, width, height, start)
        .iter()
        .filter(|position| !obstructions.contains(position))
        .filter(|&position| {
            let mut d = 0;
            let (mut x, mut y) = start;
            let mut visited: HashSet<(i32, i32, usize)> = HashSet::new();
            let mut found = false;
            while x >= 0 && x < width && y >= 0 && y < height {
                if visited.contains(&(x, y, d)) {
                    found = true;
                    break;
                }
                visited.insert((x, y, d));
                let delta = DIRECTIONS[d];
                let p = (x + delta.0, y + delta.1);
                if p == *position || obstructions.contains(&p) {
                    d = (d + 1) % 4;
                    continue;
                }
                (x, y) = p;
            }
            found
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
