use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> i32 {
    let direction = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let (ex, ey) = input
        .split_terminator(&[',', ' ', '\n'])
        .into_iter()
        .filter(|s| !s.is_empty())
        .fold(((0i32, 0i32), 0i32), |((mut x, mut y), mut cd), s| {
            let lr = s.chars().next().expect("LR expected");
            let num = s[1..].parse::<i32>().expect("should be num");
            match lr {
                'L' => {
                    cd = (cd + 3) % 4;
                    let (dx, dy) = direction[cd as usize];
                    x = x + (dx * num);
                    y = y + (dy * num);
                    ((x, y), cd)
                }
                'R' => {
                    cd = (cd + 1) % 4;
                    let (dx, dy) = direction[cd as usize];
                    x = x + (dx * num);
                    y = y + (dy * num);
                    ((x, y), cd)
                }
                _ => unreachable!(),
            }
        })
        .0;

    (0 - ex).abs() + (0 - ey).abs()
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
