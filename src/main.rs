use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use num_traits::float::Float;

// Round a floating-point number to the nearest integer
fn round_to_nearest<T: Float>(value: T) -> i64 {
    value.round().to_i64().unwrap_or_else(|| value.to_f64().unwrap_or(0.0) as i64)
}


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
    let values: Vec<i64> = io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok().and_then(|s| s.parse().ok()))
        .collect();

    // Calculate statistics
    let average: f64 = values.iter().sum::<i64>() as f64 / values.len() as f64;

    let variance = values
        .iter()
        .map(|&x| (x as f64 - average).powi(2))
        .sum::<f64>()
        / values.len() as f64;
    let std_deviation = round_to_nearest(variance.sqrt());

    // Print the results as integers
    println!("Average: {}", round_to_nearest(average));
    let mut sorted_values = values.clone();
    sorted_values.sort();
    if sorted_values[values.len() / 2] / sorted_values[values.len() / 2 - 1] == 1{
    println!("Median: {}", sorted_values[values.len() / 2]);
    } else {
        let median1 =  sorted_values[values.len()/2-1];
        let median2:  i64=sorted_values[values.len()/2];
        println!("Median: {:.1}", (median1 + median2)/2);
    }
    println!("Variance: {}", round_to_nearest(variance));
    println!("Standard Deviation: {}", std_deviation);
}
