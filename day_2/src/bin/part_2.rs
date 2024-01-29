use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> String {
    let keypad = [
        ['.', '.', '1', '.', '.'],
        ['.', '2', '3', '4', '.'],
        ['5', '6', '7', '8', '9'],
        ['.', 'A', 'B', 'C', '.'],
        ['.', '.', 'D', '.', '.'],
    ];

    let mut x = 0;
    let mut y = 2;
    let dim = 4;
    let mut num = String::new();

    for l in input.lines() {
        for c in l.trim().chars() {
            let x_border = (2 - y as i32).abs() as usize;
            let y_border = (2 - x as i32).abs() as usize;
            match c {
                'U' => {
                    if y > y_border {
                        y -= 1
                    }
                }
                'D' => {
                    if y < dim - y_border {
                        y += 1
                    }
                }
                'L' => {
                    if x > x_border {
                        x -= 1
                    }
                }
                'R' => {
                    if x < dim - x_border {
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
