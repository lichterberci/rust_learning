mod boolean_expression;
mod compared_value;
mod select_statement;
mod token_supplier;

use std::error::Error;

use crate::query_parser::select_statement::parse_source_tables;
use crate::query_tokenizer::QueryTokenType;
use crate::rel_alg_ast::RelAlgAST;

pub use self::boolean_expression::parse_boolean_expression;
use self::select_statement::parse_projection;
pub use self::token_supplier::TokenSupplier;

pub fn parse(tokens: &mut TokenSupplier) -> Result<RelAlgAST, Box<dyn Error>> {
    if tokens.get()?.get_type() == QueryTokenType::Select {
        tokens.consume()?; // select

        let projection = parse_projection(tokens)?;

        tokens.consume_with_assert(QueryTokenType::From)?;

        let source_relations = parse_source_tables(tokens)?;

        if tokens.get()?.get_type() == QueryTokenType::Where {
            tokens.consume()?; // where

            let boolean_expression = parse_boolean_expression(tokens)?;

            tokens.consume_with_assert(QueryTokenType::Semicolon)?;

            if let Some(projected_attributes) = projection {
                return Ok(RelAlgAST::Projection(
                    Box::new(RelAlgAST::Selection(
                        Box::new(source_relations),
                        boolean_expression,
                    )),
                    projected_attributes,
                ));
            } else {
                return Ok(RelAlgAST::Selection(
                    Box::new(source_relations),
                    boolean_expression,
                ));
            }
        } else {
            tokens.consume_with_assert(QueryTokenType::Semicolon)?;

            if let Some(projected_attributes) = projection {
                return Ok(RelAlgAST::Projection(
                    Box::new(source_relations),
                    projected_attributes,
                ));
            } else {
                return Ok(source_relations);
            }
        }
    } else {
        Err(format!("Invalid start of statement: {:?}", tokens.get()).into())
    }
}
