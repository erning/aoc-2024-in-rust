use std::collections::HashSet;
use std::collections::VecDeque;

type Grid = Vec<Vec<char>>;
type Pos = (i32, i32);
const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|s| s.chars().collect()).collect()
}

fn flood_region(grid: &Grid, x: i32, y: i32) -> HashSet<Pos> {
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
            let region = flood_region(grid, x, y);
            visited.extend(region.iter().cloned());
            regions.push(region);
        }
    }
    regions
}

fn region_perimeter(region: &HashSet<Pos>) -> usize {
    let connected_edges: usize = region
        .iter()
        .map(|(x, y)| {
            DIRS.map(|(dx, dy)| region.contains(&(x + dx, y + dy)))
                .into_iter()
                .filter(|&v| v)
                .count()
        })
        .sum();
    region.len() * 4 - connected_edges
}

fn region_sides(region: &HashSet<Pos>) -> usize {
    // https://www.youtube.com/watch?v=KXwKGWSQvS0
    let region: HashSet<Pos> =
        region.iter().map(|(x, y)| (x * 2 + 1, y * 2 + 1)).collect();
    const CORNERS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];

    let corners: HashSet<Pos> = region
        .iter()
        .flat_map(|(x, y)| CORNERS.map(|(dx, dy)| (x + dx, y + dy)))
        .collect();
    corners
        .iter()
        .map(|(x, y)| {
            CORNERS.map(|(dx, dy)| region.contains(&(x + dx, y + dy)))
        })
        .map(|v| match v.iter().filter(|&&v| v).count() {
            1 | 3 => 1,
            2 if v[0] == v[2] => 2,
            _ => 0,
        })
        .sum()
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    find_regions(&grid)
        .into_iter()
        .map(|region| region.len() * region_perimeter(&region))
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let grid = parse_input(input);
    find_regions(&grid)
        .into_iter()
        .map(|region| region.len() * region_sides(&region))
        .sum()
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

        let region = flood_region(&grid, 0, 0);
        assert_eq!(region.len(), 21);
        assert_eq!(region_perimeter(&region), 36);

        let region = flood_region(&grid, 1, 1);
        assert_eq!(region.len(), 1);
        assert_eq!(region_perimeter(&region), 4);
    }

    #[test]
    fn example_2() {
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

        let region = flood_region(&grid, 0, 0);
        assert_eq!(region.len(), 21);
        assert_eq!(region_perimeter(&region), 36);

        let region = flood_region(&grid, 1, 1);
        assert_eq!(region.len(), 1);
        assert_eq!(region_perimeter(&region), 4);
    }

    #[test]
    fn example_3() {
        #[rustfmt::skip]
        const INPUT: &str =
            concat!(
                "AAAA\n",
                "BBCD\n",
                "BBCC\n",
                "EEEC\n",
            );
        let grid = parse_input(INPUT);
        // A
        let region = flood_region(&grid, 0, 0);
        assert_eq!(region.len(), 4);
        assert_eq!(region_sides(&region), 4);
        // B
        let region = flood_region(&grid, 0, 1);
        assert_eq!(region.len(), 4);
        assert_eq!(region_sides(&region), 4);
        // C
        let region = flood_region(&grid, 2, 1);
        assert_eq!(region.len(), 4);
        assert_eq!(region_sides(&region), 8);
        // D
        let region = flood_region(&grid, 3, 1);
        assert_eq!(region.len(), 1);
        assert_eq!(region_sides(&region), 4);
        // E
        let region = flood_region(&grid, 0, 3);
        assert_eq!(region.len(), 3);
        assert_eq!(region_sides(&region), 4);
    }

    #[test]
    fn example() {
        let input = read_example(12);
        assert_eq!(part_one(&input), 1930);
        assert_eq!(part_two(&input), 1206);
    }
}
