mod boolean_expression;
mod calculated_value;
mod compared_value;
mod constant_calculated_value;
mod insert_statement;
mod select_statement;
mod token_supplier;

use std::error::Error;

use crate::query_parser::select_statement::parse_select_statement;
use crate::query_tokenizer::QueryTokenType;
use crate::rel_alg_ast::RelAlgAST;

pub use self::boolean_expression::parse_boolean_expression;
pub use self::token_supplier::TokenSupplier;

pub fn parse(tokens: &mut TokenSupplier) -> Result<RelAlgAST, Box<dyn Error>> {
    if tokens.get()?.get_type() == QueryTokenType::Select {
        parse_select_statement(tokens)
    } else {
        Err(format!("Invalid start of statement: {:?}", tokens.get()).into())
    }
}
