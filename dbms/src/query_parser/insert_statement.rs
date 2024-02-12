use std::error::Error;

use crate::{
    query_tokenizer::{
        ParenthesisType::{Closing, Opening},
        QueryTokenType,
    },
    rel_alg_ast::RelAlgAST,
};

use super::TokenSupplier;

pub fn parse_insert_statement(
    tokens: &mut TokenSupplier,
) -> Result<Option<RelAlgAST>, Box<dyn Error>> {
}

pub fn parse_numerical_expression(
    tokens: &mut TokenSupplier,
) -> Result<Option<RelAlgAST>, Box<dyn Error>> {
    if (tokens.get()?.get_type() == QueryTokenType::Parenthesis(Opening)) {
        tokens.consume()?; // (
        let result = parse_numerical_expression(tokens)?;
        tokens.consume_with_assert(QueryTokenType::Parenthesis(Closing))?;
        Ok(result)
    } else if (tokens.get()?.get_type() == QueryTokenType::Value(())) {
        let value = tokens.consume()?; // value
    }
}
