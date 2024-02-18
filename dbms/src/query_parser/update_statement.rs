use std::error::Error;

use crate::{
    query_tokenizer::{
        ParenthesisType::{Closing, Opening},
        QueryToken, QueryTokenType,
    },
    rel_alg_ast::{CalculatedValue, Identifier, ProjectedColumns, RelAlgAST, TransformFunction},
};

use super::{calculated_value::parse_calculated_value, parse_boolean_expression, TokenSupplier};

pub fn parse_update_statement(tokens: &mut TokenSupplier) -> Result<RelAlgAST, Box<dyn Error>> {
    tokens.consume_with_assert(QueryTokenType::Update)?;

    let relation_name = match tokens.consume_with_assert(QueryTokenType::Identifier)? {
        QueryToken::Identifier(relation_name) => String::from(relation_name),
        _ => return Err("Expected an identifier!".into()),
    };

    tokens.consume_with_assert(QueryTokenType::Parenthesis(Opening))?;

    let attribute_names = parse_attribute_names(tokens)?;

    tokens.consume_with_assert(QueryTokenType::Parenthesis(Closing))?;

    tokens.consume_with_assert(QueryTokenType::Values)?;

    tokens.consume_with_assert(QueryTokenType::Parenthesis(Opening))?;

    let attribute_values = parse_attribute_values(tokens)?;

    tokens.consume_with_assert(QueryTokenType::Parenthesis(Closing))?;

    tokens.consume_with_assert(QueryTokenType::Where)?;

    let filter_expression = parse_boolean_expression(tokens)?;

    if attribute_names.len() != attribute_values.len() {
        return Err("Attribute names and values don't have the same length!".into());
    }

    Ok(RelAlgAST::Union(
        Box::new(RelAlgAST::Difference(
            Box::new(RelAlgAST::Relation(Identifier::RelationName(String::from(
                &relation_name,
            )))),
            Box::new(RelAlgAST::Selection(
                Box::new(RelAlgAST::Relation(Identifier::RelationName(String::from(
                    &relation_name,
                )))),
                filter_expression.clone(),
            )),
        )),
        Box::new(RelAlgAST::Selection(
            Box::new(RelAlgAST::Projection(
                Box::new(RelAlgAST::Relation(Identifier::RelationName(String::from(
                    &relation_name,
                )))),
                ProjectedColumns::All,
                TransformFunction::General(
                    attribute_names
                        .into_iter()
                        .zip(attribute_values.into_iter())
                        .collect(),
                ),
            )),
            filter_expression,
        )),
    ))
}

fn parse_attribute_names(tokens: &mut TokenSupplier) -> Result<Vec<Identifier>, Box<dyn Error>> {
    let identifier_token = match tokens.consume_with_assert(QueryTokenType::Identifier)? {
        QueryToken::Identifier(id) => Identifier::AttributeName(String::from(id)),
        _ => return Err("Expected an identifier!".into()),
    };

    if tokens.get()?.get_type() == QueryTokenType::Comma {
        tokens.consume()?; // ,

        let other_attribute_names = parse_attribute_names(tokens)?;

        Ok(vec![vec![identifier_token], other_attribute_names].concat())
    } else {
        Ok(vec![identifier_token])
    }
}

fn parse_attribute_values(
    tokens: &mut TokenSupplier,
) -> Result<Vec<CalculatedValue>, Box<dyn Error>> {
    let value_tree = parse_calculated_value(tokens)?;

    if tokens.get()?.get_type() == QueryTokenType::Comma {
        tokens.consume()?; // ,

        let other_values = parse_attribute_values(tokens)?;

        Ok(vec![vec![value_tree], other_values].concat())
    } else {
        Ok(vec![value_tree])
    }
}
