use std::collections::BinaryHeap;
use std::collections::HashSet;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|s| s.chars().collect()).collect()
}

fn find_min_score(maze: &[Vec<char>]) -> i32 {
    let mut queue: BinaryHeap<(i32, i32, i32, usize)> = BinaryHeap::new();
    let start = || -> (i32, i32) {
        for y in 0..maze.len() {
            for x in 0..maze[0].len() {
                if maze[y][x] == 'S' {
                    return (x as i32, y as i32);
                }
            }
        }
        (0, 0)
    }();
    queue.push((0, start.0, start.1, 0));
    let mut visited: HashSet<(i32, i32, usize)> = HashSet::new();
    let mut answer = 0;
    while let Some((score, x, y, d)) = queue.pop() {
        if maze[y as usize][x as usize] == 'E' {
            answer = -score;
            break;
        }
        if maze[y as usize][x as usize] == '#' {
            continue;
        }
        if visited.contains(&(x, y, d)) {
            continue;
        }
        visited.insert((x, y, d));
        queue.push((score - 1, x + DIRS[d].0, y + DIRS[d].1, d));
        queue.push((score - 1000, x, y, (d + 1) % 4));
        queue.push((score - 1000, x, y, (d + 3) % 4));
    }
    answer
}

pub fn part_one(input: &str) -> i32 {
    let maze = parse_input(input);
    find_min_score(&maze)
}

pub fn part_two(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_2() {
        const INPUT: &str = concat!(
            "#################\n",
            "#...#...#...#..E#\n",
            "#.#.#.#.#.#.#.#.#\n",
            "#.#.#.#...#...#.#\n",
            "#.#.#.#.###.#.#.#\n",
            "#...#.#.#.....#.#\n",
            "#.#.#.#.#.#####.#\n",
            "#.#...#.#.#.....#\n",
            "#.#.#####.#.###.#\n",
            "#.#.#.......#...#\n",
            "#.#.###.#####.###\n",
            "#.#.#...#.....#.#\n",
            "#.#.#.#####.###.#\n",
            "#.#.#.........#.#\n",
            "#.#.#.#########.#\n",
            "#S#.............#\n",
            "#################\n",
        );
        let maze = parse_input(INPUT);
        let score = find_min_score(&maze);
        assert_eq!(score, 11048);
    }

    #[test]
    fn example() {
        let input = read_example(16);
        assert_eq!(part_one(&input), 7036);
        assert_eq!(part_two(&input), 0);
    }
}
