#[derive(Debug)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    p: usize,
}

impl Computer {
    fn combo(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        }
    }

    fn exec_once(&mut self, program: &[u8], output: &mut Vec<u8>) -> bool {
        let opcode = program[self.p];
        let operand = program[self.p + 1];
        self.p += 2;
        match opcode {
            /* adv */ 0 => self.a >>= self.combo(operand),
            /* bxl */ 1 => self.b ^= operand as u64,
            /* bst */ 2 => self.b = self.combo(operand) % 8,
            /* jnz */ 3 if self.a != 0 => self.p = operand as usize,
            /* bxc */ 4 => self.b ^= self.c,
            /* out */ 5 => output.push(self.combo(operand) as u8 % 8),
            /* bdv */ 6 => self.b = self.a >> self.combo(operand),
            /* cdv */ 7 => self.c = self.a >> self.combo(operand),
            _ => {}
        }
        self.p < program.len()
    }

    fn execute(&mut self, program: &[u8]) -> Vec<u8> {
        self.p = 0;
        let mut output = Vec::new();
        while self.exec_once(program, &mut output) {}
        output
    }
}

fn parse_input(input: &str) -> (Computer, Vec<u8>) {
    const REGI_PREFIX_LEN: usize = "Register #:".len();
    const PROG_PREFIX_LEN: usize = "Program:".len();
    let lines: Vec<&str> = input.trim().lines().collect();
    let a = lines[0][REGI_PREFIX_LEN..].trim().parse().unwrap();
    let b = lines[1][REGI_PREFIX_LEN..].trim().parse().unwrap();
    let c = lines[2][REGI_PREFIX_LEN..].trim().parse().unwrap();
    let program = lines[4][PROG_PREFIX_LEN..]
        .trim()
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    (Computer { a, b, c, p: 0 }, program)
}

pub fn part_one(input: &str) -> String {
    let (mut comp, program) = parse_input(input);
    comp.execute(&program)
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn part_two(input: &str) -> u64 {
    let (mut comp, program) = parse_input(input);
    let mut a = 1;
    loop {
        comp.a = a;
        let output = comp.execute(&program);
        if output == program[program.len() - output.len()..] {
            if output.len() == program.len() {
                break;
            }
            a <<= 3;
        } else {
            a += 1;
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    const INPUT_P2: &str = concat!(
        "Register A: 2024\n",
        "Register B: 0\n",
        "Register C: 0\n",
        "\n",
        "Program: 0,3,5,4,3,0"
    );

    #[test]
    fn example() {
        let input = read_example(17);
        assert_eq!(part_one(&input), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(part_two(INPUT_P2), 117440);
    }
}
