fn parse_input(input: &str) -> Vec<Vec<Vec<i64>>> {
    input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .map(|s| {
                    s.splitn(2, ':')
                        .last()
                        .unwrap()
                        .trim()
                        .splitn(2, ',')
                        .map(|s| s.trim())
                        .map(|s| s[2..].parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

/// claude.ai
///
/// Solves a system of two linear equations with two unknowns
///
/// The system of equations is represented as:
/// ax + by = c
/// dx + ey = f
///
/// # Arguments
/// * `a` - Coefficient of x in the first equation
/// * `b` - Coefficient of y in the first equation
/// * `c` - Constant term in the first equation
/// * `d` - Coefficient of x in the second equation
/// * `e` - Coefficient of y in the second equation
/// * `f` - Constant term in the second equation
///
/// # Returns
/// An Option containing a tuple of (x, y) if an integer solution exists,
/// or None if no integer solution exists
fn solve_two_variable_linear_equations(
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
    f: i64,
) -> Option<(i64, i64)> {
    // Use Cramer's rule to solve the system of linear equations

    // Calculate the determinant of the coefficient matrix
    let det = a * e - b * d;

    // If determinant is zero, the system has either no solution or infinite solutions
    if det == 0 {
        // Check if the equations are consistent
        // Solve first equation for x: x = (c - by) / a
        // Substitute into second equation
        // c/a - by/a = (f - ey) / d
        // cd/a - bdy/a = f - ey
        // Rearrange to check consistency
        return if a * f == d * c && b * f == e * c {
            // Infinite solutions (dependent equations)
            None
        } else {
            // No solution (inconsistent equations)
            None
        };
    }

    // Calculate x using Cramer's rule
    let x_det = c * e - b * f;
    let y_det = a * f - c * d;

    // Check if x and y are integers
    if x_det % det == 0 && y_det % det == 0 {
        Some((x_det / det, y_det / det))
    } else {
        // No integer solution
        None
    }
}

pub fn part_one(input: &str) -> i64 {
    let configs = parse_input(input);
    configs
        .iter()
        .filter_map(|config| {
            let (a, b, c) = (config[0][0], config[1][0], config[2][0]);
            let (d, e, f) = (config[0][1], config[1][1], config[2][1]);
            solve_two_variable_linear_equations(a, b, c, d, e, f)
        })
        .map(|v| v.0 * 3 + v.1)
        .sum()
}

pub fn part_two(input: &str) -> i64 {
    let configs = parse_input(input);
    configs
        .iter()
        .filter_map(|config| {
            let (a, b, mut c) = (config[0][0], config[1][0], config[2][0]);
            let (d, e, mut f) = (config[0][1], config[1][1], config[2][1]);
            c += 10000000000000;
            f += 10000000000000;
            solve_two_variable_linear_equations(a, b, c, d, e, f)
        })
        .map(|v| v.0 * 3 + v.1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(13);
        assert_eq!(part_one(&input), 480);
    }
}
