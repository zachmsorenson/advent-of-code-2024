use rayon::prelude::*;

#[derive(Debug)]
pub enum Operator {
    Add(u64),
    Multiply(u64),
    Concat(u64),
}

impl Operator {
    fn op(&self, x: u64) -> u64 {
        match self {
            Operator::Add(y) => x + y,
            Operator::Multiply(y) => x * y,
            Operator::Concat(y) => {
                let mut digits = 1;
                while digits < *y {
                    digits *= 10;
                }

                x * digits + y
            }
        }
    }
}

#[derive(Debug)]
pub struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

pub struct Input {
    equations: Vec<Equation>,
}

pub fn parse_input(input: &str) -> Input {
    let mut equations = Vec::new();
    for line in input.lines() {
        let mut iter = line.split(':');
        let result = iter.next().unwrap().parse::<u64>().unwrap();
        let numbers = iter
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        equations.push(Equation { result, numbers })
    }

    Input { equations }
}

fn backtrack(_target: u64, accs: &mut Vec<u64>, nums: &[u64], part2: bool) -> bool {
    if nums.is_empty() {
        return accs.last().unwrap() == &_target;
    }

    let next = nums[0];

    let operators = if part2 {
        vec![
            Operator::Add(next),
            Operator::Multiply(next),
            Operator::Concat(next),
        ]
    } else {
        vec![Operator::Add(next), Operator::Multiply(next)]
    };

    for operator in operators {
        let result = operator.op(accs[accs.len() - 1]);

        if result == _target && nums.len() == 1 {
            return true;
        }

        accs.push(result);
        if backtrack(_target, accs, &nums[1..], part2) {
            return true;
        }
        accs.pop();
    }

    false
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mut sum = 0;

    for equation in &input.equations {
        let target = equation.result;

        let mut accs = vec![equation.numbers[0]];
        let nums = &equation.numbers[1..];

        if backtrack(target, &mut accs, nums, false) {
            sum += equation.result;
        }
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    let sum = input
        .equations
        .par_iter()
        .map(|eq| {
            let target = eq.result;

            let mut accs = vec![eq.numbers[0]];
            let nums = &eq.numbers[1..];

            if backtrack(target, &mut accs, nums, true) {
                eq.result
            } else {
                0
            }
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day07/test.txt");
    #[test]
    fn test_day07_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(3749));
    }

    #[test]
    fn test_day07_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(11387));
    }
}
