use std::collections::HashSet;

type Grid = Vec<Vec<char>>;
type Pos = (i32, i32);
const DIRS: [Pos; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|s| s.chars().collect()).collect()
}

fn plantap(
    grid: &Grid,
    x: i32,
    y: i32,
    visited: &mut HashSet<Pos>,
) -> (usize, usize) {
    visited.insert((x, y));
    let mut area = 1;
    let mut perimeter = 4;
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    let ch = grid[y as usize][x as usize];
    for (dx, dy) in DIRS {
        let (x, y) = (x + dx, y + dy);
        if x < 0 || x >= w || y < 0 || y >= h {
            continue;
        }
        let c = grid[y as usize][x as usize];
        if ch != c {
            continue;
        }
        perimeter -= 1;
        if !visited.contains(&(x, y)) {
            let v = plantap(grid, x, y, visited);
            area += v.0;
            perimeter += v.1;
        }
    }
    (area, perimeter)
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut total_price = 0;
    let mut visited: HashSet<Pos> = HashSet::new();
    for y in 0..h {
        for x in 0..w {
            if visited.contains(&(x, y)) {
                continue;
            }
            let (area, perimeter) = plantap(&grid, x, y, &mut visited);
            let price = area * perimeter;
            total_price += price;
        }
    }
    total_price
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
        let mut visited: HashSet<Pos> = HashSet::new();
        let (area, perimeter) = plantap(&grid, 0, 0, &mut visited);
        assert_eq!(area, 21);
        assert_eq!(perimeter, 36);
        let (area, perimeter) = plantap(&grid, 1, 1, &mut visited);
        assert_eq!(area, 1);
        assert_eq!(perimeter, 4);
    }

    #[test]
    fn example() {
        let input = read_example(12);
        assert_eq!(part_one(&input), 1930);
        assert_eq!(part_two(&input), 1206);
    }
}
