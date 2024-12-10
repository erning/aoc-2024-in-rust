use core::str;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.trim().lines().map(|s| s.bytes().collect()).collect()
}

fn build_lps(pattern: &[u8]) -> Vec<usize> {
    let mut lps = vec![0; pattern.len()];
    let mut length = 0; // length of the previous longest prefix suffix
    let mut i = 1;

    while i < pattern.len() {
        if pattern[i] == pattern[length] {
            length += 1;
            lps[i] = length;
            i += 1;
        } else if length != 0 {
            // Go back to the previous longest prefix suffix
            length = lps[length - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }

    lps
}

fn kmp_search_all(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    let lps = build_lps(pattern);
    let mut matches = Vec::new(); // Store all match indices

    let mut i = 0; // Index for `text`
    let mut j = 0; // Index for `pattern`

    while i < text.len() {
        if text[i] == pattern[j] {
            i += 1;
            j += 1;
        }

        if j == pattern.len() {
            // Found a match, record the starting index
            matches.push(i - j);
            j = lps[j - 1]; // Continue searching for more matches
        } else if i < text.len() && text[i] != pattern[j] {
            // Mismatch after `j` matches
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    matches
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let h = grid.len();
    let w = grid[0].len();
    let mut occurs = 0;

    fn search(s: &[u8]) -> usize {
        let a: &[u8] = "XMAS".as_bytes();
        let b: &[u8] = "SAMX".as_bytes();
        kmp_search_all(s, a).len() + kmp_search_all(s, b).len()
    }

    // horizontal
    #[allow(clippy::needless_range_loop)]
    for y in 0..h {
        let s: Vec<u8> = (0..w).map(|x| grid[y][x]).collect();
        occurs += search(&s);
    }
    // vertical
    for x in 0..w {
        let s: Vec<u8> = (0..h).map(|y| grid[y][x]).collect();
        occurs += search(&s);
    }
    // slash
    for i in 0..w {
        let mut x = i;
        let mut y = 0;
        let mut s: Vec<u8> = Vec::new();
        while y < h {
            s.push(grid[y][x]);
            if x == 0 {
                break;
            }
            x -= 1;
            y += 1;
        }
        occurs += search(&s);
    }
    for i in 1..h {
        let mut x = w - 1;
        let mut y = i;
        let mut s: Vec<u8> = Vec::new();
        while y < h {
            s.push(grid[y][x]);
            if x == 0 {
                break;
            }
            x -= 1;
            y += 1;
        }
        occurs += search(&s);
    }
    // backslash
    for i in 0..w {
        let mut x = i;
        let mut y = 0;
        let mut s: Vec<u8> = Vec::new();
        while y < h && x < w {
            s.push(grid[y][x]);
            x += 1;
            y += 1;
        }
        occurs += search(&s);
    }
    for i in 1..h {
        let mut x = 0;
        let mut y = i;
        let mut s: Vec<u8> = Vec::new();
        while y < h && x < w {
            s.push(grid[y][x]);
            x += 1;
            y += 1;
        }
        occurs += search(&s);
    }

    //
    occurs
}

pub fn part_two(input: &str) -> usize {
    let grid = parse_input(input);
    let h = grid.len();
    let w = grid[0].len();
    // find all possible 'A' positions
    // and validate X-MAS
    (1..h - 1)
        .flat_map(|y| {
            (1..w - 1)
                .map(|x| (x, y))
                .filter(|&(x, y)| grid[y][x] == b'A')
                .collect::<Vec<(usize, usize)>>()
        })
        .filter(|(x, y)| {
            matches!(
                (
                    (grid[y - 1][x - 1], grid[y + 1][x + 1]),
                    (grid[y - 1][x + 1], grid[y + 1][x - 1]),
                ),
                ((b'M', b'S'), (b'M', b'S'))
                    | ((b'M', b'S'), (b'S', b'M'))
                    | ((b'S', b'M'), (b'M', b'S'))
                    | ((b'S', b'M'), (b'S', b'M'))
            )
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(4);
        assert_eq!(part_one(&input), 18);
        assert_eq!(part_two(&input), 9);
    }
}
