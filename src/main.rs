use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){

    // First generate a random number
    // Ask the user to enter a number
    // Check the random number against the number entered by the user
    // Based on the ordering print whats necessary

    let secret_number:i32 = rand::thread_rng().gen_range(0..11);
    println!("Please enter a number (between 0 and 10) : ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Unable to read input from stdin");
    let guess:i32 = guess.trim().parse().expect("Not a number");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater=>println!("Too great!"),
        Ordering::Equal=>println!("You guessed it correct")
    }
}