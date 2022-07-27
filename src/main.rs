use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number was {}", secret_number);

    loop {
        println!("  Please, enter your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Couldn't read line");

        let guess: u32 = guess.trim().parse()
            .expect("Couldn't parse guess to number");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Yay! You won");
                break;
            },
            Ordering::Greater => println!("Your number is greater"),
            Ordering::Less => println!("Your number is less")
        }
    }
}
