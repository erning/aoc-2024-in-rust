use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
enum Gate {
    OR,
    AND,
    XOR,
}

impl Gate {
    fn exec(&self, a: bool, b: bool) -> bool {
        match self {
            Gate::OR => a | b,
            Gate::AND => a & b,
            Gate::XOR => a ^ b,
        }
    }
}

type Knowns = HashMap<String, bool>;
type Unknowns = HashMap<String, (String, String, Gate)>;

fn parse_input(input: &str) -> (Knowns, Unknowns) {
    let mut lines = input.trim().lines();
    let mut knowns = Knowns::new();
    let mut unknowns = Unknowns::new();

    while let Some(s) = lines.next() {
        if s.is_empty() {
            break;
        }
        let v = s.split_ascii_whitespace().collect::<Vec<_>>();
        let name = v[0][0..3].to_string();
        let value = v[1] == "1";
        knowns.insert(name, value);
    }
    while let Some(s) = lines.next() {
        let v = s.split_ascii_whitespace().collect::<Vec<_>>();
        let name = v[4].to_string();
        let n1 = v[0].to_string();
        let n2 = v[2].to_string();
        let gate = match v[1] {
            "OR" => Gate::OR,
            "AND" => Gate::AND,
            "XOR" => Gate::XOR,
            _ => panic!(),
        };
        unknowns.insert(name, (n1, n2, gate));
    }

    (knowns, unknowns)
}

pub fn part_one(input: &str) -> u64 {
    let (mut knowns, unknowns) = parse_input(input);

    fn get_value(
        name: &str,
        knowns: &mut Knowns,
        unknowns: &Unknowns,
    ) -> bool {
        if let Some(v) = knowns.get(name) {
            return *v;
        }
        if let Some((n1, n2, gate)) = unknowns.get(name) {
            let v1 = get_value(n1, knowns, unknowns);
            let v2 = get_value(n2, knowns, unknowns);
            let v = gate.exec(v1, v2);
            knowns.insert(name.to_string(), v);
            return v;
        }
        panic!()
    }

    let zs: Vec<u64> = {
        let mut vs = unknowns
            .keys()
            .filter(|it| it.starts_with('z'))
            .map(|it| (it, get_value(it, &mut knowns, &unknowns)))
            .collect::<Vec<_>>();
        vs.sort_unstable();
        vs.into_iter()
            .rev()
            .map(|(_, v)| if v { 1 } else { 0 })
            .collect()
    };
    zs.into_iter().reduce(|acc, e| acc << 1 | e).unwrap()
}

pub fn part_two(input: &str) -> i32 {
    let (mut knowns, unknowns) = parse_input(input);

    fn get_depends(
        name: &str,
        knowns: &Knowns,
        unknowns: &Unknowns,
    ) -> Vec<String> {
        if knowns.contains_key(name) {
            return vec![];
        }
        if let Some((n1, n2, gate)) = unknowns.get(name) {
            let mut v =
                vec![n1.to_string(), n2.to_string(), format!("{:?}", gate)];
            v.sort_unstable();
            let mut v = vec![v.join(" ")];
            let v1 = get_depends(n1, knowns, unknowns);
            let v2 = get_depends(n2, knowns, unknowns);
            v.extend(v1);
            v.extend(v2);
            return v;
        }
        panic!()
    }

    let n = knowns.len() / 2 + 1;

    let depends: Vec<(String, Vec<String>)> = unknowns
        .keys()
        .map(|it| (it.to_string(), get_depends(it, &knowns, &unknowns)))
        .collect();

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

        let input = concat!(
            "x00: 0\n",
            "x01: 1\n",
            "x02: 0\n",
            "x03: 1\n",
            "x04: 0\n",
            "x05: 1\n",
            "y00: 0\n",
            "y01: 0\n",
            "y02: 1\n",
            "y03: 1\n",
            "y04: 0\n",
            "y05: 1\n",
            "\n",
            "x00 AND y00 -> z05\n",
            "x01 AND y01 -> z02\n",
            "x02 AND y02 -> z01\n",
            "x03 AND y03 -> z03\n",
            "x04 AND y04 -> z04\n",
            "x05 AND y05 -> z00\n",
        );
        assert_eq!(part_two(&input), 0);
    }
}
