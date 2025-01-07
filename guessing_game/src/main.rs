use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    let mut lives = 5;

    loop {
        println!("-----------------------------------------------");
        println!("You have {} lives remaining.", lives);
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small! That means you lose a life.");
                lives -= 1;
            },
            Ordering::Greater => {
                println!("Too big! That means you lose a life.");
                lives -= 1;
            }, 
            Ordering::Equal => {
                println!("You win! You survived with {} lives remaining.", lives);
                break;
            },
        };
    };
}
