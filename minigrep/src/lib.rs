use std::{error::Error, fs, process};

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            Err("Not enough arguments!")
        } else {
            Ok(Self {
                query: args[1].clone(),
                file_path: args[2].clone(),
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    search(&config.query, &contents)
        .iter()
        .for_each(|result| println!("{}", result));

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = r#"
            Rust:
            Safe fast and productive.
            Pick three.
        "#;

        assert_eq!(vec!["Safe fast and productive."], search(query, contents))
    }

    #[test]
    fn case_sensitive() {

        let query = "duct";
        let contents = r#"
            Rust:
            Safe fast and productive.
            Pick three.
        "#;

        assert_eq!(vec!["Safe fast and productive."], search(query, contents))
    }
}
