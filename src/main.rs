use std::io;
use std::io::Write;

fn main() {
    print!("Enter an array of numbers separated by spaces: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    println!("The sum of the numbers is: {}", sum);
}