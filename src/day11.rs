use std::collections::HashMap;

type Cache = HashMap<(u64, u8), usize>;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn split_even_digits(v: u64) -> Option<(u64, u64)> {
    let s = v.to_string();
    let n = s.len();
    if n % 2 == 0 {
        let m = n / 2;
        let a = s[..m].parse().unwrap();
        let b = s[m..].parse().unwrap();
        Some((a, b))
    } else {
        None
    }
}

fn blink_stones(stones: &mut Vec<u64>) {
    let n = stones.len();
    for i in 0..n {
        let v = &mut stones[i];
        if *v == 0 {
            *v = 1;
        } else if let Some((a, b)) = split_even_digits(*v) {
            *v = a;
            stones.push(b);
        } else {
            stones[i] *= 2024;
        }
    }
}

fn blink_stone(cache: &mut Cache, v: u64, times: u8) -> usize {
    if times == 0 {
        return 1;
    }
    if let Some(&n) = cache.get(&(v, times)) {
        return n;
    }
    let mut stones = vec![v];
    blink_stones(&mut stones);
    let n = stones
        .into_iter()
        .map(|v| blink_stone(cache, v, times - 1))
        .sum();
    cache.insert((v, times), n);
    n
}

fn blink_times(stones: &[u64], times: u8) -> usize {
    let mut cache = Cache::new();
    stones
        .iter()
        .map(|&v| blink_stone(&mut cache, v, times))
        .sum()
}

pub fn part_one(input: &str) -> usize {
    let mut stones = parse_input(input);
    for _ in 0..25 {
        blink_stones(&mut stones);
    }
    stones.len()
}

pub fn part_two(input: &str) -> usize {
    let stones = parse_input(input);
    blink_times(&stones, 75)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_1() {
        let mut stones: Vec<u64> = vec![0, 1, 10, 99, 999];
        blink_stones(&mut stones);
        assert_eq!(stones, vec![1, 2024, 1, 9, 2021976, 0, 9]);
    }
    #[test]
    fn example_2() {
        let mut stones: Vec<u64> = vec![125, 17];
        for n in [3, 4, 5, 9, 13, 22] {
            blink_stones(&mut stones);
            assert_eq!(stones.len(), n);
        }
    }

    #[test]
    fn example() {
        let input = read_example(11);
        assert_eq!(part_one(&input), 55312);
        let stones = parse_input(&input);
        assert_eq!(blink_times(&stones, 25), 55312);
    }
}
