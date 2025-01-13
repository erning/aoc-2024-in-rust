fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .trim()
        .split("\n\n")
        .map(|section| section.lines().map(|s| s.chars().collect()).collect())
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let grids = parse_input(input);
    let pins: Vec<(bool, Vec<usize>)> = grids
        .iter()
        .map(|grid| {
            let n = grid.len();
            let is_lock = grid[0][0] == '#';
            let pin = grid
                .iter()
                .skip(if is_lock { 1 } else { 0 })
                .take(n - 1)
                .map(|row| {
                    row.iter()
                        .map(|ch| if ch == &'#' { 1 } else { 0 })
                        .collect::<Vec<_>>()
                })
                .reduce(|acc, e| {
                    std::iter::zip(acc, e).map(|(a, b)| a + b).collect()
                })
                .unwrap();
            (is_lock, pin)
        })
        .collect();

    let locks: Vec<&[usize]> = pins
        .iter()
        .filter(|(is_lock, _)| *is_lock)
        .map(|(_, pin)| &pin[..])
        .collect();
    let keys: Vec<&[usize]> = pins
        .iter()
        .filter(|(is_lock, _)| !*is_lock)
        .map(|(_, pin)| &pin[..])
        .collect();

    locks
        .iter()
        .flat_map(|lock| {
            keys.iter().map(|key| {
                std::iter::zip(*lock, *key).all(|(a, b)| a + b <= 5)
            })
        })
        .filter(|it| *it)
        .count()
}

pub fn part_two(_: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(25);
        assert_eq!(part_one(&input), 3);
    }
}
