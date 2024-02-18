use std::error::Error;

use crate::schema_tokenizer::{SchemaToken, SchemaTokenType};

pub struct TokenSupplier {
    tokens: Vec<SchemaToken>,
    head: usize,
}

impl TokenSupplier {
    pub fn new(tokens: Vec<SchemaToken>) -> Self {
        Self { tokens, head: 0 }
    }

    pub fn peek(&self) -> Option<&SchemaToken> {
        self.tokens.get(self.head)
    }

    pub fn consume(&mut self) -> Result<&SchemaToken, Box<dyn Error>> {
        let result = self.tokens.get(self.head);
        println!("Consumed {:?}", result);
        self.head += 1;
        match result {
            Some(result) => Ok(result),
            None => Err("Token-stream ended unexpectedly!".into()),
        }
    }

    pub fn get(&self) -> Result<&SchemaToken, Box<dyn Error>> {
        match self.tokens.get(self.head) {
            Some(result) => Ok(result),
            None => Err("Token-stream ended unexpectedly!".into()),
        }
    }

    pub fn consume_with_assert(
        &mut self,
        token_type: SchemaTokenType,
    ) -> Result<&SchemaToken, Box<dyn Error>> {
        let current_token = self.consume()?;

        if SchemaTokenType::from(current_token) == token_type {
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
