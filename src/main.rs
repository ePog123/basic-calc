mod functions;
use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut function = String::new();

    println!("Enter 1st number");
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    let num1: f64 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Enter a number"),
    };
    println!("Enter function");
    io::stdin()
        .read_line(&mut function)
        .expect("Failed to read line");

    println!("Enter 2nd number");
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    let num2: f64 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Enter a number"),
    };

    loop {
        if function.trim() == "+".to_string() {
            println!("{} {} {} = {}", num1, function.trim(), num2, functions::add(num1, num2));
            break;
        } else if function.trim() == "*".to_string() {
            println!("{} {} {} = {}", num1, function.trim(), num2, functions::mlt(num1, num2));
            break;
        } else if function.trim() == "/".to_string() {
            println!("{} {} {} = {}", num1, function.trim(), num2, functions::div(num1, num2));
            break;
        } else if function.trim() == "-".to_string() {
            println!("{} {} {} = {}", num1, function.trim(), num2, functions::sub(num1, num2));
            break;
        } else {
            panic!("Enter valid function")
        }
    }
}
