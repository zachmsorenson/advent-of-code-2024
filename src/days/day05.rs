use std::collections::{HashMap, HashSet};

pub struct Rules {
    pub pre: HashMap<i32, HashSet<i32>>,
    pub post: HashMap<i32, HashSet<i32>>,
}

pub struct Input {
    rules: Rules,
    updates: Vec<Vec<i32>>,
}

pub fn parse_input(input: &str) -> Input {
    let mut iter = input.lines();

    let mut pre = HashMap::new();
    let mut post = HashMap::new();
    for line in iter.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut sp = line.split("|");
        let first = sp.next().unwrap().parse::<i32>().unwrap();
        let second = sp.next().unwrap().parse::<i32>().unwrap();

        pre.entry(second)
            .and_modify(|v: &mut HashSet<i32>| {
                v.insert(first);
            })
            .or_insert(HashSet::from([first]));
        post.entry(first)
            .and_modify(|v: &mut HashSet<i32>| {
                v.insert(second);
            })
            .or_insert(HashSet::from([second]));
    }

    let updates = iter
        .map(|update| update.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    let rules = Rules { pre, post };
    Input { rules, updates }
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mut sum = 0;

    for update in input.updates.iter() {
        let mut seen = HashSet::new();
        let passes = update.iter().all(|&n| {
            seen.insert(n);

            match input.rules.post.get(&n) {
                Some(post) => post.intersection(&seen).next().is_none(),
                None => true,
            }
        });

        if passes {
            sum += update[update.len() / 2] as u64;
        }
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    let mut sum = 0;

    for update in input.updates.iter() {
        let mut seen = HashSet::new();
        let passes = update.iter().all(|&n| {
            seen.insert(n);
            match input.rules.post.get(&n) {
                Some(post) => post.intersection(&seen).next().is_none(),
                None => true,
            }
        });

        if passes {
            continue;
        }

        let all: HashSet<i32> = HashSet::from_iter(update.iter().cloned());

        let midpoint = all
            .iter()
            .map(|&n| {
                let count = match input.rules.pre.get(&n) {
                    Some(pre) => all.intersection(pre).count(),
                    None => 0,
                };

                (count, n)
            })
            .find(|&(count, n)| count == update.len() / 2);

        let midpoint = midpoint.map(|(count, n)| n).unwrap_or(0);

        sum += midpoint as u64;
    }

    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day05/test.txt");
    #[test]
    fn test_day05_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(143));
    }

    #[test]
    fn test_day05_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(123));
    }
}
