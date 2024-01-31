use itertools::Itertools;
use std::{collections::VecDeque, time::Instant};

fn process(input: &str) -> usize {
    let len_x = 50;
    let len_y = 6;
    let v = vec!['.'; len_x];
    let row = VecDeque::from(v);
    let mut grid = Vec::with_capacity(len_y);

    for _ in 0..len_y {
        grid.push(row.clone());
    }

    let instructions = input
        .lines()
        .map(|instruction| {
            instruction
                .split_terminator(&[' ', '\n', '=', 'x', 'y', 'b'])
                .filter(|s| !s.is_empty())
                .collect_vec()
        })
        .collect_vec();

    for ins in instructions {
        if ins.len() == 3 {
            let x = ins[1].parse::<usize>().unwrap();
            let y = ins[2].parse::<usize>().unwrap();

            for dy in 0..y {
                for dx in 0..x {
                    grid[dy][dx] = '#';
                }
            }
        } else {
            match ins[1] {
                "row" => {
                    let yidx = ins[2].parse::<usize>().unwrap();
                    let amt = ins[3].parse::<usize>().unwrap();
                    for _ in 0..amt {
                        let temp = grid[yidx].pop_back().unwrap();
                        grid[yidx].push_front(temp);
                    }
                }
                "column" => {
                    let xidx = ins[2].parse::<usize>().unwrap();
                    let amt = ins[3].parse::<usize>().unwrap();

                    for _ in 0..amt {
                        let mut prev = grid[len_y - 1][xidx];
                        for y in 0..len_y {
                            let temp = grid[y][xidx];
                            grid[y][xidx] = prev;
                            prev = temp;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    let display = grid
        .iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect_vec();

    println!("display:\n");
    for s in display {
        println!("{s}");
    }
    1
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
