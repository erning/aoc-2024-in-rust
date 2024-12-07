use std::sync::RwLockWriteGuard;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|s| s.split_whitespace().map(|v| v.parse().unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|row| row.windows(2).map(|v| v[0] - v[1]).collect::<Vec<i32>>())
        .filter(|row| {
            row.iter().all(|&v| v < 0) || row.iter().all(|&v| v > 0)
        })
        .filter(|row| row.iter().all(|&v| (1..=3).contains(&v.abs())))
        .count()
}

pub fn part_two(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(2);
        assert_eq!(part_one(&input), 2);
        assert_eq!(part_two(&input), 0);
    }
}
