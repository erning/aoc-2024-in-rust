use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::OnceLock;

type Keypad = HashMap<(i8, i8), u8>;
static NUM_KEYPAD: OnceLock<Keypad> = OnceLock::new();
static DIR_KEYPAD: OnceLock<Keypad> = OnceLock::new();

type KeySeqs = HashMap<(u8, u8), Vec<String>>;
static NUM_SEQUENCES: OnceLock<KeySeqs> = OnceLock::new();
static DIR_SEQUENCES: OnceLock<KeySeqs> = OnceLock::new();

type Cache = HashMap<(u8, u8, usize), usize>;

fn init_keypad(keypad: &[&str]) -> Keypad {
    keypad
        .iter()
        .enumerate()
        .flat_map(|(y, s)| {
            s.bytes()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    b'0'..=b'9' | b'A' | b'^' | b'<' | b'v' | b'>' => {
                        Some(((x as i8, y as i8), c))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

*/
fn init_numeric_keypad() -> Keypad {
    init_keypad(&["789", "456", "123", ".0A"])
}

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/
fn init_directional_keypad() -> Keypad {
    init_keypad(&[".^A", "<v>"])
}

fn init_keyseqs(keypad: &Keypad) -> HashMap<(u8, u8), Vec<String>> {
    let mut sequences = HashMap::new();
    for (p, a) in keypad.iter() {
        for b in keypad.values() {
            let mut seqs: Vec<String> = Vec::new();
            if a == b {
                seqs.push("A".to_string());
            } else {
                let mut queue: VecDeque<((i8, i8), String)> = VecDeque::new();
                queue.push_back((*p, String::new()));
                let mut step = usize::MAX;
                while let Some(((x, y), seq)) = queue.pop_front() {
                    if seq.len() > step {
                        continue;
                    }
                    if let Some(c) = keypad.get(&(x, y)) {
                        if c == b {
                            step = seq.len();
                            let mut s = seq.clone();
                            s.push('A');
                            seqs.push(s);
                        } else {
                            [
                                ((x, y - 1), '^'),
                                ((x + 1, y), '>'),
                                ((x, y + 1), 'v'),
                                ((x - 1, y), '<'),
                            ]
                            .into_iter()
                            .for_each(
                                |(p, c)| {
                                    let mut s = seq.clone();
                                    s.push(c);
                                    queue.push_back((p, s));
                                },
                            );
                        }
                    }
                }
            }
            let min = seqs
                .iter()
                .map(|s| {
                    s.as_bytes()
                        .windows(2)
                        .map(|v| v[0] != v[1])
                        .filter(|v| *v)
                        .count()
                })
                .min()
                .unwrap();
            let seqs = seqs
                .into_iter()
                .filter(|s| {
                    s.as_bytes()
                        .windows(2)
                        .map(|v| v[0] != v[1])
                        .filter(|v| *v)
                        .count()
                        == min
                })
                .collect();
            sequences.insert((*a, *b), seqs);
        }
    }
    sequences
}

fn init_numeric_seqs() -> KeySeqs {
    init_keyseqs(NUM_KEYPAD.get_or_init(init_numeric_keypad))
}

fn init_directional_seqs() -> KeySeqs {
    init_keyseqs(DIR_KEYPAD.get_or_init(init_directional_keypad))
}

fn cartesian_product(lists: &[Vec<String>]) -> Vec<String> {
    if lists.is_empty() {
        return vec![String::new()];
    }
    let mut result = vec![];
    let rest = cartesian_product(&lists[1..]); // Recurse with the rest of the vectors
    for item in &lists[0] {
        for combination in &rest {
            result.push(item.to_string() + combination.as_str());
        }
    }
    result
}

fn expend(s: &str, keyseqs: &KeySeqs) -> Vec<String> {
    let expended: Vec<Vec<String>> = s
        .as_bytes()
        .windows(2)
        .map(|v| keyseqs.get(&(v[0], v[1])).unwrap().to_vec())
        .collect();
    cartesian_product(&expended)
}

fn compute_length(
    a: u8,
    b: u8,
    d: usize,
    keyseqs: &KeySeqs,
    cache: &mut Cache,
) -> usize {
    let k = (a, b, d);
    if let Some(length) = cache.get(&k) {
        return *length;
    }
    let seqs = keyseqs.get(&(a, b)).unwrap();
    if d <= 1 {
        return seqs[0].len();
    }
    let length = seqs
        .iter()
        .map(|s| "A".to_string() + s)
        .map(|s| {
            s.as_bytes()
                .windows(2)
                .map(|v| compute_length(v[0], v[1], d - 1, keyseqs, cache))
                .sum()
        })
        .min()
        .unwrap();
    cache.insert(k, length);
    length
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

pub fn part_one(input: &str) -> usize {
    let numseqs = NUM_SEQUENCES.get_or_init(init_numeric_seqs);
    let dirseqs = DIR_SEQUENCES.get_or_init(init_directional_seqs);

    parse_input(input)
        .iter()
        .map(|s| "A".to_string() + s)
        .map(|s| {
            let mut expended = expend(&s, numseqs);
            for _ in 0..2 {
                let mut t = Vec::new();
                expended.iter().map(|s| "A".to_string() + s).for_each(|s| {
                    t.extend(expend(&s, dirseqs));
                });
                expended = t;
            }
            let v = expended.iter().map(|s| s.len()).min().unwrap();
            let w = s[1..s.len() - 1].parse::<usize>().unwrap();
            v * w
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let numseqs = NUM_SEQUENCES.get_or_init(init_numeric_seqs);
    let dirseqs = DIR_SEQUENCES.get_or_init(init_directional_seqs);

    let mut cache: HashMap<(u8, u8, usize), usize> = HashMap::new();
    parse_input(input)
        .iter()
        .map(|s| {
            let s = "A".to_string() + s;
            let expended = expend(&s, numseqs);
            let v: usize = expended
                .iter()
                .map(|s| "A".to_string() + s)
                .map(|s| {
                    s.as_bytes()
                        .windows(2)
                        .map(|v| {
                            compute_length(
                                v[0], v[1], 25, dirseqs, &mut cache,
                            )
                        })
                        .sum()
                })
                .min()
                .unwrap();
            let w = s[1..s.len() - 1].parse::<usize>().unwrap();
            v * w
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(21);
        assert_eq!(part_one(&input), 126384);
    }
}
