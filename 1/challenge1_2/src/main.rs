/**
 * Challenge 1_2
 * * Not Obtained
 * > https://adventofcode.com/2021/day/1
 */
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let filename = "input/input_test.txt";
    let list = BufReader::new(File::open(filename).unwrap())
	.lines()
	.collect::<Vec<_>>();

    // assignation des variables
    let mut resultat = 0;
    let mut previous_group: Vec<i32> = vec![];
    let mut current_group: Vec<i32> = vec![];

    for line in list {
        if let Ok(line_str) = line {
            println!("{}", line_str);
            let line_nbr = line_str.parse::<i32>().unwrap();

            if current_group.len() == 2 {
                if previous_group.len() == 0 {
                    previous_group = current_group;
                }
                // faire le calcul previous_group > current_group
                let mut previous_group_nbr = 0;
                for i in 0..previous_group.len() {
                    previous_group_nbr = previous_group_nbr + previous_group[i];
                }
                println!("> {}", previous_group_nbr);

                let mut current_group_nbr=0;
                for i in 0..current_group.len() {
                    current_group_nbr = current_group_nbr + current_group[i];
                }
                println!("> {}", current_group_nbr);

                if previous_group_nbr > current_group_nbr {
                    // faire l'increment du resultat si c'est bon
                    resultat = resultat + 1;
                }
                previous_group = current_group;
                current_group = vec![];
            } else {
                // ajouter le résultat actuel au tableau
                current_group.insert(current_group.len(), line_nbr);
            }
        }

    }

    println!("Résultat final : {}", resultat);

    Ok(())
}
