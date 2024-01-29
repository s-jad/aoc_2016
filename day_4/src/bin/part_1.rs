use itertools::Itertools;
use std::{collections::BTreeMap, time::Instant};

fn process(input: &str) -> usize {
    let mut rooms = input
        .lines()
        .map(|l| {
            let mut check = false;

            l.chars().fold(
                (BTreeMap::new(), Vec::new(), String::new()),
                |(mut letters, mut checksum, mut id), c| match (check, c) {
                    (false, '[') => {
                        check = true;
                        (letters, checksum, id)
                    }
                    (false, ch) if ch.is_alphabetic() => {
                        *letters.entry(ch).or_insert(0) += 1;
                        (letters, checksum, id)
                    }
                    (true, ch) if ch.is_alphabetic() => {
                        checksum.push(ch);
                        (letters, checksum, id)
                    }
                    (_, ch) if ch.is_numeric() => {
                        id.push(ch);
                        (letters, checksum, id)
                    }
                    _ => (letters, checksum, id),
                },
            )
        })
        .collect_vec();

    let mut total = 0;

    for (letters, checksum, id) in rooms.iter_mut() {
        let mut checks = [false; 5];
        let mut vals = letters
            .iter()
            .sorted_by(|x, y| x.1.cmp(&y.1).then_with(|| (*y.0 as u8).cmp(&(*x.0 as u8))))
            .collect_vec();

        for i in 0..5 {
            let current = vals.pop().unwrap();
            let check = checksum[i];

            if current.0 == &check {
                checks[i] = true;
            }
        }

        if checks.iter().all(|c| c == &true) {
            total += id.parse::<usize>().unwrap();
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
