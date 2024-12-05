use std::fs::File;
use std::io::{self, Read};

use regex::Regex;

fn evaluate(line: &str, re: &Regex) -> i32 {
    let mut mul = 0;
    re.captures_iter(line)
        .enumerate()
        .map(|(_, m)| (m.get(1).unwrap().as_str(), m.get(2).unwrap().as_str()))
        .for_each(|(a, b)| mul += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap());

    mul
}

fn evaluate_with_directive(line: &str, re: &Regex) -> i32 {
    line.split("do()")
        .map(|section| {
            section
                .find("don't()")
                .and_then(|index| Some(section.split_at(index).0))
                .unwrap_or(section)
        })
        .map(|section| evaluate(section, &re))
        .sum::<i32>()
}
fn main() {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let file_path = "./day3/input.txt";

    let Ok(file) = File::open(file_path) else {
        println!("Error on file open");
        return;
    };

    let mut text = String::default();
    _ = io::BufReader::new(file).read_to_string(&mut text);

    println!(
        "mul: {}, with directive: {}",
        evaluate(text.as_str(), &re),
        evaluate_with_directive(text.as_str(), &re)
    );
}
