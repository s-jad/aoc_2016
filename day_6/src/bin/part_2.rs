use itertools::Itertools;
use std::{collections::BTreeMap, time::Instant};

fn process(input: &str) -> String {
    let cols =
        input
            .lines()
            .map(|l| l.chars().collect_vec())
            .fold(BTreeMap::new(), |mut acc, line| {
                for i in 0..line.len() {
                    acc.entry(i)
                        .and_modify(|v: &mut Vec<char>| v.push(line[i]))
                        .or_insert_with(|| vec![line[i]]);
                }
                acc
            });

    cols.into_values()
        .map(|col| {
            col.into_iter()
                .counts()
                .into_iter()
                .min_by_key(|&(_, count)| count)
                .unwrap()
                .0
        })
        .collect::<String>()
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
