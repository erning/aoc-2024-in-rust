use std::cmp::Ordering;
use std::collections::HashSet;

fn parse_input(input: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let i = input.find("\n\n").unwrap();
    let rules: HashSet<(i32, i32)> = input[..i]
        .trim()
        .lines()
        .map(|s| {
            s.splitn(2, '|')
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| (v[0], v[1]))
        .collect();
    let updates: Vec<Vec<i32>> = input[i..]
        .trim()
        .lines()
        .map(|s| s.split(',').map(|v| v.parse::<i32>().unwrap()).collect())
        .collect();
    (rules, updates)
}

pub fn part_one(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);
    updates
        .iter()
        .map(|row| {
            (0..row.len() - 1)
                .all(|i| {
                    (i + 1..row.len())
                        .map(|j| (row[i], row[j]))
                        .all(|(a, b)| rules.contains(&(a, b)))
                })
        })
        .enumerate()
        .filter(|&(_, is_correct)| is_correct)
        .map(|(i, _)| &updates[i])
        .map(|row| row[row.len() / 2])
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);
    updates
        .iter()
        .map(|row| {
            (0..row.len() - 1)
                .all(|i| {
                    (i + 1..row.len())
                        .map(|j| (row[i], row[j]))
                        .all(|(a, b)| rules.contains(&(a, b)))
                })
        })
        .enumerate()
        .filter(|&(_, is_correct)| !is_correct)
        .map(|(i, _)| {
            let mut row = updates[i].clone();
            row.sort_unstable_by(|a, b| match rules.contains(&(*a, *b)) {
                true => Ordering::Less,
                false => Ordering::Greater,
            });
            row
        })
        .map(|row| row[row.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(5);
        assert_eq!(part_one(&input), 143);
        assert_eq!(part_two(&input), 123);
    }
}
