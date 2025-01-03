use std::collections::HashMap;
use std::collections::HashSet;

struct SecretNumber(i64);

impl SecretNumber {
    fn current(&self) -> i64 {
        self.0
    }

    fn next(&mut self) -> (i64, i64) {
        let mut n = self.0;
        n = (n * 64) ^ n;
        n %= 16777216;
        n = (n / 32) ^ n;
        n %= 16777216;
        n = (n * 2048) ^ n;
        n %= 16777216;

        std::mem::swap(&mut n, &mut self.0);
        (n, self.0)
    }
}

fn parse_input(input: &str) -> Vec<i64> {
    input.trim().lines().map(|s| s.parse().unwrap()).collect()
}

fn changes(list: Vec<i64>) -> Vec<(i64, i64)> {
    list.windows(2)
        .map(|v| (v[0] % 10, v[1] % 10))
        .map(|(a, b)| (b, b - a))
        .collect()
}

pub fn part_one(input: &str) -> i64 {
    parse_input(input)
        .into_iter()
        .map(|n| {
            let mut sn = SecretNumber(n);
            for _ in 0..2000 {
                sn.next();
            }
            sn.current()
        })
        .sum()
}

pub fn part_two(input: &str) -> i64 {
    let maps = parse_input(input)
        .iter()
        .map(|&n| {
            let mut sn = SecretNumber(n);
            let list = (0..=2000).map(|_| sn.next().0).collect::<Vec<_>>();
            changes(list)
                .windows(4)
                .map(|v| (v.iter().map(|v| v.1).collect::<Vec<_>>(), v[3].0))
                .map(|(k, v)| ([k[0], k[1], k[2], k[3]], v))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sums: HashMap<&[i64; 4], i64> = HashMap::new();
    let mut visited: HashSet<(usize, &[i64; 4])> = HashSet::new();
    for (i, m) in maps.iter().enumerate() {
        for (k, v) in m.iter() {
            if visited.insert((i, k)) {
                if let Some(s) = sums.get_mut(k) {
                    *s += *v;
                } else {
                    sums.insert(k, *v);
                }
            }
        }
    }
    *sums.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_next_secret_number() {
        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544,
            12249484, 7753432, 5908254,
        ];
        let mut sn = SecretNumber(123);
        let list: Vec<i64> = (0..10).map(|_| sn.next().1).collect();
        assert_eq!(list, expected);
    }

    #[test]
    fn example_changes() {
        let expected = vec![
            (0, -3),
            (6, 6),
            (5, -1),
            (4, -1),
            (4, 0),
            (6, 2),
            (4, -2),
            (4, 0),
            (2, -2),
            (4, 2),
        ];
        let mut sn = SecretNumber(123);
        let list = (0..=10).map(|_| sn.next().0).collect::<Vec<i64>>();
        let changes = changes(list);
        assert_eq!(expected, changes);
    }

    #[test]
    fn example() {
        let input = read_example(22);
        assert_eq!(part_one(&input), 37327623);

        let input = "1\n2\n3\n2024";
        assert_eq!(part_two(&input), 23);
    }
}
