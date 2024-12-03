pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();

    for line in input.lines() {
        let report = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        reports.push(report);
    }

    reports
}

#[allow(unused_variables)]
pub fn part1(input: &[Vec<i32>]) -> Option<u64> {
    let mut count = 0;

    for report in input {
        let mut iter = report.iter();

        let curr = iter.next().unwrap();
        let next = iter.next().unwrap();

        let coeff = if next > curr { 1 } else { -1 };

        let mut curr = Some(coeff * curr);
        let mut next = Some(coeff * next);

        let mut bad = false;
        while let (Some(c), Some(n)) = (curr, next) {
            let diff = n - c;
            if !(1..=3).contains(&diff) {
                bad = true;
                break;
            }

            curr = next;
            next = iter.next().map(|m| coeff * m);
        }

        if !bad {
            count += 1;
        }
    }

    Some(count)
}

#[allow(unused_variables)]
pub fn part2(input: &[Vec<i32>]) -> Option<u64> {
    let mut count = 0;

    for report in input {
        let is_mono_inc = report
            .windows(2)
            .all(|sl| (1..=3).contains(&(sl[1] - sl[0])));

        if is_mono_inc {
            count += 1;
            continue;
        }

        let is_mono_dec = report
            .windows(2)
            .all(|sl| (-3..=-1).contains(&(sl[1] - sl[0])));

        if is_mono_dec {
            count += 1;
            continue;
        }

        // increasing pass
        let src_diffs = report.windows(2).map(|sl| sl[1] - sl[0]);

        let mut pos = src_diffs.clone().position(|d| !(1..=3).contains(&d));

        let mut diffs = report.windows(2).map(|sl| sl[1] - sl[0]);

        let mut prev = 0;
        let mut fails = 0;
        // increasing pass
        for (i, d) in diffs.enumerate() {
            if !(1..=3).contains(&d) {
                fails += 1;
                if fails == 0 && (1..=3).contains(&(prev + d)) {
                    continue;
                }

                fails += 1;
                break;
            }
        }

        // // increasing pass
        // let mut iter = report.iter();
        // let mut curr = iter.next();
        // let mut next = iter.next();
        // let mut prev = None;
        //
        // let mut fails = 0;
        // while let (Some(c), Some(n)) = (curr, next) {
        //     let diff = n - c;
        //     if !(1..=3).contains(&diff) {
        //         fails += 1;
        //         if fails > 1 {
        //             break;
        //         }
        //
        //         let nextnext = iter.next();
        //         if nextnext.is_none_or(|nn| (1..=3).contains(&(nn - c))) {
        //             next = nextnext;
        //         } else if prev.is_some() {
        //             curr = prev;
        //             next = nextnext;
        //         } else {
        //             fails = 2;
        //             break;
        //         }
        //
        //         println!(
        //             "going to next iter with curr={:?}, next={:?}, fails={}",
        //             curr, next, fails
        //         );
        //         continue;
        //     }
        //
        //     prev = curr;
        //     curr = next;
        //     next = iter.next();
        //     println!(
        //         "going to next iter with curr={:?}, next={:?}, fails={}",
        //         curr, next, fails
        //     );
        // }
        //
        // if fails < 2 {
        //     count += 1;
        //     continue;
        // }
        //
        // // decreasing pass
        // let mut iter = report.iter().peekable();
        // let mut curr = iter.next();
        // let next = iter.peek();
        //
        // let tmp = *next.unwrap() + 1;
        // let mut next = curr;
        //
        // curr = Some(&tmp);
        //
        // let mut fails = 0;
        // while let (Some(c), Some(n)) = (curr, next) {
        //     let diff = n - c;
        //     if !(-3..=-1).contains(&diff) {
        //         fails += 1;
        //         if fails > 1 {
        //             break;
        //         }
        //
        //         next = iter.next();
        //
        //         println!(
        //             "going to next iter with curr={:?}, next={:?}, fails={}",
        //             curr, next, fails
        //         );
        //         continue;
        //     }
        //
        //     prev = curr;
        //     curr = next;
        //     next = iter.next();
        //     println!(
        //         "going to next iter with curr={:?}, next={:?}, fails={}",
        //         curr, next, fails
        //     );
        // }
        //
        // if fails < 2 {
        //     count += 1;
        // }
    }

    Some(count)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day02/test.txt");
    #[test]
    fn test_day00_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(1));
    }

    #[test]
    fn test_day00_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(4));
    }
}
