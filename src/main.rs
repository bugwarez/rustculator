//!First program in rust. OFC it is a calculator.

use std::io;

fn main() {
    println!("WELCOME TO THE MIGHTY RUSTCULATOR!!\nTO TEST IT, SIMPLY TYPE YOUR QUESTION.\nLIKE THIS -> 1 + 2. PUT SPACES AROUND CHARACTERS OR IT'LL BRAKE(I COULDNT FIX IT :/ ANY FIX PR WILL BE ACCEPTED. THANKS<3)");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut tokens: std::str::SplitWhitespace = input.split_whitespace();

    let num1: f64 = tokens.next().unwrap().parse::<f64>().unwrap();
    let operator: &str = tokens.next().unwrap();
    let num2: f64 = tokens.next().unwrap().parse::<f64>().unwrap();

    let result: f64 = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => panic!("Invalid operator"),
    };

    println!("{}", result);
}
