use std::io::{self, Write};
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    fn from_str(input: &str) -> Result<Self, &'static str> {
        match input.to_lowercase().as_str() {
            "&" | "and" => Ok(Operation::And),
            "|" | "or" => Ok(Operation::Or),
            "^" | "xor" => Ok(Operation::Xor),
            _ => Err("Invalid operation"),
        }
    }

    fn apply(&self, lhs: u32, rhs: u32) -> u32 {
        match self {
            Operation::And => lhs & rhs,
            Operation::Or => lhs | rhs,
            Operation::Xor => lhs ^ rhs,
        }
    }
}

fn parse_number(input: &str) -> Result<u32, ParseIntError> {
    if let Some(hex_value) = input.strip_prefix("0x") {
        u32::from_str_radix(hex_value, 16) // Hexadecimal input
    } else if let Some(bin_value) = input.strip_prefix("0b") {
        u32::from_str_radix(bin_value, 2) // Binary input
    } else {
        input.parse::<u32>() // Decimal input
    }
}

fn main() {
    // Read first number
    let first_number = read_input("Please enter the first number: ");
    let lhs = match parse_number(&first_number) {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number format.");
            return;
        }
    };

    // Read second number
    let second_number = read_input("Please enter the second number: ");
    let rhs = match parse_number(&second_number) {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number format.");
            return;
        }
    };

    // Read operation
    let operation = read_input("Please enter the desired operation: ");
    let op = match Operation::from_str(&operation) {
        Ok(op) => op,
        Err(_) => {
            println!("Invalid operation.");
            return;
        }
    };

    // Calculate the result
    let result = op.apply(lhs, rhs);

    // Display the result
    println!("The result of {} {} {} is {}", lhs, operation, rhs, result);
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure prompt appears before input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_decimal() {
        assert_eq!(parse_number("12").unwrap(), 12);
    }

    #[test]
    fn test_parse_hexadecimal() {
        assert_eq!(parse_number("0xF8").unwrap(), 248);
    }

    #[test]
    fn test_parse_binary() {
        assert_eq!(parse_number("0b10").unwrap(), 2);
    }

    #[test]
    fn test_operation_and() {
        let op = Operation::from_str("&").unwrap();
        assert_eq!(op.apply(2, 27), 2);
    }

    #[test]
    fn test_operation_or() {
        let op = Operation::from_str("|").unwrap();
        assert_eq!(op.apply(248, 58), 250);
    }

    #[test]
    fn test_operation_xor() {
        let op = Operation::from_str("^").unwrap();
        assert_eq!(op.apply(12, 32), 44);
    }

    #[test]
    fn test_invalid_number_format() {
        assert!(parse_number("invalid").is_err());
    }

    #[test]
    fn test_invalid_operation() {
        assert!(Operation::from_str("invalid").is_err());
    }
}
