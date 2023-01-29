use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn day_03() {
    let filename = "inputs/input_day_03.txt";

    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let result = reader
                .lines()
                .map(|line| {if let Ok(l) = line {
                    let length = l.len();
                    (l[0..length/2].chars().collect::<HashSet<char>>(), l[length/2..].chars().collect::<HashSet<char>>())} else {(HashSet::new(), HashSet::new())}})
                .map(|(start_set, end_set)| *(start_set.intersection(&end_set).collect::<Vec<&char>>()[0]) as u32)
                .map(|c| if c >= 'a' as u32 && c <= 'z' as u32 {c + 1 - 'a' as u32} else {c + 27 - 'A' as u32})
                .sum::<u32>()
                ;
            println!("day03 Part1: {:?}", result);
        },
        Err(_) => println!("Error opening the input file!"),
    };

    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let result1 = reader
                .lines()
                .map(|line| if let Ok(l) = line {l.chars().collect::<HashSet<char>>()} else {HashSet::new()})
                .collect::<Vec<_>>()
                ;
            let result = result1
                .chunks(3)
                .map(|ch| *(ch[0].iter().filter(|c| ch[1].contains(c) && ch[2].contains(c)).collect::<Vec<_>>()[0]) as u32)
                .map(|c| if c >= 'a' as u32 && c <= 'z' as u32 {c + 1 - 'a' as u32} else {c + 27 - 'A' as u32})
                .sum::<u32>()
                // .collect::<Vec<_>>()
                ;
            println!("day03 Part2: {:?}", result);
        },
        Err(_) => println!("Error opening the input file!"),
    };

}