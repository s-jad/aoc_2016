use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    input.lines().fold(0usize, |mut total, l| {
        let (v1, v2, v3) = l
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect_tuple::<(_, _, _)>()
            .unwrap();

        match (v1 + v2) > v3 && (v1 + v3) > v2 && (v2 + v3) > v1 {
            true => {
                total += 1;
            }
            _ => {}
        }

        total
    })
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
