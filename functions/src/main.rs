fn main() {
    my_function(6);
    print_labeled_measurement(5, 'h');

    let sqrt = sqrt(2);
    println!("The sqrt of 2 is: {sqrt}")

}

fn my_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit: char) {
    println!("The measurment is: {value}{unit}")
}

fn sqrt(x: i32) -> i32 {
    x * x
}