use std::io;
use std::cmp::Ordering;
use rand::Rng; // Add this line to your code to bring the rand crate into scope

fn main() {
    println!("Guess the number!");

    // Generate a random secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is : {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Read a line from standard input (the console) and store it in the 'guess' variable
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // Parse the string 'guess' into an unsigned 32-bit integer (u32)
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Print the user's guessed value using string interpolation
        println!("You guessed: {}", guess);

        // Compare the user's guess with the secret number using a match statement
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
   }
}
