use itertools::Itertools;
use std::time::Instant;

fn check_pattern(s: &str, b: &str) -> bool {
    let mut valid = false;

    for ((s1, s2, s3), (b1, b2, b3)) in s
        .chars()
        .tuple_windows::<(_, _, _)>()
        .cartesian_product(b.chars().tuple_windows::<(_, _, _)>())
    {
        if s2 == b1 && s2 == b3 && b2 == s1 && b2 == s3 {
            valid = true;
        }
    }

    valid
}

fn process(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let (s, b) = l.split_terminator(&['[', ']', '\n'][..]).enumerate().fold(
                (Vec::new(), Vec::new()),
                |(mut s, mut b), (i, split)| match i % 2 {
                    0 => {
                        s.push(split);
                        (s, b)
                    }
                    1 => {
                        b.push(split);
                        (s, b)
                    }
                    _ => unreachable!(),
                },
            );

            let mut valid = None;
            for (s1, b1) in s.into_iter().cartesian_product(b.into_iter()) {
                if check_pattern(s1, b1) {
                    valid = Some(1);
                }
            }
            valid
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}
