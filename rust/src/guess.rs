use rand::Rng;
use std::cmp::Ordering::*;
use std::io;

fn main() {
    println!("Guess something!");
    let n = rand::thread_rng().gen_range(1..=5);
    println!("n!: {}", n);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Ok(n) => n,
            Err(..) => {
                continue;
            }
        };

        println!("guess is: {}", guess);

        match guess.cmp(&n) {
            Less => println!("Guess is lower than n"),
            Greater => println!("Guess is grester than n"),
            Equal => {
                println!("WIN!");
                break;
            }
        }
    }
}
