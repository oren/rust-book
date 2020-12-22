use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // std::string::String - a UTF-8 encoded,
        // std is a Crate. string is a Module. String is a Struct
        let mut guess = String::new();

        // std::io::stdin - function that constructs a new handle to the standard input of the current process.
        // returns Struct std::io::Stdin
        // std is a Crate. io is a Module. stdin is a function
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // parse convert string to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

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
