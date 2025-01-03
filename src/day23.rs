use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .lines()
        .map(|s| s.trim().splitn(2, '-').collect::<Vec<_>>())
        .map(|v| (v[0], v[1]))
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let input = parse_input(input);
    let mut network: HashMap<&str, HashSet<&str>> = HashMap::new();
    for &(a, b) in input.iter() {
        network
            .entry(a)
            .and_modify(|v| {
                v.insert(b);
            })
            .or_insert_with(|| HashSet::from([b]));
        network
            .entry(b)
            .and_modify(|v| {
                v.insert(a);
            })
            .or_insert_with(|| HashSet::from([a]));
    }
    let mut answers: HashSet<Vec<&str>> = HashSet::new();
    fn st(s: &str) -> bool {
        s.starts_with('t')
    }
    for (a, v) in network.iter() {
        for (i, b) in v.iter().enumerate().take(v.len() - 1) {
            for c in v.iter().skip(i + 1) {
                if (st(a) || st(b) || st(c)) && network[b].contains(c) {
                    let mut v = vec![*a, *b, *c];
                    v.sort_unstable();
                    answers.insert(v);
                }
            }
        }
    }
    answers.len()
}

pub fn part_two(input: &str) -> String {
    let input = parse_input(input);

    let connected: HashSet<(&str, &str)> = input
        .iter()
        .flat_map(|&(a, b)| vec![(a, b), (b, a)])
        .collect();
    let nodes: HashSet<&str> =
        input.iter().flat_map(|&(a, b)| vec![a, b]).collect();
    let mut clusters: Vec<Vec<&str>> =
        nodes.iter().map(|&node| vec![node]).collect();

    for &a in nodes.iter() {
        for cluster in clusters.iter_mut() {
            if cluster.iter().all(|&b| connected.contains(&(a, b))) {
                cluster.push(a);
            }
        }
    }
    for cluster in clusters.iter_mut() {
        cluster.sort_unstable();
    }
    let clusters = clusters
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let size = clusters.iter().map(|v| v.len()).max().unwrap();

    clusters.iter().find(|v| v.len() == size).unwrap().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(23);
        assert_eq!(part_one(&input), 7);
        assert_eq!(part_two(&input), "co,de,ka,ta");
    }
}
