use std::collections::HashSet;

type Pos = (i32, i32);
const DIRECTIONS: [Pos; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse_input(input: &str) -> (HashSet<Pos>, i32, i32, Pos) {
    let mut start = (0, 0);
    let mut height = 0;
    let mut width = 0;
    let obstructions = input
        .trim()
        .lines()
        .enumerate()
        .filter(|(y, _)| {
            height = height.max(*y as i32);
            true
        })
        .flat_map(|(y, s)| {
            s.bytes()
                .enumerate()
                .filter(|(x, _)| {
                    width = width.max(*x as i32);
                    true
                })
                .filter(|&(x, c)| match c {
                    b'#' => true,
                    b'^' => {
                        start = (x as i32, y as i32);
                        false
                    }
                    _ => false,
                })
                .map(|(x, _)| (x as i32, y as i32))
                .collect::<Vec<_>>()
        })
        .collect();
    (obstructions, width + 1, height + 1, start)
}

fn search(
    obstructions: &HashSet<Pos>,
    width: i32,
    height: i32,
    pretrace: &[(Pos, usize)],
) -> (Vec<(Pos, usize)>, bool) {
    let (mut p, mut d) = pretrace.last().unwrap();
    let mut trace: Vec<_> = pretrace[..pretrace.len() - 1].to_vec();
    let mut visited: HashSet<_> = trace.iter().cloned().collect();
    let mut is_loop = false;
    while p.0 >= 0 && p.0 < width && p.1 >= 0 && p.1 < height {
        if visited.contains(&(p, d)) {
            is_loop = true;
            break;
        }
        trace.push((p, d));
        visited.insert((p, d));
        let dir = DIRECTIONS[d];
        let q = (p.0 + dir.0, p.1 + dir.1);
        if obstructions.contains(&q) {
            d = (d + 1) % 4;
            continue;
        }
        p = q;
    }
    (trace, is_loop)
}

pub fn part_one(input: &str) -> usize {
    let (obstructions, width, height, start) = parse_input(input);
    let (trace, _) = search(&obstructions, width, height, &[(start, 0)]);
    trace.iter().map(|&(p, _)| p).collect::<HashSet<_>>().len()
}

pub fn part_two(input: &str) -> usize {
    let (mut obstructions, width, height, start) = parse_input(input);
    let mut visited = HashSet::new();
    let (trace, _) = search(&obstructions, width, height, &[(start, 0)]);
    trace
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, (p, _))| (i, *p))
        .filter(|&(i, p)| {
            !visited.contains(&p) && {
                visited.insert(p);
                obstructions.insert(p);
                let (_, is_loop) =
                    search(&obstructions, width, height, &trace[..i]);
                obstructions.remove(&p);
                is_loop
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(6);
        assert_eq!(part_one(&input), 41);
        assert_eq!(part_two(&input), 6);
    }
}
