use std::error::Error;

use crate::{
    query_tokenizer::{NumericalOperatorType, QueryToken, QueryTokenType},
    rel_alg_ast::RelAlgAST,
};

use super::TokenSupplier;

pub fn parse_projection(tokens: &mut TokenSupplier) -> Result<Option<Vec<String>>, Box<dyn Error>> {
    if tokens.get()?.get_type() == QueryTokenType::NumericalOperator(NumericalOperatorType::Mult) {
        tokens.consume()?; // *

        return Ok(None);
    }

    let identifier = match tokens.consume_with_assert(QueryTokenType::Identifier)? {
        QueryToken::Identifier(identifier) => String::from(identifier),
        _ => return Err("Identifier expected!".into()),
    };

    if tokens.get()?.get_type() == QueryTokenType::Comma {
        tokens.consume()?; // ,

        let Some(rest_of_selection) = parse_projection(tokens)? else {
            return Err("Identifier expected after comma when selecting columns!".into());
        };

        Ok(Some(vec![vec![identifier], rest_of_selection].concat()))
    } else if tokens.get()?.get_type() == QueryTokenType::Dot {
        tokens.consume()?; // .

        let second_identifier = match tokens.consume_with_assert(QueryTokenType::Identifier)? {
            QueryToken::Identifier(identifier) => String::from(identifier),
            _ => return Err("Identifier expected!".into()),
        };

        let identifier = String::from(format!("{identifier}.{second_identifier}"));

        if tokens.get()?.get_type() == QueryTokenType::Comma {
            tokens.consume()?; // ,

            let Some(rest_of_selection) = parse_projection(tokens)? else {
                return Err("Identifier expected after comma when selecting columns!".into());
            };

            Ok(Some(vec![vec![identifier], rest_of_selection].concat()))
        } else {
            Ok(Some(vec![identifier]))
        }
    } else {
        Ok(Some(vec![identifier]))
    }
}

pub fn parse_source_tables(tokens: &mut TokenSupplier) -> Result<RelAlgAST, Box<dyn Error>> {
    let identifier = match tokens.consume_with_assert(QueryTokenType::Identifier)? {
        QueryToken::Identifier(identifier) => String::from(identifier),
        _ => return Err("Identifier expected!".into()),
    };

    if tokens.get()?.get_type() == QueryTokenType::Comma {
        tokens.consume()?; // ,

        let rest_of_selection = parse_source_tables(tokens)?;

        Ok(RelAlgAST::CartesianProduct(
            Box::new(RelAlgAST::Relation(identifier)),
            Box::new(rest_of_selection),
        ))
    } else {
        Ok(RelAlgAST::Relation(identifier))
    }
}
