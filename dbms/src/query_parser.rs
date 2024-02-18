mod boolean_expression;
mod calculated_value;
mod compared_value;
mod constant_calculated_value;
mod delete_statement;
mod insert_statement;
mod select_statement;
mod update_statement;

use std::error::Error;

use crate::query_parser::select_statement::parse_select_statement;
use crate::query_tokenizer::{QueryToken, QueryTokenType};
use crate::rel_alg_ast::RelAlgAST;
use crate::token_supplier::TokenSupplier;

pub use self::boolean_expression::parse_boolean_expression;
use self::delete_statement::parse_delete_statement;
use self::insert_statement::parse_insert_statement;
use self::update_statement::parse_update_statement;

pub fn parse(tokens: &mut TokenSupplier<QueryToken>) -> Result<RelAlgAST, Box<dyn Error>> {
    if tokens.get()?.get_type() == QueryTokenType::Select {
        parse_select_statement(tokens)
    } else if tokens.get()?.get_type() == QueryTokenType::Insert {
        parse_insert_statement(tokens)
    } else if tokens.get()?.get_type() == QueryTokenType::Update {
        parse_update_statement(tokens)
    } else if tokens.get()?.get_type() == QueryTokenType::Delete {
        parse_delete_statement(tokens)
    } else {
        Err(format!("Invalid start of statement: {:?}", tokens.get()).into())
    }
}
