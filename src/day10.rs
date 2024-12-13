fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap_or(255) as u8)
                .collect()
        })
        .collect()
}

fn find_trailheads(map: &[Vec<u8>]) -> Vec<(i32, i32)> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, v)| **v == 0)
                .map(|(x, _)| (x as i32, y as i32))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_trailhead_score(map: &[Vec<u8>], head: (i32, i32)) -> usize {
    0
}

pub fn part_one(input: &str) -> usize {
    let map = parse_input(input);
    0
}

pub fn part_two(input: &str) -> usize {
    0
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
        let map = parse_input(INPUT_3);
        assert_eq!(find_trailhead_score(&map, (0, 1)), 1);
        assert_eq!(find_trailhead_score(&map, (5, 6)), 2);
    }

    #[test]
    fn example() {
        let input = read_example(10);
        assert_eq!(part_one(&input), 36);
        assert_eq!(part_two(&input), 0);
    }
}
