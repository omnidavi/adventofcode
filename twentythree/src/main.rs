use std::fs::{File, self};
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let input_path = "inputs/1.txt";
    let output_path = "outputs/1.txt";
    // File hosts.txt must exist in the current path
    process_file(input_path, output_path, get_numbers);

    // write_output(output_path, output_path, processor(ip))
}

fn get_numbers(s: String) -> String {
    let first_num = s.chars().find(|c| c.is_numeric()).ok_or("").expect("");
    let last_num = s.chars().rev().find(|c| c.is_numeric()).ok_or("").expect("");

    let result = format!("{}{}", first_num, last_num);
    return result;
}

fn process_file<F>(input_path: &str, output_path: &str, processor: F) where F: Fn(String) -> String {
    if let Ok(lines) = read_lines(input_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                // println!("{}\n", processor(ip));
            }
        }
    }
}

fn write_output(filepath: &str, content: String) {
    fs::write(filepath, content);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
