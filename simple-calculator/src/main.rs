use std::io;

fn main() {
    println!("Enter an expression like: 5 + 3");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() == 3 {
        let left = match tokens[0].parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid number: {}", tokens[0]);
                return;
            }
        };
        let op = tokens[1];
        let right = match tokens[2].parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid number: {}", tokens[2]);
                return;
            }
        };

        let result = match op {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            "%" => left % right,
            _ => {
                println!("Unknown operator: {}", op);
                return;
            }
        };

        println!("= {}", result);
    } else {
        println!("❌ Invalid input. Use format like: 5 + 3");
    }
}
