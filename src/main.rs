use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    //Generate and Print The Secret
    let secret_number = rand::thread_rng().gen_range(1..=100);
    

    //Loop for Repeating the game until the user correctly inputs the correct value
    loop {
        println!("Please input your guess.");

        //Create mutable string: guess
        let mut guess = String::new();

        //Capture user input and assign it to guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");


        //lets the user quit if they would like to
        match guess.trim() {
            "quit" => break,
            _ => {},
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");


        // compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!"); 
                break;
            }
        }
    }
}
