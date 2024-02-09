use std::error::Error;

use crate::query_tokenizer::{QueryToken, QueryTokenType};

pub struct TokenSupplier {
    tokens: Vec<QueryToken>,
    head: usize,
}

impl TokenSupplier {
    pub fn new(tokens: Vec<QueryToken>) -> Self {
        Self { tokens, head: 0 }
    }

    pub fn peek(&self) -> Option<&QueryToken> {
        self.tokens.get(self.head)
    }

    pub fn consume(&mut self) -> Result<&QueryToken, Box<dyn Error>> {
        let result = self.tokens.get(self.head);
        println!("Consumed {:?}", result);
        self.head += 1;
        match result {
            Some(result) => Ok(result),
            None => Err("Token-stream ended unexpectedly!".into()),
        }
    }

    pub fn get(&self) -> Result<&QueryToken, Box<dyn Error>> {
        match self.tokens.get(self.head) {
            Some(result) => Ok(result),
            None => Err("Token-stream ended unexpectedly!".into()),
        }
    }

    pub fn consume_with_assert(
        &mut self,
        token_type: QueryTokenType,
    ) -> Result<&QueryToken, Box<dyn Error>> {
        let current_token = self.consume()?;

        if QueryTokenType::from(current_token) == token_type {
            Ok(current_token)
        } else {
            Err(format!(
                "Expected a {:?} token but found {:?}!",
                token_type, current_token
            )
            .into())
        }
    }
}
