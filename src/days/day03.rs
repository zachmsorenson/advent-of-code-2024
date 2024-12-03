use regex::Regex;

#[derive(Debug)]
pub enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    let re = Regex::new("(mul\\(([0-9]+),([0-9]+)\\))|(do\\(\\))|(don't\\(\\))").unwrap();
    let instructions = re
        .captures_iter(input)
        .map(|capture| {
            if capture.get(1).is_some() {
                let arg1 = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let arg2 = capture.get(3).unwrap().as_str().parse::<i32>().unwrap();

                Instruction::Mul(arg1, arg2)
            } else if capture.get(4).is_some() {
                Instruction::Do
            } else {
                Instruction::Dont
            }
        })
        .collect();

    instructions
}

#[allow(unused_variables)]
pub fn part1(input: &[Instruction]) -> Option<u64> {
    let sum: i32 = input
        .iter()
        .map(|instr| match instr {
            Instruction::Mul(x, y) => x * y,
            _ => 0,
        })
        .sum();

    Some(sum as u64)
}

#[allow(unused_variables)]
pub fn part2(input: &[Instruction]) -> Option<u64> {
    let mut enabled = 1;

    let sum: i32 = input
        .iter()
        .map(|instr| match instr {
            Instruction::Mul(x, y) => enabled * x * y,
            Instruction::Do => {
                enabled = 1;
                0
            }
            Instruction::Dont => {
                enabled = 0;
                0
            }
        })
        .sum();

    Some(sum as u64)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day03/test.txt");
    #[test]
    fn test_day03_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(161));
    }

    #[test]
    fn test_day03_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(48));
    }
}
