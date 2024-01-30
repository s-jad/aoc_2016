use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

const GOAL: &str = "northpoleobjects";

fn check_room<'a>(encrypted: &'a mut str, id: &'a str, goal: &'a str) -> Option<&'a str> {
    let unencrypted = encrypted
        .chars()
        .map(|c| {
            if c.is_alphabetic() && c.is_lowercase() {
                ((((c as usize - 97) + id.parse::<usize>().unwrap()) % 26) as u8 + 97) as char
            } else if c.is_alphabetic() && c.is_uppercase() {
                ((((c as usize - 65) + id.parse::<usize>().unwrap()) % 26) as u8 + 65) as char
            } else {
                ' '
            }
        })
        .collect::<String>();

    println!("unencrypted: {unencrypted:?}");
    if unencrypted.contains(goal) {
        return Some(id);
    } else {
        None
    }
}

fn process(input: &str) -> String {
    let mut rooms = input
        .lines()
        .map(|l| {
            let mut check = false;

            l.chars().fold(
                (String::new(), Vec::new(), String::new()),
                |(mut msg, mut checksum, mut id), c| match (check, c) {
                    (false, '[') => {
                        check = true;
                        (msg, checksum, id)
                    }
                    (_, ch) if ch.is_numeric() => {
                        id.push(ch);
                        (msg, checksum, id)
                    }
                    (false, ch) if ch.is_alphabetic() => {
                        msg.push(ch);
                        (msg, checksum, id)
                    }
                    (true, ch) if ch.is_alphabetic() => {
                        checksum.push(ch);
                        (msg, checksum, id)
                    }
                    _ => (msg, checksum, id),
                },
            )
        })
        .collect_vec();

    for (msg, checksum, id) in rooms.iter_mut() {
        let mut checks = [false; 5];
        let mut vals = msg
            .chars()
            .filter(|c| c.is_alphabetic())
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .sorted_by(|x, y| x.1.cmp(&y.1).then_with(|| (y.0 as u8).cmp(&(x.0 as u8))))
            .collect_vec();

        for i in 0..5 {
            let current = vals.pop().unwrap();
            let check = checksum[i];

            if current.0 == check {
                checks[i] = true;
            }
        }

        let room_id = if checks.iter().all(|c| c == &true) {
            check_room(msg.as_mut_str(), id.as_str(), GOAL)
        } else {
            None
        };

        if room_id.is_some() {
            return room_id.unwrap().to_string();
        }
    }
    return "0".to_string();
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
    fn test_check_room() {
        let id = "343";
        let mut encrypted = "qzmt-zixmtkozy-ivhz".to_string();
        let output = check_room(&mut encrypted, id, GOAL);
        assert_eq!(None, output);
    }
}
