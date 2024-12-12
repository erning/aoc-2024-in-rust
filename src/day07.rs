fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .trim()
        .lines()
        .map(|s| s.splitn(2, ':').map(|s| s.trim()).collect::<Vec<&str>>())
        .map(|s| {
            (
                s[0].parse::<i64>().unwrap(),
                s[1].split_whitespace()
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn total_calibration(
    input: &str,
    f: fn(i64, &[i64], i64, usize) -> bool,
) -> i64 {
    parse_input(input)
        .iter()
        .filter_map(|(expect, numbers)| {
            match f(*expect, numbers, numbers[0], 1) {
                true => Some(*expect),
                false => None,
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> i64 {
    fn calc(expect: i64, numbers: &[i64], value: i64, i: usize) -> bool {
        if i >= numbers.len() {
            value == expect
        } else {
            calc(expect, numbers, value + numbers[i], i + 1)
                || calc(expect, numbers, value * numbers[i], i + 1)
        }
    }
    total_calibration(input, calc)
}

pub fn part_two(input: &str) -> i64 {
    fn calc(expect: i64, numbers: &[i64], value: i64, i: usize) -> bool {
        if i >= numbers.len() {
            value == expect
        } else {
            calc(expect, numbers, value + numbers[i], i + 1)
                || calc(expect, numbers, value * numbers[i], i + 1)
                || {
                    let mut a = value * 10;
                    let mut b = numbers[i];
                    while b >= 10 {
                        b /= 10;
                        a *= 10;
                    }
                    calc(expect, numbers, a + numbers[i], i + 1)
                }
        }
    }
    total_calibration(input, calc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(7);
        assert_eq!(part_one(&input), 3749);
        assert_eq!(part_two(&input), 11387);
    }
}
