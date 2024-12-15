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

    fn example_smaller() {
        const INPUT_SMALLER: &str = concat!(
            "########\n",
            "#..O.O.#\n",
            "##@.O..#\n",
            "#...O..#\n",
            "#.#.O..#\n",
            "#...O..#\n",
            "#......#\n",
            "########\n",
            "\n",
            "<^^>>>vv<v>>v<<"
        );
        // for this smaller example,
        // the sum of all boxes' GPS coordinates is
        // 2028
    }

    #[test]
    fn example() {
        let input = read_example(15);
        assert_eq!(part_one(&input), 10092);
        assert_eq!(part_two(&input), 0);
    }
}
