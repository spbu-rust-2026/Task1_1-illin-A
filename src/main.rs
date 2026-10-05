use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    
    if io::stdin().read_to_string(&mut input).is_ok() {
        let mut numbers = input
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap());
        
        if let (Some(num1), Some(num2)) = (numbers.next(), numbers.next()) {
            println!("{}", num1 + num2);
        }
    }
}