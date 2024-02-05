use crate::query_tokenizer::{ComparisonOperatorType, NumericalOperatorType, QueryToken, Value};

#[derive(Debug, PartialEq)]
pub enum RelAlgAST {
    Relation(String),
    Union(Box<RelAlgAST>, Box<RelAlgAST>),
    CartesianProduct(Box<RelAlgAST>, Box<RelAlgAST>),
    Selection(Box<RelAlgAST>, SelectionExpression),
    Projection(Box<RelAlgAST>, Vec<String>),
}

#[derive(Debug, PartialEq)]
pub enum SelectionExpression {
    Comparison(
        Box<ComparedValue>,
        Box<ComparedValue>,
        ComparisonOperatorType,
    ),
    And(Box<SelectionExpression>, Box<SelectionExpression>),
    Or(Box<SelectionExpression>, Box<SelectionExpression>),
    Not(Box<SelectionExpression>),
}

#[derive(PartialEq, Debug)]
pub enum ComparedValue {
    Identifier(String),
    Value(Value),
    Composite(
        NumericalOperatorType,
        Box<ComparedValue>,
        Box<ComparedValue>,
    ),
}
