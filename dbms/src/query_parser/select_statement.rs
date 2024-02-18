use std::error::Error;

use crate::{
    query_tokenizer::{NumericalOperatorType, QueryToken, QueryTokenType},
    rel_alg_ast::{Identifier, RelAlgAST},
};

use super::{parse_boolean_expression, TokenSupplier};

pub fn parse_select_statement(tokens: &mut TokenSupplier) -> Result<RelAlgAST, Box<dyn Error>> {
    tokens.consume()?;
    let projection = parse_projection(tokens)?;
    tokens.consume_with_assert(QueryTokenType::From)?;
    let source_relations = parse_source_tables(tokens)?;

    if tokens.get()?.get_type() == QueryTokenType::Where {
        tokens.consume()?; // where

        let boolean_expression = parse_boolean_expression(tokens)?;

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
        if let Some(projected_attributes) = projection {
            return Ok(RelAlgAST::Projection(
                Box::new(source_relations),
                projected_attributes,
            ));
        } else {
            return Ok(source_relations);
        }
    }
}

fn parse_projection(tokens: &mut TokenSupplier) -> Result<Option<Vec<Identifier>>, Box<dyn Error>> {
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

        Ok(Some(
            vec![
                vec![Identifier::AttributeName(identifier)],
                rest_of_selection,
            ]
            .concat(),
        ))
    } else if tokens.get()?.get_type() == QueryTokenType::Dot {
        tokens.consume()?; // .

        let second_identifier = match tokens.consume_with_assert(QueryTokenType::Identifier)? {
            QueryToken::Identifier(identifier) => String::from(identifier),
            _ => return Err("Identifier expected!".into()),
        };

        if tokens.get()?.get_type() == QueryTokenType::Comma {
            tokens.consume()?; // ,

            let Some(rest_of_selection) = parse_projection(tokens)? else {
                return Err("Identifier expected after comma when selecting columns!".into());
            };

            Ok(Some(
                vec![
                    vec![Identifier::QualifiedAttributeName(
                        identifier,
                        second_identifier,
                    )],
                    rest_of_selection,
                ]
                .concat(),
            ))
        } else {
            Ok(Some(vec![Identifier::QualifiedAttributeName(
                identifier,
                second_identifier,
            )]))
        }
    } else {
        Ok(Some(vec![Identifier::AttributeName(identifier)]))
    }
}

fn parse_source_tables(tokens: &mut TokenSupplier) -> Result<RelAlgAST, Box<dyn Error>> {
    let identifier = match tokens.consume_with_assert(QueryTokenType::Identifier)? {
        QueryToken::Identifier(identifier) => String::from(identifier),
        _ => return Err("Identifier expected!".into()),
    };

    if tokens.get()?.get_type() == QueryTokenType::Comma {
        tokens.consume()?; // ,

        let rest_of_selection = parse_source_tables(tokens)?;

        Ok(RelAlgAST::CartesianProduct(
            Box::new(RelAlgAST::Relation(Identifier::RelationName(identifier))),
            Box::new(rest_of_selection),
        ))
    } else {
        Ok(RelAlgAST::Relation(Identifier::RelationName(identifier)))
    }
}
