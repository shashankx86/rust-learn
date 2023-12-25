use std::io;
use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let mut rng = thread_rng();
    let genn: i32 = rng.gen_range(1..100); // gen random value between 1 - 100

    loop {
        println!("Please input you guess.");

        let mut guess = String::new(); 
        io::stdin()
            .read_line (&mut guess).expect("Failed to read line");  // input

        let guess: i32 = match guess.trim().parse() {  // 2 things: (1) converting the String type to i32, and (2) in a situation where I want to change the value of 'guess,' I can use this for the change, as the initial 'guess' needs to be mutable in order to accept input.
            Ok(num) => num,
            Err(_) => continue, 
        }; // Note that the 'guess' variable defined previously is different from the 'guess' variable here; a new variable is being created.
        
    //    let genn: i32 = guess;

        match guess.cmp(&genn) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }

        println!("rand value {genn} and your value {guess}");
    }
}
