use crate::query_tokenizer::{ComparisonOperatorType, Value};

#[derive(Debug, PartialEq)]
pub enum RelAlgAST {
    Relation(String),
    Union(Box<RelAlgAST>, Box<RelAlgAST>),
    DescartesProduct(Box<RelAlgAST>, Box<RelAlgAST>),
    Selection(Box<RelAlgAST>, SelectionExpression),
    Projection(Box<RelAlgAST>, Vec<String>),
}

#[derive(Debug, PartialEq)]
pub enum SelectionExpression {
    Identifier(String),
    Value(Value),
    Comparison(
        Box<SelectionExpression>,
        Box<SelectionExpression>,
        ComparisonOperatorType,
    ),
    And(Box<SelectionExpression>, Box<SelectionExpression>),
    Or(Box<SelectionExpression>, Box<SelectionExpression>),
    Not(Box<SelectionExpression>),
}
