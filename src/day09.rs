fn parse_input(input: &str) -> Vec<Option<usize>> {
    input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let n = c.to_digit(10).unwrap() as usize;
            match i % 2 {
                0 => vec![Some(i / 2); n],
                _ => vec![None; n],
            }
        })
        .collect()
}

fn checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .map(|(i, v)| match v {
            Some(v) => i * v,
            _ => 0,
        })
        .sum()
}

pub fn part_one(input: &str) -> usize {
    let mut disk = parse_input(input);
    let mut a = 0;
    let mut b = disk.len() - 1;
    while a < b {
        while a < b && disk[a].is_some() {
            a += 1;
        }
        while a < b && disk[b].is_none() {
            b -= 1;
        }
        disk.swap(a, b);
    }
    checksum(&disk)
}

pub fn part_two(input: &str) -> usize {
    let mut disk = parse_input(input);
    let mut files: Vec<(usize, usize)> = {
        let mut p = 0;
        input
            .trim()
            .chars()
            .enumerate()
            .filter_map(|(i, c)| {
                let n = c.to_digit(10).unwrap() as usize;
                let f = if i % 2 == 0 { Some((p, n)) } else { None };
                p += n;
                f
            })
            .collect()
    };

    fn find_empty(
        disk: &[Option<usize>],
        p: usize,
    ) -> Option<(usize, usize)> {
        let q = disk.len();
        let mut i = p;
        while i < q && disk[i].is_some() {
            i += 1;
        }
        if i < q {
            let mut j = i;
            while j < q && disk[j].is_none() {
                j += 1;
            }
            Some((i, j - i))
        } else {
            None
        }
    }

    while let Some((fi, fs)) = files.pop() {
        let mut p = 0;
        while let Some((ei, es)) = find_empty(&disk, p) {
            if ei >= fi {
                break;
            }
            if es >= fs {
                for i in 0..fs {
                    disk.swap(ei + i, fi + i);
                }
                break;
            }
            p = ei + es;
        }
    }

    checksum(&disk)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_checksum() {
        let disk: Vec<Option<usize>> =
            "0099811188827773336446555566.............."
                .chars()
                .map(|c| Some(c.to_digit(10).unwrap_or(0) as usize))
                .collect();
        assert_eq!(checksum(&disk), 1928);
    }

    #[test]
    fn example() {
        let input = read_example(9);
        assert_eq!(part_one(&input), 1928);
        assert_eq!(part_two(&input), 2858);
    }
}
