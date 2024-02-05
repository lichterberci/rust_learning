use core::panic;
use std::{clone, error::Error};

use crate::{
    query_tokenizer::{LogicalOperatorType, ParenthesisType, QueryToken, QueryTokenType, Value},
    rel_alg_ast::SelectionExpression,
};

pub struct TokenSupplier {
    tokens: Vec<QueryToken>,
    head: usize,
}

impl TokenSupplier {
    pub fn new(tokens: Vec<QueryToken>) -> Self {
        Self { tokens, head: 0 }
    }

    fn peek(&self) -> Option<&QueryToken> {
        self.tokens.get(self.head)
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

pub fn parse_boolean_expression(
    tokens: &mut TokenSupplier,
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
        let right_subtree = parse_boolean_expression_prime(tokens)?;

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

fn combine_boolean_expression_with_prime(
    tokens: &mut TokenSupplier,
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

fn parse_compared_value(tokens: &mut TokenSupplier) -> Result<SelectionExpression, Box<dyn Error>> {
    if tokens.get()?.get_type() == QueryTokenType::Identifier {
        let first_identifier = tokens.consume()?;

        let first_id_string = match first_identifier {
            QueryToken::Identifier(name) => String::from(name),
            _ => panic!("Error during parsing! Expected an identifier!"),
        };

        if tokens.get()?.get_type() != QueryTokenType::Dot {
            return Ok(SelectionExpression::Identifier(first_id_string));
        }

        tokens.consume()?; // dot

        let second_identifier = tokens.consume_with_assert(QueryTokenType::Identifier)?;

        let second_id_string = match second_identifier {
            QueryToken::Identifier(name) => String::from(name),
            _ => panic!("Error during parsing! Expected an identifier!"),
        };

        return Ok(SelectionExpression::Identifier(String::from(format!(
            "{first_id_string}.{second_id_string}"
        ))));
    } else if let QueryToken::Value(_) = tokens.get()? {
        let token = tokens.consume()?;
        Ok(SelectionExpression::Value(match token {
            QueryToken::Value(value) => value.clone(),
            _ => panic!("Error during parsing! Expected a value here!"),
        }))
    } else {
        Err(format!("Expected Identifier or Value but got {:?}", tokens.get()?).into())
    }
}

fn parse_boolean_expression_prime(
    tokens: &mut TokenSupplier,
) -> Result<Option<(LogicalOperatorType, SelectionExpression)>, Box<dyn Error>> {
    if let Some(token) = tokens.peek().map(|x| x.get_type()) {
        if QueryTokenType::LogicalOperator(LogicalOperatorType::And) == token
            || QueryTokenType::LogicalOperator(LogicalOperatorType::Or) == token
        {
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
