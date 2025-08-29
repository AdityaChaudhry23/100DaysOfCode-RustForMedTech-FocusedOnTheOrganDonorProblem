fn main() {
    // Variables in Rust
    // Immutable Variables
    //let x = 5; // This is an immutable Variable 
    //println!("The value of x is: {x}");
    //x = 6; // The Compiler will error because x is Immutable
    //println!("The value of x is: {x}");

    // To make this mutable add the mut keyword
    let mut x =5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");
    // Constant Variable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Constants may be set only to a constant expression,
    // not the result of a value that could only be computed at runtime.
    println!("{THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2; // The value of y in this scope is different than the global value of y
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    //Numeric Operations
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    // Control Flow
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // Functions
    let z = plus_one(5);

    println!("The value of z is: {z}");

    print_labeled_measurement(5, 'h');
}

// Adding the semi colon does not return the value
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}