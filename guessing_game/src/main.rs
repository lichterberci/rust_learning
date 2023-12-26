use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut num_guesses = 0;

    loop {
        println!("Please choose a number between 1 and 100!");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Input should be valid!");

        num_guesses += 1;

        match input.trim().parse::<u8>() {
            Ok(val) => match val.cmp(&secret_number) {
                std::cmp::Ordering::Less => println!("Too small!"),
                std::cmp::Ordering::Equal => {
                    println!("You won!");
                    break;
                }
                std::cmp::Ordering::Greater => println!("Too big!"),
            },
            Err(_) => continue
        };
    }

    println!("Congratulations, you won in {num_guesses} guesses!");
}
