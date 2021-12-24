use std::collections::HashMap;
use std::io;

fn main() {
    println!("Enter some integers: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut numbers = Vec::new();

    for number in input.split_whitespace() {
        match number.parse::<i64>() {
            Ok(n) => numbers.push(n),
            Err(e) => println!("{} not a valid integer", e),
        };
    }

    println!("\nMean:\t{}", calculate_mean(&mut numbers));
    println!("Median:\t{}", calculate_median(&mut numbers));
    println!("Mode:\t{}", calculate_mode(&mut numbers));
}

/// Calculates the mean of a vector of integers
fn calculate_mean(numbers: &[i64]) -> f64 {
    let sum: i64 = Iterator::sum(numbers.iter());
    sum as f64 / numbers.len() as f64
}

/// Calculates the median of a vector of integers
fn calculate_median(numbers: &mut [i64]) -> f64 {
    let len = numbers.len();
    numbers.sort_unstable();
    let mid = len / 2;

    if len % 2 == 0 {
        calculate_mean(&numbers[(mid - 1)..(mid + 1)])
    } else {
        numbers[mid] as f64
    }
}

/// Calculates the mode of a vector of integers
fn calculate_mode(numbers: &mut [i64]) -> i64 {
    let mut map = HashMap::new();

    for n in numbers {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }

    let mode = map.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap();
    **mode.0
}
