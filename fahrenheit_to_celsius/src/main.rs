/*
    Problem:
        Convert temperatures between Fahrenheit and Celsius.
    Formula:
        (°F − 32) × 5/9 = °C
        (°C × 9/5) + 32 = °F
*/
use std::io::{self, Write};

fn main() {
    println!("---------- Convert temperatures between Fahrenheit and Celsius. ----------\n");

    println!("1. F → C \n2. C → F");
    let conversion_type: u8 = get_input("Select from [1, 2]: ", "Please select a conversion type");

    if conversion_type == 1 {
        let fahrenheit: f32 = get_input("Enter the value for °F: ", "Please enter °F.");

        let celsius = (fahrenheit - 32.0) * 5.0/9.0;
        println!("{fahrenheit:.2}°F = {celsius:.2}°C")
    } else if conversion_type == 2 {
        let celsius: f32 = get_input("Enter the value for °C: ", "Please enter °C.");

        let fahrenheit = (celsius * 9.0/5.0) + 32.0;
        println!("{celsius:.2}°C = {fahrenheit:.2}°F")
    } else {
        println!("Enter a valid conversion_type.")
    }

}

fn get_input<T>(prompt: &str, err_msg: &str) -> T 
where 
    T: std::str::FromStr, 
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{   
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("failed to flush stdout");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("\nfailed to read line");

        match buf.trim().parse::<T>() {
            Ok(val) => return val,
            Err(_) => println!("\n{err_msg}"),
        }
    }
}