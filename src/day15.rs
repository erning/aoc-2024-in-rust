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

#[derive(Debug)]
struct LargeMap {
    robot: Pos,
    tiles: HashMap<Pos, char>,
    w: i32,
    h: i32,
}

impl LargeMap {
    fn affected_boxes(
        &self,
        p: Pos,
        dir: usize,
        boxes: &mut HashMap<Pos, char>,
    ) -> bool {
        let next = (p.0 + DIRS[dir].0, p.1 + DIRS[dir].1);
        if next.0 < 0 || next.0 >= self.w || next.1 < 0 || next.1 >= self.h {
            return false;
        }
        // println!("next={:?}", (next, self.tiles.get(&next)));
        match self.tiles.get(&next) {
            Some('[') => {
                boxes.insert(next, '[');
                if dir % 2 == 0 {
                    let next1 = (next.0 + 1, next.1);
                    boxes.insert(next1, ']');
                    self.affected_boxes(next, dir, boxes)
                        && self.affected_boxes(next1, dir, boxes)
                } else {
                    self.affected_boxes(next, dir, boxes)
                }
            }
            Some(']') => {
                boxes.insert(next, ']');
                if dir % 2 == 0 {
                    let next1 = (next.0 - 1, next.1);
                    boxes.insert(next1, '[');
                    self.affected_boxes(next, dir, boxes)
                        && self.affected_boxes(next1, dir, boxes)
                } else {
                    self.affected_boxes(next, dir, boxes)
                }
            }
            Some('#') => false,
            _ => true,
        }
    }

    fn move_robot(&mut self, dir: usize) {
        let mut boxes = HashMap::new();
        if self.affected_boxes(self.robot, dir, &mut boxes) {
            self.robot =
                (self.robot.0 + DIRS[dir].0, self.robot.1 + DIRS[dir].1);
            boxes.keys().for_each(|p| {
                self.tiles.remove(p);
            });
            boxes
                .iter()
                .map(|(p, ch)| ((p.0 + DIRS[dir].0, p.1 + DIRS[dir].1), ch))
                .for_each(|(p, ch)| {
                    self.tiles.insert(p, *ch);
                })
        }
    }

    // fn print(&self) {
    //     for y in 0..self.h {
    //         for x in 0..self.w {
    //             let ch = match self.tiles.get(&(x, y)) {
    //                 Some(ch) => *ch,
    //                 None if (x, y) == self.robot => '@',
    //                 None => '.',
    //             };
    //             print!("{}", ch);
    //         }
    //         println!();
    //     }
    // }
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
            h = h.max(y + 1);
            s.chars().enumerate().for_each(|(x, ch)| {
                let x = x as i32;
                w = w.max(x + 1);
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

fn enlarge(map: Map) -> LargeMap {
    LargeMap {
        robot: (map.robot.0 * 2, map.robot.1),
        tiles: map
            .tiles
            .iter()
            .flat_map(|(&(x, y), ch)| match ch {
                'O' => vec![((x * 2, y), '['), ((x * 2 + 1, y), ']')],
                '#' => vec![((x * 2, y), '#'), ((x * 2 + 1, y), '#')],
                _ => vec![],
            })
            .collect(),
        w: map.w * 2,
        h: map.h,
    }
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
    let (map, movements) = parse_input(input);
    let mut map = enlarge(map);
    movements.iter().for_each(|dir| map.move_robot(*dir));
    map.tiles
        .into_iter()
        .filter(|(_, ch)| ch == &'[')
        .map(|((x, y), _)| x + 100 * y)
        .sum()
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

    #[test]
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
        let (map, movements) = parse_input(INPUT);
        let mut map = enlarge(map);
        movements.iter().for_each(|dir| map.move_robot(*dir));
        assert_eq!(map.tiles.get(&(5, 1)), Some(&'['));
    }

    #[test]
    fn example() {
        let input = read_example(15);
        assert_eq!(part_one(&input), 10092);
        assert_eq!(part_two(&input), 9021);
    }
}
