mod utils;

use std::collections::HashMap;
use std::fs::{File, self};
use std::io::{self, BufRead};
use std::path::Path;
use utils::read_lines;

fn main() {
    let input_path = "inputs/3.txt";
    let matrix: Vec<Vec<i32>> = process_file(input_path);

    // let sum = result.iter().fold(0, |acc, n| acc+n);

    println!("RESULT {:?}", matrix);
    matrix.iter().for_each(|x| println!("{:?}", x));

    sum_valid_numbers(matrix);
}

fn process_file(input_path: &str) -> Vec<Vec<i32>>{
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines(input_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut line: Vec<i32> = Vec::new();
                ip.chars().for_each(|x| line.push(char_to_number(x)));
                println!("{:?}", line);
                matrix.push(line);
            }
        }
    }
    return matrix;
}

fn char_to_number(s: char) -> i32 {
    if s.is_numeric() {
        return s.to_digit(10).unwrap() as i32;
    }
    else if s == '.' {
        return -1;
    }
    else {
        return -2;
    }
}

fn sum_valid_numbers(matrix: Vec<Vec<i32>>) {
    let mut sum = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == -1 || matrix[i][j] == -2 {
                continue;
            }
            else {
                sum += 
            } 
        }
    }
}