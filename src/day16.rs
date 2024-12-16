use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
type Pos = (i32, i32);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|s| s.chars().collect()).collect()
}

fn find_ch(maze: &[Vec<char>], c: char) -> Option<Pos> {
    maze.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &ch)| ch == c)
                .map(|(x, _)| (x as i32, y as i32))
                .collect::<Vec<_>>()
        })
        .next()
}

fn find_min_score(maze: &[Vec<char>], best_path: bool) -> (i32, usize) {
    let h = maze.len() as i32;
    let w = maze[0].len() as i32;
    let mut queue: BinaryHeap<(i32, Pos, usize, Vec<Pos>)> =
        BinaryHeap::new();
    let start = find_ch(maze, 'S').unwrap();
    let end = find_ch(maze, 'E').unwrap();
    queue.push((0, start, 0, vec![start]));
    let mut visited: HashMap<(Pos, usize), i32> = HashMap::new();
    let mut answer = i32::MIN;
    let mut tails: HashSet<Pos> = HashSet::new();
    while let Some((score, p, d, t)) = queue.pop() {
        if p.0 < 0 || p.0 >= w || p.1 < 0 || p.1 > h {
            continue;
        }
        if maze[p.1 as usize][p.0 as usize] == '#' {
            continue;
        }
        if score < answer {
            break;
        }
        if p == end {
            answer = score;
            if !best_path {
                break;
            }
            tails.extend(t.iter().cloned());
            continue;
        }
        if let Some(visited_score) = visited.get(&(p, d)) {
            if !best_path || score < *visited_score {
                continue;
            }
        }
        visited.insert((p, d), score);
        let next = (p.0 + DIRS[d].0, p.1 + DIRS[d].1);
        let mut next_tiles = t.clone();
        if best_path {
            next_tiles.push(next);
        }
        queue.push((score - 1, next, d, next_tiles));
        queue.push((score - 1000, p, (d + 1) % 4, t.clone()));
        queue.push((score - 1000, p, (d + 3) % 4, t.clone()));
    }
    (-answer, tails.len())
}

pub fn part_one(input: &str) -> i32 {
    let maze = parse_input(input);
    let (score, _) = find_min_score(&maze, false);
    score
}

pub fn part_two(input: &str) -> usize {
    let maze = parse_input(input);
    let (_, tiles) = find_min_score(&maze, true);
    tiles
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
        let (score, tiles) = find_min_score(&maze, true);
        assert_eq!(score, 11048);
        assert_eq!(tiles, 64);
    }

    #[test]
    fn example() {
        let input = read_example(16);
        assert_eq!(part_one(&input), 7036);
        assert_eq!(part_two(&input), 45);
    }
}
