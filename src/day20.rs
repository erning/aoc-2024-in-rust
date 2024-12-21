use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

type Pos = (i32, i32);
type Grid = HashMap<Pos, char>;

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse_input(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars()
                .enumerate()
                .map(|(x, c)| ((x as i32, y as i32), c))
                .collect::<HashMap<_, _>>()
        })
        .collect()
}

fn find_char(grid: &Grid, ch: char) -> Option<Pos> {
    grid.iter().find(|(_, &c)| c == ch).map(|(&p, _)| p)
}

fn distance_from_position(grid: &Grid, position: Pos) -> HashMap<Pos, usize> {
    let mut visited: HashMap<Pos, usize> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(usize, Pos)>> = BinaryHeap::new();
    queue.push(Reverse((0, position)));
    while let Some(Reverse((d, p))) = queue.pop() {
        match grid.get(&p) {
            Some('#') => continue,
            Some(_) => {}
            None => continue,
        }
        if visited.contains_key(&p) {
            continue;
        }
        visited.insert(p, d);
        for (dx, dy) in DIRS {
            let next = (p.0 + dx, p.1 + dy);
            if grid.contains_key(&next) {
                queue.push(Reverse((d + 1, next)));
            }
        }
    }
    visited
}

fn cheats_count(
    distances_from_start: &HashMap<Pos, usize>,
    cheat_distance: usize,
    max_save_distance: usize,
) -> HashMap<usize, usize> {
    let mut counts: HashMap<usize, usize> = HashMap::new();

    let mut delta: Vec<Pos> = Vec::new();
    for d in 1..=cheat_distance as i32 {
        delta.extend([(d, 0), (-d, 0), (0, d), (0, -d)]);
        delta.extend((1..d).map(|dx| (dx, d - dx)).flat_map(|(dx, dy)| {
            vec![(dx, dy), (dx, -dy), (-dx, dy), (-dx, -dy)]
        }));
    }

    for (&p, &d) in distances_from_start.iter() {
        for (dx, dy) in delta.iter() {
            let p2 = (p.0 + dx, p.1 + dy);
            if !distances_from_start.contains_key(&p2) {
                continue;
            }
            let d2 = *distances_from_start.get(&p2).unwrap();
            let dc = (dx.abs() + dy.abs()) as usize;
            if d + dc + max_save_distance <= d2 {
                let saved = d2 - d - dc;
                counts.entry(saved).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }

    counts
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let start = find_char(&grid, 'S').unwrap();
    let dists = distance_from_position(&grid, start);
    let mcd = 2;
    let msd = 100;
    let counts = cheats_count(&dists, mcd, msd);
    counts
        .iter()
        .filter(|(saved, _)| saved >= &&msd)
        .map(|(_, count)| count)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let grid = parse_input(input);
    let start = find_char(&grid, 'S').unwrap();
    let dists = distance_from_position(&grid, start);
    let mcd = 20;
    let msd = 100;
    let counts = cheats_count(&dists, mcd, msd);
    counts
        .iter()
        .filter(|(saved, _)| saved >= &&msd)
        .map(|(_, count)| count)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(20);
        let grid = parse_input(&input);
        let start = find_char(&grid, 'S').unwrap();
        let dists = distance_from_position(&grid, start);
        // part_one
        let counts = cheats_count(&dists, 2, 0);
        assert_eq!(counts[&2], 14);
        assert_eq!(counts[&6], 2);
        assert_eq!(counts[&8], 4);
        assert_eq!(counts[&10], 2);
        assert_eq!(counts[&12], 3);
        assert_eq!(counts[&20], 1);
        assert_eq!(counts[&36], 1);
        assert_eq!(counts[&38], 1);
        assert_eq!(counts[&40], 1);
        assert_eq!(counts[&64], 1);
        // part_two
        let counts = cheats_count(&dists, 20, 50);
        assert_eq!(counts[&50], 32);
        assert_eq!(counts[&52], 31);
        assert_eq!(counts[&54], 29);
        assert_eq!(counts[&56], 39);
        assert_eq!(counts[&58], 25);
        assert_eq!(counts[&60], 23);
        assert_eq!(counts[&62], 20);
        assert_eq!(counts[&64], 19);
        assert_eq!(counts[&66], 12);
        assert_eq!(counts[&68], 14);
        assert_eq!(counts[&70], 12);
        assert_eq!(counts[&72], 22);
        assert_eq!(counts[&74], 4);
        assert_eq!(counts[&76], 3);
    }
}
