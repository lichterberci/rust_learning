use std::error::Error;

use crate::{
    query_tokenizer::{LogicalOperatorType, ParenthesisType, QueryToken, QueryTokenType},
    rel_alg_ast::SelectionExpression,
    token_supplier::TokenSupplier,
};

use super::compared_value::parse_compared_value;

pub fn parse_boolean_expression(
    tokens: &mut TokenSupplier<QueryToken>,
) -> Result<SelectionExpression, Box<dyn Error>> {
    if tokens.get()?.get_type() == QueryTokenType::Parenthesis(ParenthesisType::Opening) {
        tokens.consume()?;

        let left_subtree = parse_boolean_expression(tokens)?;

        tokens.consume_with_assert(QueryTokenType::Parenthesis(ParenthesisType::Closing))?;

        let subtree = combine_boolean_expression_with_prime(tokens, left_subtree)?;

        Ok(subtree)
    } else if tokens.get()?.get_type() == QueryTokenType::LogicalOperator(LogicalOperatorType::Not)
    {
        tokens.consume()?;
        let left_subtree = parse_boolean_expression(tokens)?;

        let subtree = combine_boolean_expression_with_prime(tokens, left_subtree)?;

        Ok(SelectionExpression::Not(Box::new(subtree)))
    } else {
        let left_operand = parse_compared_value(tokens)?;

        let comparison_operator;

        if let QueryTokenType::ComparisonOperator(_) = tokens.get()?.get_type() {
            comparison_operator = match tokens.consume()? {
                QueryToken::ComparisonOperator(comparison_operator) => *comparison_operator,
                _ => panic!("Error during parsing! Expected a comparison operator!"),
            };
        } else {
            return Err(format!(
                "Expected a comparison operator but found {:?}",
                tokens.get()?
            )
            .into());
        }

        let right_operand = parse_compared_value(tokens)?;

        let left_subtree = SelectionExpression::Comparison(
            Box::new(left_operand),
            Box::new(right_operand),
            comparison_operator,
        );

        let subtree = combine_boolean_expression_with_prime(tokens, left_subtree)?;

        Ok(subtree)
    }
}

fn parse_boolean_expression_prime(
    tokens: &mut TokenSupplier<QueryToken>,
) -> Result<Option<(LogicalOperatorType, SelectionExpression)>, Box<dyn Error>> {
    if let Some(token) = tokens.peek().map(|x| x.get_type()) {
        if QueryTokenType::LogicalOperator(LogicalOperatorType::And) == token
            || QueryTokenType::LogicalOperator(LogicalOperatorType::Or) == token
        {
            tokens.consume()?; // AND | OR

            Ok({
                let left_subtree = parse_boolean_expression(tokens)?;

                let subtree = combine_boolean_expression_with_prime(tokens, left_subtree)?;

                match token {
                    QueryTokenType::LogicalOperator(operator_type) => match operator_type {
                        LogicalOperatorType::Or => Some((LogicalOperatorType::Or, subtree)),
                        LogicalOperatorType::And => Some((LogicalOperatorType::And, subtree)),
                        _ => panic!("Error during parsing! Expected and or or!"),
                    },
                    _ => panic!("Error during parsing! Expected and or or!"),
                }
            })
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn combine_boolean_expression_with_prime(
    tokens: &mut TokenSupplier<QueryToken>,
    left_subtree: SelectionExpression,
) -> Result<SelectionExpression, Box<dyn Error>> {
    let expr_prime_result = parse_boolean_expression_prime(tokens)?;

    let subtree = if let Some((operator_type, right_subtree)) = expr_prime_result {
        match operator_type {
            LogicalOperatorType::And => {
                SelectionExpression::And(Box::new(left_subtree), Box::new(right_subtree))
            }
            LogicalOperatorType::Or => {
                SelectionExpression::Or(Box::new(left_subtree), Box::new(right_subtree))
            }
            _ => panic!(
                "Error during parsing! unexpected token: {:?}",
                right_subtree
            ),
        }
    } else {
        left_subtree
    };
    Ok(subtree)
}
