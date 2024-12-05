use std::fs::File;
use std::io::{self, Read};

fn eval_dimension(buffer: &[u8]) -> (usize, usize) {
    let mut width = 0;
    for char in buffer {
        width += 1;
        if *char == b'\n' {
            break;
        }
    }
    let height = buffer.len() / width;
    //println!("width: {width}, height: {height}, {}", buffer.len());

    (width, height)
}

fn evaluate(buffer: &[u8]) {
    let (width, height) = eval_dimension(buffer);

    let mut count = 0;
    let mut index = 0;

    for j in 0..height {
        for i in 0..width {
            if buffer[index] != b'X' {
                index += 1;
                continue;
            }

            if i < width - 3 && buffer[index..index + 4].to_ascii_uppercase() == b"XMAS" {
                count += 1;
            }

            if j < height - 3 {
                if buffer[index + width] == b'M'
                    && buffer[index + width * 2] == b'A'
                    && buffer[index + width * 3] == b'S'
                {
                    count += 1;
                }

                if i < width - 3
                    && buffer[index + width + 1] == b'M'
                    && buffer[index + width * 2 + 2] == b'A'
                    && buffer[index + width * 3 + 3] == b'S'
                {
                    count += 1;
                }

                if i > 2
                    && buffer[index + width - 1] == b'M'
                    && buffer[index + width * 2 - 2] == b'A'
                    && buffer[index + width * 3 - 3] == b'S'
                {
                    count += 1;
                }
            }

            if i > 2 && buffer[index - 3..index + 1].to_ascii_uppercase() == b"SAMX" {
                count += 1;
            }

            if j > 2 {
                if buffer[index - width] == b'M'
                    && buffer[index - width * 2] == b'A'
                    && buffer[index - width * 3] == b'S'
                {
                    count += 1;
                }

                if i < width - 3
                    && buffer[index - width + 1] == b'M'
                    && buffer[index - width * 2 + 2] == b'A'
                    && buffer[index - width * 3 + 3] == b'S'
                {
                    count += 1;
                }

                if i > 2
                    && buffer[index - width - 1] == b'M'
                    && buffer[index - width * 2 - 2] == b'A'
                    && buffer[index - width * 3 - 3] == b'S'
                {
                    count += 1;
                }
            }

            index += 1;
        }
    }
    println!("XMAS: {count}");
}

fn evaluate_x(buffer: &[u8]) {
    let (width, height) = eval_dimension(buffer);

    let mut count = 0;

    for j in 1..height - 1 {
        for i in 1..width - 1 {
            let index = j * width + i;

            if buffer[index] != b'A' {
                continue;
            }
            if (buffer[index - width - 1] == b'M' && buffer[index + width + 1] == b'S'
                || buffer[index - width - 1] == b'S' && buffer[index + width + 1] == b'M')
                && (buffer[index - width + 1] == b'M' && buffer[index + width - 1] == b'S'
                    || buffer[index - width + 1] == b'S' && buffer[index + width - 1] == b'M')
            {
                count += 1;
            }
        }
    }
    println!("X-MAS: {count}");
}

fn main() {
    let file_path = "./day4/input.txt";

    let Ok(file) = File::open(file_path) else {
        println!("Error on file open");
        return;
    };

    let mut buffer = vec![];
    _ = io::BufReader::new(file).read_to_end(&mut buffer);
    evaluate(&buffer);
    evaluate_x(&buffer);
}
