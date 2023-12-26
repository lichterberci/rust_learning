use std::{cmp::Ordering, io::stdin};

use rand::{thread_rng, Rng};

fn main() {
    let secret_number = thread_rng().gen_range(1..=100);

    let mut num_guesses = 0;

    loop {
        println!("Please choose a number between 1 and 100!");

        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Input should be valid!");

        num_guesses += 1;

        match input.trim().parse::<u8>() {
            Ok(val) => match val.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Equal => break,
                Ordering::Greater => println!("Too big!"),
            },
            Err(_) => continue,
        };
    }

    println!("Congratulations, you won in {num_guesses} guesses!");
}
