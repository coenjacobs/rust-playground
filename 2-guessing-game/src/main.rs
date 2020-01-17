use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing game!");

    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please enter your next guess:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line, bye!");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
