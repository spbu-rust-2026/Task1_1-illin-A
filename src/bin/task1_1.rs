use std::io;
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut numbers = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    let num1 = numbers.next().unwrap();
    let num2 = numbers.next().unwrap();

    println!("{}", num1 + num2);
}
