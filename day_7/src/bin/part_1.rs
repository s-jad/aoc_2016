use itertools::Itertools;
use std::time::Instant;

fn check_pattern(s: &str) -> bool {
    let mut valid = false;
    for (c1, c2, c3, c4) in s.chars().tuple_windows::<(_, _, _, _)>() {
        if c1 == c4 && c2 == c3 && c1 != c2 && c4 != c2 {
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

            for split in b {
                match check_pattern(split) {
                    true => return None,
                    false => {}
                }
            }

            for split in s {
                match check_pattern(split) {
                    true => return Some(1),
                    false => {}
                }
            }
            None
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
