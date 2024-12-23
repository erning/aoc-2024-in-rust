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
        let input = read_example(22);
        //assert_eq!(part_one(&input), 37327623);
        assert_eq!(part_two(&input), 0);
    }
}
