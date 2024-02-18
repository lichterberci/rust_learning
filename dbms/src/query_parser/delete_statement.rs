use std::error::Error;

use crate::{
    query_tokenizer::{QueryToken, QueryTokenType},
    rel_alg_ast::{Identifier, RelAlgAST},
    token_supplier::TokenSupplier,
};

use super::parse_boolean_expression;

pub fn parse_delete_statement(
    tokens: &mut TokenSupplier<QueryToken>,
) -> Result<RelAlgAST, Box<dyn Error>> {
    tokens.consume_with_assert(QueryTokenType::Delete)?;
    tokens.consume_with_assert(QueryTokenType::From)?;

    let QueryToken::Identifier(relation_name) =
        tokens.consume_with_assert(QueryTokenType::Identifier)?
    else {
        return Err("Expected an identifier!".into());
    };

    let identifier = Identifier::RelationName(String::from(relation_name));

    tokens.consume_with_assert(QueryTokenType::Where)?;

    let filter_expression = parse_boolean_expression(tokens)?;

    Ok(RelAlgAST::Difference(
        Box::new(RelAlgAST::Relation(identifier.clone())),
        Box::new(RelAlgAST::Selection(
            Box::new(RelAlgAST::Relation(identifier)),
            filter_expression,
        )),
    ))
}
