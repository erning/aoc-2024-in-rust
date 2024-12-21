use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

type Pos = (i32, i32);
type Grid = HashMap<Pos, char>;
type Distances = HashMap<Pos, i32>;
type Counts = HashMap<i32, usize>;

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

fn build_dists(grid: &Grid, position: Pos) -> Distances {
    let mut dists = Distances::new();
    let mut queue: BinaryHeap<Reverse<(i32, Pos)>> = BinaryHeap::new();
    queue.push(Reverse((0, position)));
    while let Some(Reverse((d, p))) = queue.pop() {
        match grid.get(&p) {
            Some('#') => continue,
            Some(_) => {}
            None => continue,
        }
        if dists.contains_key(&p) {
            continue;
        }
        dists.insert(p, d);
        for (dx, dy) in DIRS {
            let next = (p.0 + dx, p.1 + dy);
            if grid.contains_key(&next) {
                queue.push(Reverse((d + 1, next)));
            }
        }
    }
    dists
}

fn cheats_count(dists: &Distances, mcd: i32, msd: i32) -> Counts {
    let mut counts = Counts::new();

    let delta = {
        let mut v: Vec<Pos> = Vec::new();
        for i in 1..=mcd {
            v.extend([(i, 0), (-i, 0), (0, i), (0, -i)]);
            v.extend((1..i).map(|dx| (dx, i - dx)).flat_map(|(dx, dy)| {
                vec![(dx, dy), (dx, -dy), (-dx, dy), (-dx, -dy)]
            }));
        }
        v
    };

    dists.iter().for_each(|(p1, d1)| {
        delta
            .iter()
            .map(|(dx, dy)| ((p1.0 + dx, p1.1 + dy), dx.abs() + dy.abs()))
            .filter(|(p2, _)| dists.contains_key(p2))
            .map(|(p2, cd)| (p2, dists[&p2], cd))
            .map(|(_, d2, cd)| d2 - d1 - cd)
            .filter(|&saved| saved >= msd)
            .for_each(|saved| {
                counts.entry(saved).and_modify(|e| *e += 1).or_insert(1);
            })
    });

    counts
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let start = find_char(&grid, 'S').unwrap();
    let dists = build_dists(&grid, start);
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
    let dists = build_dists(&grid, start);
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
        let dists = build_dists(&grid, start);
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
