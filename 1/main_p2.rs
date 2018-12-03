use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);
    let mut frequency = 0;
    let mut frequency_counts = HashMap::new();

    let input_lines: Vec<String> = f
        .lines()
        .map(|l| l.expect("Could not read input line"))
        .collect();

    'outer: loop {
        'inner: for input_line in &input_lines {
            let change_op = &input_line[..1];
            let change_value: i32 = (&input_line[1..])
                .parse()
                .expect("Non number encountered in the input!");

            match change_op {
                "+" => frequency += change_value,
                "-" => frequency -= change_value,
                _ => panic!(
                    "Invalid frequency change input encountered - {}",
                    input_line
                ),
            };

            let count = frequency_counts.entry(frequency.to_string()).or_insert(0);
            *count += 1;

            if *count == 2 {
                println!("Reached count two for frequency - {}", frequency);
                break 'outer;
            }
        }
    }
}
