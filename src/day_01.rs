use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::{Itertools};
 
pub fn day_01() {
    let filename = "inputs/input_day_01.txt";

    // To many collects...
    // match File::open(filename) {
    //     Ok(file) => {
    //         let reader = BufReader::new(file);
    //         let mut calories = reader
    //             .lines()
    //             .map(|line| if let Ok(l) = line {l.parse::<i32>().ok()} else {Option::None})
    //             .collect::<Vec<_>>()
    //             .split(|line| line.is_none())
    //             .map(|somes| somes.iter().map(|some| some.unwrap()).collect::<Vec<_>>())
    //             .collect::<Vec<_>>()
    //             .iter().map(|calories| calories.iter().sum::<i32>())
    //             .collect::<Vec<_>>();

    //         calories.sort();
    //         calories.reverse();
    //         println!("day01 Part1: {}", calories.iter().take(1).sum::<i32>());
    //         println!("day01 Part2: {}", calories.iter().take(3).sum::<i32>())
    //     },
    //     Err(_) => println!("Error opening the input file!"),
    // };

    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut calories = reader
                .lines()
                .map(|line| if let Ok(l) = line {l.parse::<i32>().ok()} else {Option::None})
                .group_by(|e| *e != None)
                .into_iter()
                .filter(|(k, _)| *k == true)
                .map(|(_, g)| g.map(|o| o.unwrap()).sum::<i32>())
                .collect::<Vec<i32>>();

            calories.sort();
            calories.reverse();
            println!("day01 Part1: {}", calories.iter().take(1).sum::<i32>());
            println!("day01 Part2: {}", calories.iter().take(3).sum::<i32>())
        },
        Err(_) => println!("Error opening the input file!"),
    };
}
