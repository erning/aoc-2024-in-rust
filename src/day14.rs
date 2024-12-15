type Position = (i32, i32);
type Velocity = (i32, i32);
type Robot = (Position, Velocity);

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .trim()
        .lines()
        .map(|s| {
            s.splitn(2, ' ')
                .map(|s| {
                    s[2..]
                        .splitn(2, ',')
                        .map(|s| s.parse::<_>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|v| (v[0], v[1]))
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .collect::<Vec<_>>()
}

fn move_robot(robot: &Robot, w: i32, h: i32, s: i32) -> Position {
    let ((mut x, mut y), (dx, dy)) = robot;
    x = (x + dx * s) % w;
    y = (y + dy * s) % h;
    if x < 0 {
        x += w
    }
    if y < 0 {
        y += h
    }
    (x, y)
}

fn move_robots(robots: &[Robot], w: i32, h: i32, s: i32) -> Vec<Position> {
    robots
        .iter()
        .map(|robot| move_robot(robot, w, h, s))
        .collect()
}

fn quadrants(positions: &[Position], w: i32, h: i32) -> [usize; 4] {
    let wm = w / 2;
    let wh = h / 2;
    positions
        .iter()
        .map(|&(x, y)| {
            if x < wm && y < wh {
                [1, 0, 0, 0]
            } else if x > wm && y < wh {
                [0, 1, 0, 0]
            } else if x > wm && y > wh {
                [0, 0, 1, 0]
            } else if x < wm && y > wh {
                [0, 0, 0, 1]
            } else {
                [0, 0, 0, 0]
            }
        })
        .fold([0, 0, 0, 0], |a, b| {
            [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
        })
}

fn safety_factor(input: &str, w: i32, h: i32, s: i32) -> usize {
    let robots = parse_input(input);
    let positions = move_robots(&robots, w, h, s);
    quadrants(&positions, w, h).iter().product()
}

pub fn part_one(input: &str) -> usize {
    safety_factor(input, 101, 103, 100)
}

pub fn part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(14);
        assert_eq!(safety_factor(&input, 11, 7, 100), 12);
    }
}
