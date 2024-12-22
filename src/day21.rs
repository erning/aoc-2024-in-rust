pub fn part_one(_input: &str) -> i32 {
    0
}

pub fn part_two(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(21);
        assert_eq!(part_one(&input), 126384);
        assert_eq!(part_two(&input), 0);
    }
}
