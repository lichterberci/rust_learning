use core::panic;
use std::{error::Error, fmt::Debug, str::pattern::Pattern};

use crate::{
    query_tokenizer::{self, ParenthesisType, QueryToken, QueryTokenType, Value, ValueType},
    rel_alg_ast,
};

struct TokenSupplier {
    tokens: Vec<QueryToken>,
    head: usize,
}

impl TokenSupplier {
    fn new(tokens: Vec<QueryToken>) -> Self {
        Self { tokens, head: 0 }
    }

    fn peek_next(&self) -> Option<&QueryToken> {
        self.tokens.get(self.head + 1)
    }

    fn consume(&mut self) -> Result<&QueryToken, Box<dyn Error>> {
        let result = self.tokens.get(self.head);
        self.head += 1;
        match result {
            Some(result) => Ok(result),
            None => Err("Token-stream ended unexpectedly!".into()),
        }
    }

    fn get(&self) -> Result<&QueryToken, Box<dyn Error>> {
        match self.tokens.get(self.head) {
            Some(result) => Ok(result),
            None => Err("Token-stream ended unexpectedly!".into()),
        }
    }

    fn get_with_assert(&self, token_type: QueryTokenType) -> Result<&QueryToken, Box<dyn Error>> {
        let current_token = self.get()?;

        if QueryTokenType::from(*current_token) == token_type {
            Ok(current_token)
        } else {
            Err(format!(
                "Expected a {:?} token after {:?} but found {:?}!",
                token_type,
                self.tokens.get(self.head.wrapping_sub(1)),
                current_token
            )
            .into())
        }
    }

    fn consume_with_assert(
        &mut self,
        token_type: QueryTokenType,
    ) -> Result<&QueryToken, Box<dyn Error>> {
        let current_token = self.consume()?;

        if QueryTokenType::from(*current_token) == token_type {
            Ok(current_token)
        } else {
            Err(format!(
                "Expected a {:?} token after {:?} but found {:?}!",
                token_type,
                self.tokens.get(self.head.wrapping_sub(1)),
                current_token
            )
            .into())
        }
    }
}

pub fn parse_boolean_expression(tokens: &mut TokenSupplier) -> Result<(), Box<dyn Error>> {
    if tokens.get()?.get_type() == QueryTokenType::Parenthesis(ParenthesisType::Opening) {
        Ok({
            tokens.consume_with_assert(QueryTokenType::Parenthesis(ParenthesisType::Opening));
            parse_boolean_expression_body(tokens)?;
            tokens.consume_with_assert(QueryTokenType::Parenthesis(ParenthesisType::Closing));
        })
    } else {
        Ok({
            parse_boolean_expression_body(tokens)?;
        })
    }
}

fn parse_boolean_expression_body(tokens: &mut TokenSupplier) -> Result<(), Box<dyn Error>> {
    Ok(())
}
