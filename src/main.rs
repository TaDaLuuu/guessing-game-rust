use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Please input your guess.");
    println!("The secret number is: {}", secret_number);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        let guess: u32 = guess.trim().parse().expect("Please type a number !!!");
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win !!!");
                break;
            },
            Ordering::Greater => println!("Too big !"),
            Ordering::Less => println!("Too small !")
        }
    }
}
