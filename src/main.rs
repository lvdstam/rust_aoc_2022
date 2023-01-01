use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "inputs/input_day_01.txt";

    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut calories = reader
                .lines()
                .map(|line| if let Ok(l) = line {l.parse::<i32>().ok()} else {Option::None})
                .collect::<Vec<_>>()
                .split(|line| line.is_none())
                .map(|somes| somes.iter().map(|some| some.unwrap()).collect::<Vec<_>>())
                .collect::<Vec<_>>()
                .iter().map(|calories| calories.iter().sum::<i32>())
                .collect::<Vec<_>>();

            println!("{}", calories.iter().max().unwrap());
            calories.sort();
            calories.reverse();
            println!("{}", calories.iter().take(3).sum::<i32>())
        },
        Err(_) => println!("Error opening the input file!"),
    };
}
