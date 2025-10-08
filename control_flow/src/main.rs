fn main() {
    // if expresions
    println!("----------------- If expresions -----------------");
    let number: i32 = 3;

    if number < 5 {
        println!("Number was less then five.")
    } else {
        println!("Number was equels or grater then five.")
    }
    
    let number = if is_even(number) { 5 } else { 10 };

    println!("The value of number now is: {number}");

    // loops
    println!("----------------- loops -----------------");
    let mut counter = 0;

    let result = loop {
        if counter == 10 {
            break counter * 2;
        }
        counter += 1;
    };

    println!("the value of result was: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    for i in a {
        println!("the value is {i}");
    }

    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}

fn is_even(x: i32) -> bool {
    if (x % 2) == 0 { true } else { false }
}
