use std::collections::HashMap;
use std::collections::HashSet;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Clone, Copy)]
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
    let mut knowns = Knowns::new();
    let mut unknowns = Unknowns::new();
    let sections: Vec<&str> = input.trim().splitn(2, "\n\n").collect();

    for s in sections[0].lines() {
        let v = s.split_ascii_whitespace().collect::<Vec<_>>();
        let name = v[0][0..3].to_string();
        let value = v[1] == "1";
        knowns.insert(name, value);
    }
    for s in sections[1].lines() {
        let v = s.split_ascii_whitespace().collect::<Vec<_>>();
        let name = v[4].to_string();
        let lhs = v[0].to_string();
        let rhs = v[2].to_string();
        let gate = match v[1] {
            "OR" => Gate::OR,
            "AND" => Gate::AND,
            "XOR" => Gate::XOR,
            _ => panic!(),
        };
        unknowns.insert(name, (lhs, rhs, gate));
    }

    (knowns, unknowns)
}

pub fn part_one(input: &str) -> u64 {
    let (mut knowns, unknowns) = parse_input(input);

    fn get_value(
        name: &String,
        knowns: &mut Knowns,
        unknowns: &Unknowns,
    ) -> bool {
        if let Some(v) = knowns.get(name) {
            return *v;
        }
        if let Some((lhs, rhs, gate)) = unknowns.get(name) {
            let v = gate.exec(
                get_value(lhs, knowns, unknowns),
                get_value(rhs, knowns, unknowns),
            );
            knowns.insert(name.clone(), v);
            return v;
        }
        panic!()
    }

    let zs: Vec<u8> = {
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
    zs.into_iter()
        .map(|it| it as u64)
        .reduce(|acc, e| acc << 1 | e)
        .unwrap()
}

pub fn part_two(input: &str) -> String {
    let (knowns, mut unknowns) = parse_input(input);
    unknowns.iter_mut().for_each(|(_, (lhs, rhs, _))| {
        if lhs > rhs {
            std::mem::swap(lhs, rhs);
        }
    });

    // sum(i) = x(i) XOR y(i) XOR carry(i-1)
    //        = x(i) XOR y(i) XOR {[x(i-1) AND y(i-1)] OR [..] OR [..] ...}
    // carry(i) = [x(i) AND y(i)]
    //              OR [x(i) AND carry(i-1)]
    //              OR [y(i) AND carry(i-1)]

    let n = knowns.len() / 2;
    let sums: Vec<String> = (0..n)
        .map(|i| {
            unknowns
                .iter()
                .find(|(_, (lhs, rhs, gate))| {
                    gate == &Gate::XOR
                        && lhs == &format!("x{:02}", i)
                        && rhs == &format!("y{:02}", i)
                })
                .map(|(name, _)| name.clone())
                .unwrap_or_default()
        })
        .collect();

    let mut pairs: Vec<[String; 2]> = Vec::new();

    #[allow(clippy::needless_range_loop)]
    for i in 0..n {
        let name = format!("z{:02}", i);
        let (lhs, rhs, gate) = unknowns.get(&name).unwrap();
        if gate != &Gate::XOR {
            let wire = unknowns
                .iter()
                .find(|(_, (lhs, rhs, gate))| {
                    gate == &Gate::XOR && (lhs == &sums[i] || rhs == &sums[i])
                })
                .map(|(name, _)| name.clone())
                .unwrap();
            pairs.push([name, wire]);
            continue;
        }

        fn get_carry_depends(
            name: &str,
            unknowns: &Unknowns,
        ) -> HashSet<usize> {
            let mut v = HashSet::new();
            if let Some((lhs, rhs, gate)) = unknowns.get(name) {
                if gate == &Gate::AND
                    && (lhs.starts_with('x') && rhs.starts_with('y'))
                {
                    assert!(lhs[1..] == rhs[1..]);
                    v.insert(lhs[1..].parse().unwrap());
                }
                v.extend(get_carry_depends(lhs, unknowns));
                v.extend(get_carry_depends(rhs, unknowns));
            }
            v
        }

        if lhs != &sums[i] && rhs != &sums[i] {
            // lhs or rhs should be replaced by sums[i], but which one?
            // one of them is sum(i) and the other one is carry(i-1)
            let is_wrong = |name| {
                let depends = get_carry_depends(name, &unknowns);
                depends.len() != i || depends.iter().any(|it| it > &i)
            };
            if is_wrong(lhs) {
                pairs.push([lhs.clone(), sums[i].clone()]);
            } else if is_wrong(rhs) {
                pairs.push([rhs.clone(), sums[i].clone()]);
            };
        }
    }

    // let mut v = ["z19", "mdd", "z37", "wts", "z11", "wpd", "jqf", "skh"];
    let mut v: Vec<String> = pairs.into_iter().flatten().collect();
    v.sort_unstable();
    v.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(24);
        assert_eq!(part_one(&input), 2024);
    }
}
