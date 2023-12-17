use std::collections::HashMap;
use std::fs::{File, self};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_path = "inputs/1.txt";
    // File hosts.txt must exist in the current path

    // one 
    let result: Vec<i32> = process_file(input_path, get_numbers);
    let sum = result.iter().fold(0, |acc, n| acc+n);

    println!("RESULT {}", sum);
}

fn get_numbers(s: String) -> i32 {
    let written_numbers: HashMap<&str, i32> = HashMap::from([
        ("one", 1), 
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    
    //stores index and occurrences
    let mut occurrences: HashMap<i32, i32> = HashMap::new();

    for (k, v) in written_numbers {
        //searches for the first written number 
        let index: Option<usize> = s.find(k);
        if index.is_some() {
            let index = index.unwrap();
            occurrences.insert(index as i32, v);
        }

        //searches for the literal number
        let index: Option<usize> = s.find(v.to_string().as_str());
        if index.is_some() {
            let index = index.unwrap();
            occurrences.insert(index as i32, v);
        }

        //searches for the last written number 
        let index: Option<usize> = s.rfind(k);
        if index.is_some() {
            let index = index.unwrap();
            occurrences.insert(index as i32, v);
        }

        //searches for the last literal number
        let index: Option<usize> = s.rfind(v.to_string().as_str());
        if index.is_some() {
            let index: usize = index.unwrap();
            occurrences.insert(index as i32, v);
        }
    }

    let first_occurrence = occurrences.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().1;
    let last_occurrence = occurrences.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().1;

    let composed_number = format!("{}{}", first_occurrence, last_occurrence);
    let result = composed_number.parse::<i32>().unwrap();

    println!("{:?}", result);
    println!("{:?}\n", occurrences);

    return result
}

pub fn get_literal_numbers(s: String) -> i32 {
    let first_num = s.chars().find(|c| c.is_numeric()).ok_or("").expect("");
    let last_num = s.chars().rev().find(|c| c.is_numeric()).ok_or("").expect("");

    let result = format!("{}{}", first_num, last_num).parse::<i32>().unwrap();
    return result;
}

fn process_file<F>(input_path: &str, processor: F) -> Vec<i32> where F: Fn(String) -> i32 {
    let mut result: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(input_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                result.push(processor(ip));
            }
        }
    }
    return result;
}