use std::collections::HashMap;

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let mut sp = line.split_whitespace();

        let l = sp.next().unwrap();
        let l = l.parse::<i32>().unwrap();
        left.push(l);

        let r = sp.next().unwrap();
        let r = r.parse::<i32>().unwrap();
        right.push(r);
    }

    left.sort();
    right.sort();

    (left, right)
}

#[allow(unused_variables)]
pub fn part1(input: &(Vec<i32>, Vec<i32>)) -> Option<u64> {
    let iter = input.0.iter().zip(input.1.iter());
    let mut sum = 0;

    for (l, r) in iter {
        let diff = if l > r { l - r } else { r - l };
        sum += diff;
    }

    Some(sum as u64)
}

#[allow(unused_variables)]
pub fn part2(input: &(Vec<i32>, Vec<i32>)) -> Option<u64> {
    let mut counts = HashMap::new();
    let (left, right) = input;

    for n in right.iter() {
        counts.entry(n).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut sum = 0;
    for n in left.iter() {
        let right_count = counts.get(n).unwrap_or(&0);

        sum += n * right_count;
    }

    Some(sum as u64)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day01/test.txt");
    #[test]
    fn test_day00_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(11));
    }

    #[test]
    fn test_day00_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(31));
    }
}
