use std::collections::HashMap;

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

type Pos = (i32, i32);

#[derive(Debug)]
struct Map {
    robot: Pos,
    tiles: HashMap<Pos, char>,
    w: i32,
    h: i32,
}

impl Map {
    fn find_empty(&self, p: Pos, dir: usize) -> Option<Pos> {
        let (mut x, mut y) = (p.0 + DIRS[dir].0, p.1 + DIRS[dir].1);
        while self.tiles.get(&(x, y)) == Some(&'O') {
            x += DIRS[dir].0;
            y += DIRS[dir].1;
        }
        if self.is_empty((x, y)) {
            Some((x, y))
        } else {
            None
        }
    }

    fn is_empty(&self, p: Pos) -> bool {
        self.robot != p
            && !self.tiles.contains_key(&p)
            && p.0 >= 0
            && p.0 < self.w
            && p.1 >= 0
            && p.1 < self.h
    }

    fn move_robot(&mut self, dir: usize) {
        let p = (self.robot.0 + DIRS[dir].0, self.robot.1 + DIRS[dir].1);
        if self.is_empty(p) {
            self.robot = p;
        } else if let Some(empty) = self.find_empty(self.robot, dir) {
            self.tiles.remove(&p);
            self.tiles.insert(empty, 'O');
            self.robot = p;
        }
    }
}

fn parse_input(input: &str) -> (Map, Vec<usize>) {
    let sections: Vec<&str> = input.trim().splitn(2, "\n\n").collect();
    let map = {
        let mut robot: Pos = (-1, -1);
        let mut tiles: HashMap<Pos, char> = HashMap::new();
        let mut w = 0;
        let mut h = 0;
        sections[0].trim().lines().enumerate().for_each(|(y, s)| {
            let y = y as i32;
            h = h.max(y);
            s.chars().enumerate().for_each(|(x, ch)| {
                let x = x as i32;
                w = w.max(x);
                match ch {
                    '@' => {
                        robot = (x, y);
                    }
                    'O' | '#' => {
                        tiles.insert((x, y), ch);
                    }
                    _ => {}
                };
            });
        });
        Map { robot, tiles, w, h }
    };
    let movements = sections[1]
        .trim()
        .lines()
        .flat_map(|s| {
            s.trim().chars().map(|ch| match ch {
                '^' => 0,
                '>' => 1,
                'v' => 2,
                '<' => 3,
                _ => panic!(),
            })
        })
        .collect();
    (map, movements)
}

pub fn part_one(input: &str) -> i32 {
    let (mut map, movements) = parse_input(input);
    movements.iter().for_each(|dir| map.move_robot(*dir));
    map.tiles
        .into_iter()
        .filter(|(_, ch)| ch == &'O')
        .map(|((x, y), _)| x + 100 * y)
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_smaller() {
        const INPUT: &str = concat!(
            "########\n",
            "#..O.O.#\n",
            "##@.O..#\n",
            "#...O..#\n",
            "#.#.O..#\n",
            "#...O..#\n",
            "#......#\n",
            "########\n",
            "\n",
            "<^^>>>vv<v>>v<<"
        );
        // for this smaller example,
        // the sum of all boxes' GPS coordinates is 2028
        let (mut map, movements) = parse_input(INPUT);
        movements.iter().for_each(|dir| map.move_robot(*dir));
        let sum: i32 = map
            .tiles
            .into_iter()
            .filter(|(_, ch)| ch == &'O')
            .map(|((x, y), _)| x + 100 * y)
            .sum();
        assert_eq!(sum, 2028);
    }

    fn example_smaller_2() {
        const INPUT: &str = concat!(
            "#######\n",
            "#...#.#\n",
            "#.....#\n",
            "#..OO@#\n",
            "#..O..#\n",
            "#.....#\n",
            "#######\n",
            "\n",
            "<vv<<^^<<^^"
        );
    }

    #[test]
    fn example() {
        let input = read_example(15);
        assert_eq!(part_one(&input), 10092);
        assert_eq!(part_two(&input), 9021);
    }
}
