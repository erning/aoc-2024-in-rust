use regex::Regex;

pub fn part_one(input: &str) -> i32 {
    let pattern = r"mul\((\d{1,3},\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();
    re.captures_iter(input)
        .map(|caps| {
            caps.get(1)
                .unwrap()
                .as_str()
                .splitn(2, ',')
                .map(|v| v.parse::<i32>().unwrap())
                .product::<i32>()
        })
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    let pattern = r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)";
    let re = Regex::new(pattern).unwrap();
    let mut enabled = true;
    re.find_iter(input)
        .map(|mat| match mat.as_str() {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            // mul(_, _)
            m if enabled => m[4..m.len() - 1]
                .splitn(2, ',')
                .map(|v| v.parse::<i32>().unwrap())
                .product::<i32>(),
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(3);
        assert_eq!(part_one(&input), 161);
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_two(input), 48);
    }
}
