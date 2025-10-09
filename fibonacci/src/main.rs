/*
    Problem:
        Generate the nth Fibonacci number.
    Formula(s):
        - Base case: F(n) = n, when n = 0 or n = 1
        - Recursive case: F(n) = F(n-1) + F(n-2) for n>1
*/
use std::io;

fn main() {
    loop{
        let mut buf = String::new();

        println!("Enter a number that you want fibonacci for: ");
        io::stdin().read_line(&mut buf).expect("Failed to get fibonacci");
        let buf: u32 = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid positive number.");
                continue;
            }
        };
        println!("The fibonacci for {} is {}", buf, fibonacci(buf));
        break;
    }
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}