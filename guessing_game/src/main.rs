use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut has_guessed_correctly = false;

    while !has_guessed_correctly {
        println!("Please choose a number between 1 and 100!");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Input should be valid!");

        match i32::from_str_radix(&input.trim(), 10) {
            Ok(val) => match val.cmp(&secret_number) {
                std::cmp::Ordering::Less => println!("Too small!"),
                std::cmp::Ordering::Equal => {
                    println!("You won!");
                    has_guessed_correctly = true;
                }
                std::cmp::Ordering::Greater => println!("Too big!"),
            },
            Err(_) => eprintln!("Cannot parse int from '{input}'")
        };
    }
}
