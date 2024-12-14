use std::collections::HashSet;
use std::collections::VecDeque;

type Grid = Vec<Vec<char>>;
type Pos = (i32, i32);
const DIRS: [Pos; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|s| s.chars().collect()).collect()
}

fn find_region(grid: &Grid, x: i32, y: i32) -> HashSet<Pos> {
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    let mut region: HashSet<Pos> = HashSet::new();
    let ch = grid[y as usize][x as usize];
    let mut queue: VecDeque<Pos> = VecDeque::from([(x, y)]);
    while let Some((x, y)) = queue.pop_front() {
        if !region.insert((x, y)) {
            continue;
        }
        for (dx, dy) in DIRS {
            let (x, y) = (x + dx, y + dy);
            if x < 0 || x >= w || y < 0 || y >= h {
                continue;
            }
            if ch != grid[y as usize][x as usize] {
                continue;
            }
            if region.contains(&(x, y)) {
                continue;
            }
            queue.push_back((x, y));
        }
    }
    region
}

fn find_regions(grid: &Grid) -> Vec<HashSet<Pos>> {
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    let mut regions = Vec::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    for y in 0..h {
        for x in 0..w {
            if visited.contains(&(x, y)) {
                continue;
            }
            let region = find_region(grid, x, y);
            visited.extend(region.iter().cloned());
            regions.push(region);
        }
    }
    regions
}

fn region_perimeter(region: &HashSet<Pos>) -> usize {
    let mut v = 0;
    for (x, y) in region.iter() {
        for (dx, dy) in DIRS {
            let (x, y) = (x + dx, y + dy);
            if region.contains(&(x, y)) {
                v += 1;
            }
        }
    }
    region.len() * 4 - v
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    find_regions(&grid)
        .into_iter()
        .map(|region| region.len() * region_perimeter(&region))
        .sum()
}

pub fn part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_1() {
        #[rustfmt::skip]
        const INPUT: &str =
            concat!(
                "OOOOO\n",
                "OXOXO\n",
                "OOOOO\n",
                "OXOXO\n",
                "OOOOO\n"
            );
        let grid = parse_input(INPUT);
        let region = find_region(&grid, 0, 0);
        assert_eq!(region.len(), 21);
        assert_eq!(region_perimeter(&region), 36);
        let region = find_region(&grid, 1, 1);
        assert_eq!(region.len(), 1);
        assert_eq!(region_perimeter(&region), 4);
    }

    #[test]
    fn example() {
        let input = read_example(12);
        assert_eq!(part_one(&input), 1930);
        assert_eq!(part_two(&input), 1206);
    }
}
