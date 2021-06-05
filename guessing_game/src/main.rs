use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("number!");

    // For more detail information about rand crate, Run `cargo doc --open`
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("secret number: {}", secret_number);

    loop {

        println!("input.");

        // Rust supports type inference
        // so we don't need to specify type of variables
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("error");

        // Rust supports variable shadowing
        // so we can write code similar to python (type change of a variable)
        // parse() method can inference type to parse by type of variable (u32)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("input: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("input is lower"),
            Ordering::Greater => println!("input is greater"),
            Ordering::Equal => {
                println!("correct!");
                break;
            }
        }
    }
}
