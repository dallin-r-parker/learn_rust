extern crate rand;

use std::io; //input output library std = standard Rust
use std::cmp::Ordering;
use rand::Rng;

//cargo doc --open

fn main(){ // this is the entry point into the program "fn" declares a function
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // declaring a variable, immutable by default but mutable because of "mut" in front
        // " :: " syntax shows that what is on the left & right are bound together. Other languages call this static methods
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };


        println!("You guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You WIN!");
                break;
            }
        }
    }
}