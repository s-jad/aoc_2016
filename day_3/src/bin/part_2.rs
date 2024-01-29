use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let mut total = 0;

    for mut c in &input.lines().chunks(3) {
        let l1 = c.next().unwrap();
        let l2 = c.next().unwrap();
        let l3 = c.next().unwrap();

        let vars = l1
            .split_whitespace()
            .into_iter()
            .zip(
                l2.split_whitespace()
                    .into_iter()
                    .zip(l3.split_whitespace().into_iter()),
            )
            .map(|(n1, (n2, n3))| {
                (
                    n1.parse::<usize>().unwrap(),
                    n2.parse::<usize>().unwrap(),
                    n3.parse::<usize>().unwrap(),
                )
            })
            .collect_vec();

        for (v1, v2, v3) in vars {
            match (v1 + v2) > v3 && (v1 + v3) > v2 && (v2 + v3) > v1 {
                true => {
                    total += 1;
                }
                _ => {}
            }
        }
    }
    total
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
