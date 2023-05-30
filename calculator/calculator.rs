use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter an expression: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit" {
            break;
        }

        match evaluate_expression(input) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn evaluate_expression(expression: &str) -> Result<f64, String> {
    let parts: Vec<&str> = expression.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid expression".to_string());
    }

    let num1: f64 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid number".to_string()),
    };

    let operator = parts[1];

    let num2: f64 = match parts[2].parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid number".to_string()),
    };

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                return Err("Division by zero".to_string());
            }
            num1 / num2
        }
        _ => return Err("Invalid operator".to_string()),
    };

    Ok(result)
}
