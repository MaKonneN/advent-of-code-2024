use std::fs::File;
use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Clone)]
enum Status {
    Safe,
    SafeWithDampener,
    Unsafe,
}

fn evaluate(sequence: &Vec<i32>) -> Status {
    let Some(mut previous) = sequence.first().cloned() else {
        panic!("No token found");
    };

    let mut sign = 0;
    let mut status = Status::Safe;
    for current in sequence.iter().skip(1) {
        let diff = current - previous;
        if sign == 0 {
            sign = diff.signum();
        } else if sign != diff.signum() {
            if status == Status::Safe {
                status = Status::SafeWithDampener;
                continue;
            }
            status = Status::Unsafe;
            break;
        }

        if sign == 0 || diff.abs() > 3 {
            if status == Status::Safe {
                status = Status::SafeWithDampener;
                continue;
            }
            status = Status::Unsafe;
            break;
        }
        previous = current.clone();
    }
    status
}

fn main() {
    let file_path = "./day2/input.txt";

    let Ok(file) = File::open(file_path) else {
        println!("Error on file open");
        return;
    };

    let mut safe_count = 0;
    let mut safe_count_with_dampener = 0;
    let mut unsafe_count = 0;

    io::BufReader::new(file).lines().flatten().for_each(|line| {
        let mut sequence = line
            .split_whitespace()
            .map(|token| token.parse::<i32>())
            .flatten()
            .collect::<Vec<_>>();

        let mut status = evaluate(&sequence);

        if status == Status::Unsafe {
            sequence.reverse();
            status = evaluate(&sequence);
        }

        match status {
            Status::Safe => safe_count += 1,
            Status::SafeWithDampener => {
                safe_count += 1;
                safe_count_with_dampener += 1;
            }
            Status::Unsafe => unsafe_count += 1,
        }
    });

    println!(
        "Safe: {safe_count} (with dampener: {safe_count_with_dampener}), Unsafe: {unsafe_count}"
    );
}
