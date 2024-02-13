use std::error::Error;

use crate::{
    query_tokenizer::{NumericalOperatorType, ParenthesisType, QueryToken, QueryTokenType},
    rel_alg_ast::{ComparedValue, Identifier},
};

use super::token_supplier::TokenSupplier;

pub fn parse_compared_value(tokens: &mut TokenSupplier) -> Result<ComparedValue, Box<dyn Error>> {
    if tokens.get()?.get_type() == QueryTokenType::Identifier {
        let first_identifier = tokens.consume()?;

        let first_id_string = match first_identifier {
            QueryToken::Identifier(name) => String::from(name),
            _ => panic!("Error during parsing! Expected an identifier!"),
        };

        if tokens.get()?.get_type() != QueryTokenType::Dot {
            return Ok(ComparedValue::Identifier(Identifier::AttributeName(
                first_id_string,
            )));
        }

        tokens.consume()?; // dot

        let second_identifier = tokens.consume_with_assert(QueryTokenType::Identifier)?;

        let second_id_string = match second_identifier {
            QueryToken::Identifier(name) => String::from(name),
            _ => panic!("Error during parsing! Expected an identifier!"),
        };

        let left_subtree = ComparedValue::Identifier(Identifier::QualifiedAttributeName(
            first_id_string,
            second_id_string,
        ));

        let subtree = combine_compared_value_expression_with_prime(tokens, left_subtree)?;

        return Ok(subtree);
    } else if let QueryToken::Value(_) = tokens.get()? {
        let token = tokens.consume()?;

        let left_subtree = ComparedValue::Value(match token {
            QueryToken::Value(value) => value.clone(),
            _ => panic!("Error during parsing! Expected a value here!"),
        });

        let subtree = combine_compared_value_expression_with_prime(tokens, left_subtree)?;

        return Ok(subtree);
    } else if tokens.get()?.get_type() == QueryTokenType::Parenthesis(ParenthesisType::Opening) {
        tokens.consume()?; // '('

        let left_subtree = parse_compared_value(tokens)?;

        let subtree = combine_compared_value_expression_with_prime(tokens, left_subtree)?;

        tokens.consume_with_assert(QueryTokenType::Parenthesis(ParenthesisType::Closing))?;

        Ok(subtree)
    } else {
        Err(format!("Expected Identifier or Value but got {:?}", tokens.get()?).into())
    }
}

fn parse_compared_value_prime(
    tokens: &mut TokenSupplier,
) -> Result<Option<(NumericalOperatorType, ComparedValue)>, Box<dyn Error>> {
    if let Some(token) = tokens.peek().map(|x| x.get_type()) {
        if let QueryTokenType::NumericalOperator(operator_type) = token {
            tokens.consume()?; // + | - | * | /

            let left_subtree = parse_compared_value(tokens)?;

            let subtree = combine_compared_value_expression_with_prime(tokens, left_subtree)?;

            Ok(Some((operator_type, subtree)))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn combine_compared_value_expression_with_prime(
    tokens: &mut TokenSupplier,
    left_subtree: ComparedValue,
) -> Result<ComparedValue, Box<dyn Error>> {
    let prime_expr = parse_compared_value_prime(tokens)?;

    if let Some((operator_type, right_subtree)) = prime_expr {
        Ok(ComparedValue::Composite(
            operator_type,
            Box::new(left_subtree),
            Box::new(right_subtree),
        ))
    } else {
        Ok(left_subtree)
    }
}
