use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug,PartialEq)]
enum GameChoice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug,PartialEq)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

fn game_result_for_choice_a(choice_a: &GameChoice, choice_b: &GameChoice) -> GameResult {
    if *choice_a == *choice_b {
        GameResult::Draw
    } else if (*choice_a == GameChoice::Rock && *choice_b == GameChoice::Scissors) || 
                (*choice_a == GameChoice::Scissors && *choice_b == GameChoice::Paper) ||
                (*choice_a == GameChoice::Paper && *choice_b == GameChoice::Rock) {
        GameResult::Win
    } else {
        GameResult::Lose
    }
}

fn player_1_to_game_choice(player: char) -> GameChoice {
    match player {
        'A' => GameChoice::Rock,
        'B' => GameChoice::Paper,
        'C' => GameChoice::Scissors,
        _ => panic!("Illegal character for player 1 {}", player),
    }
}

fn player_2_to_game_choice(player: char) -> GameChoice {
    match player {
        'X' => GameChoice::Rock,
        'Y' => GameChoice::Paper,
        'Z' => GameChoice::Scissors,
        _ => panic!("Illegal character for player 2 {}", player),
    }
}

fn player_2_to_game_result(target_result: char) -> GameResult {
    match target_result {
        'X' => GameResult::Lose,
        'Y' => GameResult::Draw,
        'Z' => GameResult::Win,
        _ => panic!("Illegal character for player 2 {}", target_result),
    }
}

fn determine_choise_for_result(choice: &GameChoice, result: &GameResult) -> GameChoice {
    match [GameChoice::Paper, GameChoice::Rock, GameChoice::Scissors]
        .into_iter()
        .filter(|c| game_result_for_choice_a(c, choice) == *result)
        .collect::<Vec<_>>()[0] {
            GameChoice::Paper => GameChoice::Paper,
            GameChoice::Rock => GameChoice::Rock,
            GameChoice::Scissors => GameChoice::Scissors,
        }
}

fn calc_score(result: GameResult, choice:GameChoice) -> i32 {
    let mut score: i32 = 0;
    match choice {
        GameChoice::Rock => score += 1,
        GameChoice::Paper => score += 2,
        GameChoice::Scissors => score += 3,
    }
    match result {
        GameResult::Lose => score += 0,
        GameResult::Draw => score += 3,
        GameResult::Win => score += 6,
    }
    score
}

pub fn day_02() {
    let filename = "inputs/input_day_02.txt";

    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let result = reader
                .lines()
                .map(|line| if let Ok(l) = line { Option::Some(l.chars().collect::<Vec<char>>()) } else {Option::None})
                .map(|line| line.map(|l| (player_1_to_game_choice(l[0]), player_2_to_game_choice(l[2]))))
                .map(|game| game.map(|g| (game_result_for_choice_a(&g.1, &g.0), g.0, g.1)))
                .map(|played| played.map(|p| calc_score(p.0, p.2)))
                .map(|o| o.unwrap())
                .sum::<i32>()
                // .collect::<Vec<_>>()
                ;
            println!("day02 Part1: {:?}", result);
        },
        Err(_) => println!("Error opening the input file!"),
    };

    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let result = reader
                .lines()
                .map(|line| line.unwrap().chars().collect::<Vec<char>>())
                .map(|line| (player_2_to_game_result(line[2]), player_1_to_game_choice(line[0])))
                .map(|game| (determine_choise_for_result(&game.1, &game.0), game.0, game.1))
                .map(|played| calc_score(played.1, played.0))
                .sum::<i32>()
                // .collect::<Vec<_>>()
                ;
            println!("day02 Part2: {:?}", result);
        },
        Err(_) => println!("Error opening the input file!"),
    };
}