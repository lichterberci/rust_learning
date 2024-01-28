use std::{error::Error, marker::Copy};

use regex::{Match, Regex};

pub struct Tokenizer<Token, TokenType>
where
    TokenType: Copy,
{
    grammar: Vec<(Regex, Option<TokenType>)>,
    matching_function: Box<dyn for<'b> Fn(&Match<'b>, &TokenType) -> Result<Token, Box<dyn Error>>>,
    target_group_name: &'static str,
}

impl<Token, TokenType> Tokenizer<Token, TokenType>
where
    TokenType: Copy,
{
    pub fn build(
        grammar: &Vec<(String, Option<TokenType>)>,
        matching_function: Box<
            dyn for<'b> Fn(&Match<'b>, &TokenType) -> Result<Token, Box<dyn Error>>,
        >,
        target_group_name: &'static str,
    ) -> Self {
        Self {
            grammar: grammar
                .iter()
                .map(|(pattern, symbol_type)| {
                    (
                        Regex::new(pattern).expect("Pattern should be valid regex!"),
                        symbol_type.clone().to_owned(),
                    )
                })
                .collect(),
            matching_function,
            target_group_name,
        }
    }

    pub fn tokenize(&self, input: &str) -> Result<Vec<Token>, Box<dyn Error>> {
        let input = input.to_lowercase();
        let input = input.trim().to_owned() + " "; // this space is crucial for the regex patterns to match

        let mut head_index = 0;
        let mut output = Vec::new();

        'token_loop: while head_index < input.len() {
            let input = &input[head_index..];

            for (pattern, inferred_type) in &self.grammar {
                let captures = pattern.captures(input);

                // no matching here
                if captures.is_none() {
                    continue;
                }

                let captures = captures.expect("There should be a capture here!");

                let captured_group_of_token = captures
                    .name(self.target_group_name)
                    .expect("There should be a capture here!");

                // this is a separator or comment
                if inferred_type.is_none() {
                    head_index += captured_group_of_token.as_str().len();
                    continue 'token_loop;
                }

                let extracted_symbol: Token = match inferred_type {
                    Some(inferred_type) => {
                        (self.matching_function)(&captured_group_of_token, inferred_type)?
                    }
                    None => return Err("Inferred type should not be None here!".into()),
                };

                head_index += captured_group_of_token.as_str().len();

                output.push(extracted_symbol);

                continue 'token_loop;
            }

            return Err(format!("Sequence not recognized: {input}").into());
        }

        Ok(output)
    }
}
