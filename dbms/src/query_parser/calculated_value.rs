use std::error::Error;

use crate::{
    query_tokenizer::{
        ComparisonOperatorType, LogicalOperatorType, NumericalOperatorType, ParenthesisType,
        QueryToken, QueryTokenType, Value,
    },
    rel_alg_ast::{CalculatedValue, Identifier},
};

use super::TokenSupplier;

#[derive(PartialEq, Debug)]
enum CalculatedValueJoin {
    And,
    Or,
    NumericalOperator(NumericalOperatorType),
    ComparisonOperator(ComparisonOperatorType),
}

pub fn parse_calculated_value(
    tokens: &mut TokenSupplier,
) -> Result<CalculatedValue, Box<dyn Error>> {
    if let QueryToken::Value(_) = tokens.get()? {
        let token = tokens.consume()?;

        let left_subtree = CalculatedValue::Value(match token {
            QueryToken::Value(value) => value.clone(),
            _ => panic!("Error during parsing! Expected a value here!"),
        });

        let subtree = combine_calculated_value_expression_with_prime(tokens, left_subtree)?;

        return Ok(subtree);
    } else if let QueryToken::Identifier(_) = tokens.get()? {
        let first_identifier = tokens.consume()?;

        let first_id_string = match first_identifier {
            QueryToken::Identifier(name) => String::from(name),
            _ => panic!("Error during parsing! Expected an identifier!"),
        };

        if tokens.get()?.get_type() != QueryTokenType::Dot {
            return Ok(CalculatedValue::Identifier(Identifier::AttributeName(
                first_id_string,
            )));
        }

        tokens.consume()?; // dot

        let second_identifier = tokens.consume_with_assert(QueryTokenType::Identifier)?;

        let second_id_string = match second_identifier {
            QueryToken::Identifier(name) => String::from(name),
            _ => panic!("Error during parsing! Expected an identifier!"),
        };

        let left_subtree = CalculatedValue::Identifier(Identifier::QualifiedAttributeName(
            first_id_string,
            second_id_string,
        ));

        let subtree = combine_calculated_value_expression_with_prime(tokens, left_subtree)?;

        return Ok(subtree);
    } else if let QueryToken::NumericalOperator(NumericalOperatorType::Sub) = tokens.get()? {
        tokens.consume()?; // -

        let left_subtree = parse_calculated_value(tokens)?;

        let subtree = combine_calculated_value_expression_with_prime(tokens, left_subtree)?;

        return Ok(CalculatedValue::NumericalOperation(
            NumericalOperatorType::Mult,
            Box::new(CalculatedValue::Value(Value::Integer(-1))),
            Box::new(subtree),
        ));
    } else if let QueryToken::LogicalOperator(LogicalOperatorType::Not) = tokens.get()? {
        tokens.consume()?; // -

        let left_subtree = parse_calculated_value(tokens)?;

        let subtree = combine_calculated_value_expression_with_prime(tokens, left_subtree)?;

        return Ok(CalculatedValue::Not(Box::new(subtree)));
    } else if let QueryToken::Parenthesis(ParenthesisType::Opening) = tokens.get()? {
        tokens.consume()?;

        let left_subtree = parse_calculated_value(tokens)?;

        let subtree = combine_calculated_value_expression_with_prime(tokens, left_subtree)?;

        tokens.consume_with_assert(QueryTokenType::Parenthesis(ParenthesisType::Closing))?;

        Ok(subtree)
    } else {
        Err("Unexpected token during parsing of calculated value!".into())
    }
}

fn parse_calculated_value_prime(
    tokens: &mut TokenSupplier,
) -> Result<Option<(CalculatedValueJoin, CalculatedValue)>, Box<dyn Error>> {
    if let Some(token) = tokens.peek().map(|x| x.get_type()) {
        if let QueryTokenType::NumericalOperator(operator_type) = token {
            tokens.consume()?; // + | - | * | /

            let left_subtree = parse_calculated_value(tokens)?;

            let subtree = combine_calculated_value_expression_with_prime(tokens, left_subtree)?;

            Ok(Some((
                CalculatedValueJoin::NumericalOperator(operator_type),
                subtree,
            )))
        } else if let QueryTokenType::ComparisonOperator(operator_type) = token {
            tokens.consume()?; // == | <= | ...

            let left_subtree = parse_calculated_value(tokens)?;

            let subtree = combine_calculated_value_expression_with_prime(tokens, left_subtree)?;

            Ok(Some((
                CalculatedValueJoin::ComparisonOperator(operator_type),
                subtree,
            )))
        } else if let QueryTokenType::LogicalOperator(LogicalOperatorType::And) = token {
            tokens.consume()?; // == | <= | ...

            let left_subtree = parse_calculated_value(tokens)?;

            let subtree = combine_calculated_value_expression_with_prime(tokens, left_subtree)?;

            Ok(Some((CalculatedValueJoin::And, subtree)))
        } else if let QueryTokenType::LogicalOperator(LogicalOperatorType::Or) = token {
            tokens.consume()?; // == | <= | ...

            let left_subtree = parse_calculated_value(tokens)?;

            let subtree = combine_calculated_value_expression_with_prime(tokens, left_subtree)?;

            Ok(Some((CalculatedValueJoin::Or, subtree)))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn combine_calculated_value_expression_with_prime(
    tokens: &mut TokenSupplier,
    left_subtree: CalculatedValue,
) -> Result<CalculatedValue, Box<dyn Error>> {
    let prime_expr = parse_calculated_value_prime(tokens)?;

    if let Some((join_type, right_subtree)) = prime_expr {
        match join_type {
            CalculatedValueJoin::And => Ok(CalculatedValue::And(
                Box::new(left_subtree),
                Box::new(right_subtree),
            )),
            CalculatedValueJoin::Or => Ok(CalculatedValue::Or(
                Box::new(left_subtree),
                Box::new(right_subtree),
            )),
            CalculatedValueJoin::NumericalOperator(operator_type) => {
                Ok(CalculatedValue::NumericalOperation(
                    operator_type,
                    Box::new(left_subtree),
                    Box::new(right_subtree),
                ))
            }
            CalculatedValueJoin::ComparisonOperator(operator_type) => {
                Ok(CalculatedValue::Comparison(
                    operator_type,
                    Box::new(left_subtree),
                    Box::new(right_subtree),
                ))
            }
        }
    } else {
        Ok(left_subtree)
    }
}
