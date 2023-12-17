mod utils;

use core::num;
use std::collections::HashMap;
use std::fs::{File, self};
use std::{i32, cmp};
use std::io::{self, BufRead};
use std::path::Path;
use utils::read_lines;

struct Game {
    id: i32,
    samples: Vec<HashMap<String, i32>>
}

fn main() {
    let input_path = "inputs/2.txt";
    // File hosts.txt must exist in the current path

    // one 
    let games: Vec<i32> = process_file(input_path, process_game);

    println!("Games {:?}", games);
    println!("Games {:?}", games.iter().sum::<i32>());
}

fn process_file<F>(input_path: &str, processor: F) -> Vec<i32> where F: Fn(String) -> i32 {
    let mut result: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                result.push(processor(ip));
            }
        }
    }
    return result;
}

fn process_game(s: String) -> i32 {
    let mut split_s =  s.split(":");

    let game_info = split_s.next().unwrap();
    let sample_info= split_s.next().unwrap();

    // let game_id = game_info.rsplit_once(" ").unwrap().1;
    return get_game_power(sample_info);
}

fn get_game_possibility(s: &str, max_red : i32, max_green : i32, max_blue : i32) -> bool   {
    let mut possibilities = s.split(",").map(|sample| {
        let mut color_data = sample.replace("  ", " ");
        println!("COLOR_DATA {}", color_data);
    
        let color: String = color_data.chars().filter(|c| c.is_alphabetic()).collect();
        let amount: String = color_data.chars().filter(|c| c.is_numeric()).collect();

        let possible: bool = match color.as_str(){
            "red" => amount.parse::<i32>().unwrap() <= max_red,
            "green" => amount.parse::<i32>().unwrap() <= max_green,
            "blue" => amount.parse::<i32>().unwrap() <= max_blue,
            _ => false
        };

        return possible;
    });

    print!("{:?} ", possibilities);
    // possibilities.clone().for_each(|p| println!("{}", p));

    return !possibilities.any(|p| p == false);  
}   

fn get_game_power(s: &str ) -> i32   {
    let mut game_min: HashMap<&str, i32> = HashMap::new();
    game_min.insert("red", 0);
    game_min.insert("blue", 0);
    game_min.insert("green", 0);

    for game in s.split(";") {
        game.split(",").for_each(|sample| {
            let color_data: String = sample.replace("  ", " ");
            println!("COLOR_DATA {}", color_data);

            let color: String = color_data.chars().filter(|c| c.is_alphabetic()).collect();
            let amount: String = color_data.chars().filter(|c| c.is_numeric()).collect();

            let num_amount = amount.parse::<i32>().unwrap();
            match color.as_str(){
                "red" => game_min.insert("red", cmp::max(*game_min.get("red").unwrap(), num_amount)).unwrap(),
                "blue" => game_min.insert("blue", cmp::max(*game_min.get("blue").unwrap(), num_amount)).unwrap(),
                "green" => game_min.insert("green", cmp::max(*game_min.get("green").unwrap(), num_amount)).unwrap(),
                _ => 0
            };
        });
    }
    
    // possibilities.clone().for_each(|p| println!("{}", p));

    println!("{} {} {}", game_min.get("red").unwrap(), game_min.get("blue").unwrap(), game_min.get("green").unwrap());
    return game_min.get("green").unwrap() * game_min.get("red").unwrap() * game_min.get("blue").unwrap();
}  