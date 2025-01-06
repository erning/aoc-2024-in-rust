use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .trim()
        .lines()
        .map(|s| s.split_ascii_whitespace().collect::<Vec<_>>())
        .filter_map(|v| match v.len() {
            2 => Some(vec![&v[0][..3], v[1]]),
            5 => Some(vec![v[4], v[0], v[1], v[2]]),
            _ => None,
        })
        .collect()
}

fn get_value<'a>(
    name: &'a str,
    origin: &HashMap<&str, Vec<&'a str>>,
    result: &mut HashMap<&'a str, u8>,
) -> u8 {
    if let Some(v) = result.get(name) {
        return *v;
    }

    let value = if let Some(v) = origin.get(name) {
        match v.len() {
            1 => v[0].parse::<u8>().unwrap(),
            _ => {
                let n1 = get_value(v[0], origin, result);
                let n2 = get_value(v[2], origin, result);
                let op = v[1];
                match op {
                    "AND" => n1 & n2,
                    "OR" => n1 | n2,
                    "XOR" => n1 ^ n2,
                    _ => panic!(),
                }
            }
        }
    } else {
        panic!()
    };

    result.insert(name, value);
    value
}

pub fn part_one(input: &str) -> u64 {
    let registers: HashMap<&str, Vec<&str>> = parse_input(input)
        .iter()
        .map(|v| (v[0], v[1..].to_vec()))
        .collect();
    let mut result: HashMap<&str, u8> = HashMap::new();

    let mut zregs: Vec<(&str, u8)> = registers
        .keys()
        .filter(|v| v.starts_with('z'))
        .map(|name| (*name, get_value(name, &registers, &mut result)))
        .collect();
    zregs.sort_unstable();
    zregs
        .iter()
        .rev()
        .map(|(_, v)| *v as u64)
        .reduce(|a, c| a << 1 | c)
        .unwrap()
}

pub fn part_two(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(24);
        assert_eq!(part_one(&input), 2024);
        assert_eq!(part_two(&input), 0);
    }
}
