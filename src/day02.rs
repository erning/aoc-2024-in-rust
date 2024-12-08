fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|s| s.split_whitespace().map(|v| v.parse().unwrap()).collect())
        .collect()
}

fn is_safe(row: &[i32], ignore: usize) -> bool {
    let row: Vec<i32> = row
        .iter()
        .enumerate()
        .filter(|(i, _)| i != &ignore)
        .map(|(_, &v)| v)
        .collect();
    let mut iter = row.windows(2).map(|v| v[0] - v[1]);
    (iter.clone().all(|v| v < 0) || iter.clone().all(|v| v > 0))
        && iter.all(|v| (1..=3).contains(&v.abs()))
}

pub fn part_one(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|row| is_safe(row, row.len()))
        .count()
}

pub fn part_two(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|row| (0..=row.len()).any(|ignore| is_safe(row, ignore)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(2);
        assert_eq!(part_one(&input), 2);
        assert_eq!(part_two(&input), 4);
    }
}
