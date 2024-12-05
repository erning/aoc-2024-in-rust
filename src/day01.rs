use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    input.trim().lines().for_each(|s| {
        let row = s
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        assert!(row.len() == 2);
        a.push(row[0]);
        b.push(row[1]);
    });
    assert!(a.len() == b.len());
    (a, b)
}

pub fn part_one(input: &str) -> i32 {
    let (mut ca, mut cb) = parse_input(input);
    ca.sort_unstable();
    cb.sort_unstable();
    ca.into_iter().zip(cb).map(|(a, b)| (a - b).abs()).sum()
}

pub fn part_two(input: &str) -> i32 {
    let (ca, cb) = parse_input(input);
    let mut count: HashMap<i32, i32> = HashMap::new();
    cb.into_iter().for_each(|v| {
        *count.entry(v).or_insert(0) += 1;
    });
    ca.iter().map(|v| v * count.get(v).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(1);
        assert_eq!(part_one(&input), 11);
        assert_eq!(part_two(&input), 31);
    }
}
