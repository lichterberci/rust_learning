use std::process;

use minigrep::{Config, run};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error {err}!");
        process::exit(1);
    }
}
