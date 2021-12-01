/**
 * Challenge 1_1
 * > https://adventofcode.com/2021/day/1
 */
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let filename = "input/input_test.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let previous_line = 0;
    let count = 0;

    for line in reader.lines() {
        println!("{}", line?);

        if previous_line != 0 {
            count = count + 1;
        }
        previous_line = line.parse().unwrap();
    }

    Ok(())
}
