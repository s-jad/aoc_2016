use md5::Context;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn check_hash(s: &str) -> Option<(usize, char)> {
    let mut hasher = Context::new();
    hasher.consume(s.as_bytes());
    let digest = hasher.compute();
    let hex_digest = format!("{:x}", digest);

    if hex_digest.starts_with("00000") {
        let mut hd = hex_digest[5..7].chars();

        let n = hd.next().map(|c| c.to_digit(10)).unwrap();
        let mut num = std::usize::MAX;
        if n.is_some() {
            num = n.unwrap() as usize;
        }
        let c = hd.next().unwrap();
        if num < 8 {
            return Some((num, c));
        }
    }
    return None;
}

fn process(input: &str) -> String {
    let password = Arc::new(Mutex::new(vec!['!'; 8]));
    let mut handles = vec![];
    let start_idx = 0;

    for idx in start_idx..(start_idx + 4) {
        let password = Arc::clone(&password);
        let local_s = input.to_string();
        let handle = thread::spawn(move || {
            let mut idx = idx;
            while password.lock().unwrap().contains(&'!') {
                let mut s = String::new();
                s.push_str(&local_s);
                s.push_str(idx.to_string().as_str());
                match check_hash(&s) {
                    Some((pos, digit)) => {
                        if password.lock().unwrap()[pos] == '!' {
                            password.lock().unwrap()[pos] = digit;
                        }
                    }
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
