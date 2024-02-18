use std::error::Error;

use crate::{
    query_tokenizer::{
        ComparisonOperatorType, LogicalOperatorType, NumericalOperatorType, ParenthesisType,
        QueryToken, QueryTokenType, Value,
    },
    rel_alg_ast::CalculatedValue,
};

use super::TokenSupplier;

#[derive(PartialEq, Debug)]
enum ConstantCalculatedValueJoin {
    And,
    Or,
    NumericalOperator(NumericalOperatorType),
    ComparisonOperator(ComparisonOperatorType),
}

pub fn parse_constant_calculated_value(
    tokens: &mut TokenSupplier,
) -> Result<CalculatedValue, Box<dyn Error>> {
    if let QueryToken::Value(_) = tokens.get()? {
        let token = tokens.consume()?;

        let left_subtree = CalculatedValue::Value(match token {
            QueryToken::Value(value) => value.clone(),
            _ => panic!("Error during parsing! Expected a value here!"),
        });

        let subtree =
            combine_constant_calculated_value_expression_with_prime(tokens, left_subtree)?;

        return Ok(subtree);
    } else if let QueryToken::NumericalOperator(NumericalOperatorType::Sub) = tokens.get()? {
        tokens.consume()?; // -

        let left_subtree = parse_constant_calculated_value(tokens)?;

        let subtree =
            combine_constant_calculated_value_expression_with_prime(tokens, left_subtree)?;

        return Ok(CalculatedValue::NumericalOperation(
            NumericalOperatorType::Mult,
            Box::new(CalculatedValue::Value(Value::Integer(-1))),
            Box::new(subtree),
        ));
    } else if let QueryToken::LogicalOperator(LogicalOperatorType::Not) = tokens.get()? {
        tokens.consume()?; // -

        let left_subtree = parse_constant_calculated_value(tokens)?;

        let subtree =
            combine_constant_calculated_value_expression_with_prime(tokens, left_subtree)?;

        return Ok(CalculatedValue::Not(Box::new(subtree)));
    } else if let QueryToken::Parenthesis(ParenthesisType::Opening) = tokens.get()? {
        tokens.consume()?;

        let left_subtree = parse_constant_calculated_value(tokens)?;

        let subtree =
            combine_constant_calculated_value_expression_with_prime(tokens, left_subtree)?;

        tokens.consume_with_assert(QueryTokenType::Parenthesis(ParenthesisType::Closing))?;

        Ok(subtree)
    } else {
        Err("Unexpected token during parsing of calculated value!".into())
    }
}

fn parse_constant_calculated_value_prime(
    tokens: &mut TokenSupplier,
) -> Result<Option<(ConstantCalculatedValueJoin, CalculatedValue)>, Box<dyn Error>> {
    if let Some(token) = tokens.peek().map(|x| x.get_type()) {
        if let QueryTokenType::NumericalOperator(operator_type) = token {
            tokens.consume()?; // + | - | * | /

            let left_subtree = parse_constant_calculated_value(tokens)?;

            let subtree =
                combine_constant_calculated_value_expression_with_prime(tokens, left_subtree)?;

            Ok(Some((
                ConstantCalculatedValueJoin::NumericalOperator(operator_type),
                subtree,
            )))
        } else if let QueryTokenType::ComparisonOperator(operator_type) = token {
            tokens.consume()?; // == | <= | ...

            let left_subtree = parse_constant_calculated_value(tokens)?;

            let subtree =
                combine_constant_calculated_value_expression_with_prime(tokens, left_subtree)?;

            Ok(Some((
                ConstantCalculatedValueJoin::ComparisonOperator(operator_type),
                subtree,
            )))
        } else if let QueryTokenType::LogicalOperator(LogicalOperatorType::And) = token {
            tokens.consume()?; // == | <= | ...

            let left_subtree = parse_constant_calculated_value(tokens)?;

            let subtree =
                combine_constant_calculated_value_expression_with_prime(tokens, left_subtree)?;

            Ok(Some((ConstantCalculatedValueJoin::And, subtree)))
        } else if let QueryTokenType::LogicalOperator(LogicalOperatorType::Or) = token {
            tokens.consume()?; // == | <= | ...

            let left_subtree = parse_constant_calculated_value(tokens)?;

            let subtree =
                combine_constant_calculated_value_expression_with_prime(tokens, left_subtree)?;

            Ok(Some((ConstantCalculatedValueJoin::Or, subtree)))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn combine_constant_calculated_value_expression_with_prime(
    tokens: &mut TokenSupplier,
    left_subtree: CalculatedValue,
) -> Result<CalculatedValue, Box<dyn Error>> {
    let prime_expr = parse_constant_calculated_value_prime(tokens)?;

    if let Some((join_type, right_subtree)) = prime_expr {
        match join_type {
            ConstantCalculatedValueJoin::And => Ok(CalculatedValue::And(
                Box::new(left_subtree),
                Box::new(right_subtree),
            )),
            ConstantCalculatedValueJoin::Or => Ok(CalculatedValue::Or(
                Box::new(left_subtree),
                Box::new(right_subtree),
            )),
            ConstantCalculatedValueJoin::NumericalOperator(operator_type) => {
                Ok(CalculatedValue::NumericalOperation(
                    operator_type,
                    Box::new(left_subtree),
                    Box::new(right_subtree),
                ))
            }
            ConstantCalculatedValueJoin::ComparisonOperator(operator_type) => {
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