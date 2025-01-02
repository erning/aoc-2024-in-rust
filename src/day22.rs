struct SecretNumber(u64);

impl SecretNumber {
    fn current(&self) -> u64 {
        self.0
    }
    fn next(&mut self) -> u64 {
        let mut n = self.0;
        n = (n * 64) ^ n;
        n %= 16777216;
        n = (n / 32) ^ n;
        n %= 16777216;
        n = (n * 2048) ^ n;
        n %= 16777216;
        self.0 = n;
        n
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input.trim().lines().map(|s| s.parse().unwrap()).collect()
}

pub fn part_one(input: &str) -> u64 {
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

pub fn part_two(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_next_secret_number() {
        let mut sn = SecretNumber(123);
        let list: Vec<u64> = (0..10).map(|_| sn.next()).collect();
        println!("{:?}", list);
    }

    #[test]
    fn example() {
        let input = read_example(22);
        assert_eq!(part_one(&input), 37327623);
        assert_eq!(part_two(&input), 0);
    }
}
