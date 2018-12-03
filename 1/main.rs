use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);
    let mut frequency = 0;

    for line in f.lines() {
        let input_line = line.unwrap();
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
    }
    println!("Final frequency value - {}", frequency);
}
