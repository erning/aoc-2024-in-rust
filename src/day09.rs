fn checksum(disk: &[u32]) -> u32 {
    disk.iter().enumerate().map(|(i, v)| (i as u32) * v).sum()
}

pub fn part_one(input: &str) -> i32 {
    0
}

pub fn part_two(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_checksum() {
        let disk: Vec<u32> = "0099811188827773336446555566.............."
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(0))
            .collect();
        assert_eq!(checksum(&disk), 1928);
    }

    #[test]
    fn example() {
        let input = read_example(9);
        assert_eq!(part_one(&input), 1928);
        assert_eq!(part_two(&input), 0);
    }
}
