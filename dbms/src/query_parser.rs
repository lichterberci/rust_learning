use std::{error::Error, fmt::format, ops::Not};

use crate::query_tokenizer::{
    LogicalOperatorType, ParenthesisType, QueryToken, QueryTokenType, ValueType::Integer,
};

pub struct TokenSupplier {
    tokens: Vec<QueryToken>,
    head: usize,
}

impl TokenSupplier {
    pub fn new(tokens: Vec<QueryToken>) -> Self {
        Self { tokens, head: 0 }
    }

    fn peek_next(&self) -> Option<&QueryToken> {
        self.tokens.get(self.head + 1)
    }

    fn consume(&mut self) -> Result<&QueryToken, Box<dyn Error>> {
        let result = self.tokens.get(self.head);
        println!("Consumed: {:#?}", result);
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

    fn consume_with_assert(
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

pub fn parse_boolean_expression(tokens: &mut TokenSupplier) -> Result<(), Box<dyn Error>> {
    if tokens.get()?.get_type() == QueryTokenType::Parenthesis(ParenthesisType::Opening) {
        tokens.consume()?;

        parse_boolean_expression_body(tokens)?;

        tokens.consume_with_assert(QueryTokenType::Parenthesis(ParenthesisType::Closing))?;
    } else {
        parse_boolean_expression_body(tokens)?;
    }

    parse_boolean_expression_prime(tokens)?;

    Ok(())
}

fn parse_boolean_expression_body(tokens: &mut TokenSupplier) -> Result<(), Box<dyn Error>> {
    if tokens.get()?.get_type() == QueryTokenType::LogicalOperator(LogicalOperatorType::Not) {
        tokens.consume()?;
        parse_boolean_expression(tokens)?;
    } else {
        parse_compared_value(tokens)?;

        if let QueryTokenType::ComparisonOperator(comparison_operator_type) =
            tokens.get()?.get_type()
        {
            tokens.consume()?;
        } else {
            return Err(format!(
                "Expected a comparison operator but found {:?}",
                tokens.get()?
            )
            .into());
        }

        parse_compared_value(tokens)?;
    }

    parse_boolean_expression_prime(tokens)?;

    Ok(())
}

fn parse_compared_value(tokens: &mut TokenSupplier) -> Result<(), Box<dyn Error>> {
    if tokens.get()?.get_type() == QueryTokenType::Identifier {
        Ok({
            tokens.consume()?;
            if tokens.get()?.get_type() == QueryTokenType::Dot {
                tokens.consume()?;
                tokens.consume_with_assert(QueryTokenType::Identifier)?;
            }
        })
    } else if let QueryTokenType::Value(_) = tokens.get()?.get_type() {
        Ok({
            tokens.consume()?;
        })
    } else {
        Err(format!("Expected Identifier or Value but got {:?}", tokens.get()?).into())
    }
}

fn parse_boolean_expression_prime(tokens: &mut TokenSupplier) -> Result<(), Box<dyn Error>> {
    if let QueryTokenType::LogicalOperator(logical_operator_type) = tokens.get()?.get_type() {
        match logical_operator_type {
            LogicalOperatorType::Or | LogicalOperatorType::And => Ok({
                tokens.consume()?;
                parse_boolean_expression(tokens)?;
            }),
            LogicalOperatorType::Not => Err(format!(
                "Expected an And or Or logical operator but found {:?} instead!",
                tokens.get()?
            )
            .into()),
        }
    } else {
        Ok(())
    }
}
