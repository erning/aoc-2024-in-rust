use std::collections::HashSet;
use std::collections::VecDeque;

type Pos = (i16, i16);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap_or(128) as u8)
                .collect()
        })
        .collect()
}

fn find_trailheads(map: &[Vec<u8>]) -> Vec<Pos> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, v)| **v == 0)
                .map(|(x, _)| (x as i16, y as i16))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_trailhead_targets(map: &[Vec<u8>], head: Pos) -> Vec<Pos> {
    const DIRS: [Pos; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let h = map.len() as i16;
    let w = map[0].len() as i16;
    let mut targets = Vec::new();
    let mut queue: VecDeque<Pos> = VecDeque::new();
    queue.push_back(head);
    while let Some((x, y)) = queue.pop_front() {
        let v1 = map[y as usize][x as usize];
        if v1 == 9 {
            targets.push((x, y));
            continue;
        }
        for (dx, dy) in DIRS {
            let (x, y) = (x + dx, y + dy);
            if x < 0 || x >= w || y < 0 || y >= h {
                continue;
            }
            let v2 = map[y as usize][x as usize];
            if v1 + 1 != v2 {
                continue;
            }
            queue.push_back((x, y));
        }
    }
    targets
}

fn find_trailhead_score(map: &[Vec<u8>], head: Pos) -> usize {
    find_trailhead_targets(map, head)
        .into_iter()
        .collect::<HashSet<_>>()
        .len()
}

fn find_trailhead_rating(map: &[Vec<u8>], head: Pos) -> usize {
    find_trailhead_targets(map, head).len()
}

pub fn part_one(input: &str) -> usize {
    let map = parse_input(input);
    let heads = find_trailheads(&map);
    heads
        .into_iter()
        .map(|head| find_trailhead_score(&map, head))
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let map = parse_input(input);
    let heads = find_trailheads(&map);
    heads
        .into_iter()
        .map(|head| find_trailhead_rating(&map, head))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[rustfmt::skip]
    const INPUT_1: &str = concat!(
        "0123\n",
        "1234\n",
        "8765\n",
        "9876\n"
    );

    #[rustfmt::skip]
    const INPUT_2: &str = concat!(
        "...0...\n",
        "...1...\n",
        "...2...\n",
        "6543456\n",
        "7.....7\n",
        "8.....8\n",
        "9.....9\n"
    );

    #[rustfmt::skip]
    const INPUT_3: &str = concat!(
        "..90..9\n",
        "...1.98\n",
        "...2..7\n",
        "6543456\n",
        "765.987\n",
        "876....\n",
        "987....\n",
    );

    #[rustfmt::skip]
    const INPUT_4: &str = concat!(
        "10..9..\n",
        "2...8..\n",
        "3...7..\n",
        "4567654\n",
        "...8..3\n",
        "...9..2\n",
        ".....01\n",
    );

    #[test]
    fn example_find_trailheads() {
        assert_eq!(find_trailheads(&parse_input(INPUT_1)).len(), 1);
        assert_eq!(find_trailheads(&parse_input(INPUT_2)).len(), 1);
        assert_eq!(find_trailheads(&parse_input(INPUT_3)).len(), 1);
        assert_eq!(find_trailheads(&parse_input(INPUT_4)).len(), 2);

        let input = read_example(10);
        let map = parse_input(&input);
        let heads = find_trailheads(&map);
        assert_eq!(heads.len(), 9);
    }

    #[test]
    fn example_find_trailheads_score_1() {
        let map = parse_input(INPUT_1);
        let heads = find_trailheads(&map);
        assert_eq!(find_trailhead_score(&map, heads[0]), 1);
    }

    #[test]
    fn example_find_trailheads_score_2() {
        let map = parse_input(INPUT_2);
        let heads = find_trailheads(&map);
        assert_eq!(find_trailhead_score(&map, heads[0]), 2);
    }

    #[test]
    fn example_find_trailheads_score_3() {
        let map = parse_input(INPUT_3);
        let heads = find_trailheads(&map);
        assert_eq!(find_trailhead_score(&map, heads[0]), 4);
    }

    #[test]
    fn example_find_trailheads_score_4() {
        let map = parse_input(INPUT_4);
        assert_eq!(find_trailhead_score(&map, (1, 0)), 1);
        assert_eq!(find_trailhead_score(&map, (5, 6)), 2);
    }

    #[test]
    fn example() {
        let input = read_example(10);
        assert_eq!(part_one(&input), 36);
        assert_eq!(part_two(&input), 81);
    }
}
