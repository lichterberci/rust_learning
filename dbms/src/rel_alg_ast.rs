use crate::query_tokenizer::{ComparisonOperatorType, NumericalOperatorType, QueryToken, Value};

#[derive(Debug, PartialEq)]
pub enum RelAlgAST {
    Relation(Identifier),
    ConstantTuple(Vec<ConstantCalculatedValue>),
    CalculatedTuple(Vec<CalculatedValue>),
    Union(Box<RelAlgAST>, Box<RelAlgAST>),
    Difference(Box<RelAlgAST>, Box<RelAlgAST>),
    CartesianProduct(Box<RelAlgAST>, Box<RelAlgAST>),
    Selection(Box<RelAlgAST>, SelectionExpression),
    Projection(Box<RelAlgAST>, Vec<Identifier>),
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
    Identifier(Identifier),
    Value(Value),
    Composite(
        NumericalOperatorType,
        Box<ComparedValue>,
        Box<ComparedValue>,
    ),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Identifier {
    AttributeName(String),
    RelationName(String),
    QualifiedAttributeName(String, String),
}

#[derive(PartialEq, Debug)]
pub enum CalculatedValue {
    Value(Value),
    Identifier(Identifier),
    Not(Box<CalculatedValue>),
    NumericalOperation(
        NumericalOperatorType,
        Box<CalculatedValue>,
        Box<CalculatedValue>,
    ),
    Comparison(
        ComparisonOperatorType,
        Box<CalculatedValue>,
        Box<CalculatedValue>,
    ),
    And(Box<CalculatedValue>, Box<CalculatedValue>),
    Or(Box<CalculatedValue>, Box<CalculatedValue>),
}

#[derive(PartialEq, Debug)]
pub enum ConstantCalculatedValue {
    Value(Value),
    Not(Box<ConstantCalculatedValue>),
    NumericalOperation(
        NumericalOperatorType,
        Box<ConstantCalculatedValue>,
        Box<ConstantCalculatedValue>,
    ),
    Comparison(
        ComparisonOperatorType,
        Box<ConstantCalculatedValue>,
        Box<ConstantCalculatedValue>,
    ),
    And(Box<ConstantCalculatedValue>, Box<ConstantCalculatedValue>),
    Or(Box<ConstantCalculatedValue>, Box<ConstantCalculatedValue>),
}
