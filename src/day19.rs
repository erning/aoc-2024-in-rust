use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.trim().lines();
    let patterns = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.trim())
        .collect();
    let designs = lines.skip(1).collect();
    (patterns, designs)
}

fn is_possible(design: &str, patterns: &[&str]) -> bool {
    design.is_empty()
        || patterns.iter().any(|pattern| {
            design.starts_with(pattern)
                && is_possible(&design[pattern.len()..], patterns)
        })
}

fn all_possible<'a>(
    design: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&count) = cache.get(&design) {
        count
    } else {
        let count = patterns
            .iter()
            .filter(|&&pattern| design.starts_with(pattern))
            .map(|pattern| {
                all_possible(&design[pattern.len()..], patterns, cache)
            })
            .sum();
        cache.insert(design, count);
        count
    }
}

#[allow(dead_code)]
fn is_possible_dp(design: &str, patterns: &[&str]) -> bool {
    let n = design.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..n + 1 {
        patterns
            .iter()
            .filter(|pattern| i >= pattern.len())
            .map(|pattern| (i - pattern.len(), pattern))
            .filter(|(j, pattern)| design[*j..].starts_with(*pattern))
            .for_each(|(j, _)| dp[i] |= dp[j]);
    }
    dp[n]
}

#[allow(dead_code)]
fn all_possible_dp(design: &str, patterns: &[&str]) -> usize {
    let n = design.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 1..n + 1 {
        patterns
            .iter()
            .filter(|pattern| i >= pattern.len())
            .map(|pattern| (i - pattern.len(), pattern))
            .filter(|(j, pattern)| design[*j..].starts_with(*pattern))
            .for_each(|(j, _)| dp[i] += dp[j]);
    }
    dp[n]
}

pub fn part_one(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    designs
        .iter()
        .filter(|design| is_possible(design, &patterns))
        .count()
}

pub fn part_two(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    let mut cache: HashMap<&str, usize> = HashMap::new();
    designs
        .iter()
        .map(|design| all_possible(design, &patterns, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_dp() {
        let input = read_example(19);
        let (patterns, designs) = parse_input(&input);
        let p1: usize = designs
            .iter()
            .filter(|design| is_possible_dp(design, &patterns))
            .count();
        let p2: usize = designs
            .iter()
            .map(|design| all_possible_dp(design, &patterns))
            .sum();
        assert_eq!(p1, 6);
        assert_eq!(p2, 16);
    }

    #[test]
    fn example() {
        let input = read_example(19);
        assert_eq!(part_one(&input), 6);
        assert_eq!(part_two(&input), 16);
    }
}
