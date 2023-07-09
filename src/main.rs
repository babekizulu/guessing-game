//libraries
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn new_guess(gn: u8, sn: u8) {
    //guess 1
        println!("{gn}: ");
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
        let guess: u8 = guess.trim().parse().expect("Please enter a valid number");
        //Send response message back to user using {} for template literal
        println!("Guess number {gn}: {guess}");
        //guess
        match guess.cmp(&sn) {
        Ordering::Less => println!("Your guess is too low!"),
        Ordering::Greater => println!("Your guess is too high"),
        Ordering::Equal => {
            println!("Correct!");
        }
    }
}
fn main() {
    println!("Welcome to Guess the Number!");
    //variables
    let sn_1: u8 = rand::thread_rng().gen_range(1..=100);
    let sn_2: u8 = rand::thread_rng().gen_range(1..=100);
    let sn_3: u8 = rand::thread_rng().gen_range(1..=100);
    let gn_tup: (u8, u8, u8) = (1, 2, 3);
    println!("Guess the 3 secret numbers between 1-100");
    //guess 1
    new_guess(gn_tup.0, sn_1);
    //guess 2
    new_guess(gn_tup.1, sn_2);
    //guess 3
    new_guess(gn_tup.2, sn_3);
}
