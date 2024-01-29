use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> Vec<i32> {
    let keypad = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut x = 1;
    let mut y = 1;
    let mut num = vec![];

    for l in input.lines() {
        for c in l.trim().chars() {
            match c {
                'U' => {
                    if y > 0 {
                        y -= 1
                    }
                }
                'D' => {
                    if y < 2 {
                        y += 1
                    }
                }
                'L' => {
                    if x > 0 {
                        x -= 1
                    }
                }
                'R' => {
                    if x < 2 {
                        x += 1
                    }
                }
                _ => unreachable!(),
            }
        }
        num.push(keypad[y][x]);
    }

    num
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
