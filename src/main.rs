//libraries
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Welcome to Guess the Number!");
    //variables
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //create an infinite loop that runs until the user input matches the comparison
    //basically loop until users guess is true
    loop {
        println!("The secret number is between 1-100, guess what it is:");
        let mut guess = String::new();
        /*
        1. get standard input function from i/o module
        2. read users input and store it in the guess variable, reference must be mutable
        3. catch errors
        */
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");


        /*
        1. Trim the string so that there is no blank space from newlines
        2. Parse the string into an unsigned 32 bit integer
        3. Since Parse returns an Error or Ok Result, catch the Error case
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //Send response message back to user using {} for template literal
        println!("Your guess is {guess}");
        //check if guessed number matches secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too low!"),
            Ordering::Greater => println!("Your guess is too high"),
            Ordering::Equal => {
                println!("Well done, you guessed correctly!");
                break;
            }
        }
    }
}
