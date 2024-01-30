use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Get the file path from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // Open the file
    let file_path = &args[1];
    let file = match File::open(&Path::new(file_path)) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            std::process::exit(1);
        }
    };

    // Read values from the file
    let values: Vec<i32> = io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok().and_then(|s| s.parse().ok()))
        .collect();

    // Calculate statistics
    let average: i32 = values.iter().sum::<i32>() / values.len() as i32;
    let mut sorted_values = values.clone();
    sorted_values.sort();

    let median = if values.len() % 2 == 0 {
        let mid = values.len() / 2;
        (sorted_values[mid - 1] + sorted_values[mid]) / 2
    } else {
        sorted_values[values.len() / 2]
    };

    let variance = values.iter().map(|x| (x - average).pow(2)).sum::<i32>() / values.len() as i32;
    let std_deviation = (variance as f64).sqrt() as i32;

    // Print the results
    println!("Average: {}", average);
    println!("Median: {}", median);
    println!("Variance: {}", variance);
    println!("Standard Deviation: {}", std_deviation);
}
