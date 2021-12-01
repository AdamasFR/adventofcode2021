/**
 * Challenge 1_1
 * * Obtained !
 * > https://adventofcode.com/2021/day/1
 */
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let filename = "input/input.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // assignation des variables
    let mut previous_line = 0;
    let mut count = 0;

    for line in reader.lines() {
        if let Ok(line_str) = line {
            println!("{}", line_str);
            let line_nbr = line_str.parse::<i32>().unwrap();
            if previous_line != 0 && previous_line < line_nbr {
                count = count + 1;
            }
            previous_line = line_nbr;
        }

    }

    println!("Résultat final : {}", count);

    Ok(())
}
