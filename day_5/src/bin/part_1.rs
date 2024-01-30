use itertools::Itertools;
use md5::Context;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn check_hash(s: &str) -> Option<char> {
    let mut hasher = Context::new();
    hasher.consume(s.as_bytes());
    let digest = hasher.compute();
    let hex_digest = format!("{:x}", digest);

    if hex_digest.starts_with("00000") {
        println!("[{s}]");
        println!("[{hex_digest}]");
        return Some(hex_digest[5..6].chars().next().unwrap());
    }
    return None;
}

fn process(input: &str) -> String {
    let password = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for idx in 0..4 {
        let password = Arc::clone(&password);
        let local_s = input.to_string();
        let handle = thread::spawn(move || {
            let mut idx = idx;
            while password.lock().unwrap().len() != 8 {
                let mut s = String::new();
                s.push_str(&local_s);
                s.push_str(idx.to_string().as_str());
                match check_hash(&s) {
                    Some(digit) => password.lock().unwrap().push(digit),
                    None => {}
                }
                idx += 4;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let lock = Arc::try_unwrap(password).expect("Still has multiple owners");
    lock.into_inner().unwrap().into_iter().collect::<String>()
}

fn main() {
    let input = "ffykfhsq";

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
