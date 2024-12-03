use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "./day1/input.txt";

    let Ok(file) = File::open(file_path) else {
        return;
    };

    let ids = io::BufReader::new(file)
        .lines()
        .flatten()
        .filter_map(|line| {
            let mut token = line.split_whitespace();
            match (token.next(), token.next()) {
                (Some(a), Some(b)) => match (a.parse::<i32>(), b.parse::<i32>()) {
                    (Ok(a), Ok(b)) => Some((a, b)),
                    _ => None,
                },
                _ => None,
            }
        })
        .collect::<Vec<(_, _)>>();

    let mut left = ids.iter().map(|(a, _)| a.clone()).collect::<Vec<_>>();
    let mut right = ids.into_iter().map(|(_, b)| b).collect::<Vec<_>>();

    left.sort();
    right.sort();

    let sum: i32 = left
        .iter()
        .zip(right.iter_mut())
        .map(|(a, b)| (*a - *b).abs())
        .sum();

    let mut similarity = 0;

    let mut right_iter = right.iter().peekable();
    for a in left {
        loop {
            match right_iter.peek() {
                Some(&&b) if a > b => {
                    right_iter.next();
                }
                _ => break,
            }
        }

        let mut count = 0;
        loop {
            match right_iter.peek() {
                Some(&&b) if a == b => {
                    count += 1;
                    right_iter.next();
                }
                _ => break,
            }
        }

        similarity += a * count;

        if right_iter.peek().is_none() {
            break;
        }
    }

    println!("{sum} {similarity}");
}
