use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system
    // range of numbers: start..=end and is inclusive on the lower and upper bounds
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        

        let mut guess = String::new(); // mutable: allows it to be changed
                        // String::new(); function that returns a new instance of String
                        // :: syntax in the ::new line indicates that new is an associated function of the String type

        // trim deletes whitespace trailing and leading, newlines
        // parse converts a string into another type. In this case, an int

        io::stdin()
            .read_line(&mut guess) // appends to the reference "&" of guess
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // could have been written as io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        println!("You guessed: {guess}");


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
