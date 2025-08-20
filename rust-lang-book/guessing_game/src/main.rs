use std::io; // Importing the IO Library
use rand::Rng; // Random Number Generation 
use std::cmp::Ordering; // For comparing

fn main(){
    println!("Guess the Number!");

    let secret_number = rand::rng().random_range(1..=100); // Random Number Generation between 1 to 100

    //println!("The Secret Number is {secret_number}");
    loop {
    println!("Please input your guess.");

    let mut guess = String::new(); // Mutable variable

    io::stdin()
        .read_line(&mut guess) // Take the User Input
        .expect("Failed to Read Line"); // Handling Potential Faliure
        // Without except The Program will compile but the compiler will give warning
    
    let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // To make sure the Number is Integer

    println!("You Guessed : {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("!Too Small"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => {
            println!("You Win!");
            break;
        }
    }
}

}