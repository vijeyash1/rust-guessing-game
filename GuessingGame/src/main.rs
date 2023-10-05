use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("guess something");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read the input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("You guess a lesser number");
            }
            Ordering::Greater => {
                println!("You guess a greater number");
            }
            Ordering::Equal => {
                println!("You Won");
                break;
            }
        }
    }
}
