use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("guess something");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("You guess a lesser number");
        }
        Ordering::Greater => {
            println!("You guess a greater number");
        }
        Ordering::Equal => {
            println!("You Won");
        }
    }
}
