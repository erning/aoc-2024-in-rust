use std::collections::HashMap;
use std::collections::HashSet;

type Pos = (i32, i32);
type Antennas = HashMap<char, HashSet<Pos>>;

fn parse_input(input: &str) -> (Antennas, i32, i32) {
    let h = input.trim().lines().count() as i32;
    let w = input.trim().lines().next().unwrap().len() as i32;
    let mut antennas: Antennas = Antennas::new();
    input.trim().lines().enumerate().for_each(|(y, s)| {
        s.chars().enumerate().for_each(|(x, c)| match c {
            '.' => {}
            c => {
                let p = (x as i32, y as i32);
                antennas
                    .entry(c)
                    .and_modify(|v| {
                        v.insert(p);
                    })
                    .or_insert([p].into_iter().collect());
            }
        });
    });
    (antennas, w, h)
}

pub fn part_one(input: &str) -> usize {
    let (antennas, w, h) = parse_input(input);
    let antinodes: HashSet<Pos> = antennas
        .iter()
        .flat_map(|(_, nodes)| {
            nodes.iter().flat_map(|a| {
                nodes
                    .iter()
                    .filter_map(|b| match a == b {
                        true => None,
                        false => Some((b.0 + b.0 - a.0, b.1 + b.1 - a.1)),
                    })
                    .filter(|&(x, y)| x >= 0 && x < w && y >= 0 && y < h)
                    .collect::<Vec<Pos>>()
            })
        })
        .collect();
    antinodes.len()
}

pub fn part_two(input: &str) -> usize {
    let (antennas, w, h) = parse_input(input);
    let mut antinodes: HashSet<Pos> =
        antennas.values().flatten().cloned().collect();
    for (_, nodes) in antennas.iter() {
        for a in nodes.iter() {
            for b in nodes.iter() {
                if a == b {
                    continue;
                }
                let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                let (mut x, mut y) = (b.0 + dx, b.1 + dy);
                while x >= 0 && x < w && y >= 0 && y < h {
                    antinodes.insert((x, y));
                    (x, y) = (x + dx, y + dy)
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(8);
        assert_eq!(part_one(&input), 14);
        assert_eq!(part_two(&input), 34);
    }
}
