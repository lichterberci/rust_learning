use std::error::Error;

use regex::Match;

use crate::tokenizer;

#[derive(Debug, PartialEq)]
pub enum QueryToken {
    Select,
    Insert,
    Delete,
    From,
    Into,
    Values,
    Where,
    Comma,
    Semicolon,
    Dot,
    Identifier(String),
    Value(Value),
    Parenthesis(ParenthesisType),
    LogicalOperator(LogicalOperatorType),
    ComparisonOperator(ComparisonOperatorType),
    NumericalOperator(NumericalOperatorType),
}

impl QueryToken {
    pub fn get_type(&self) -> QueryTokenType {
        QueryTokenType::from(*self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QueryTokenType {
    Select,
    Insert,
    Delete,
    From,
    Into,
    Values,
    Where,
    Comma,
    Semicolon,
    Dot,
    Identifier,
    Value(ValueType),
    Parenthesis(ParenthesisType),
    LogicalOperator(LogicalOperatorType),
    ComparisonOperator(ComparisonOperatorType),
    NumericalOperator(NumericalOperatorType),
}

impl From<QueryToken> for QueryTokenType {
    fn from(value: QueryToken) -> Self {
        match value {
            QueryToken::Select => QueryTokenType::Select,
            QueryToken::Insert => QueryTokenType::Insert,
            QueryToken::Delete => QueryTokenType::Delete,
            QueryToken::From => QueryTokenType::From,
            QueryToken::Into => QueryTokenType::Into,
            QueryToken::Values => QueryTokenType::Values,
            QueryToken::Where => QueryTokenType::Where,
            QueryToken::Comma => QueryTokenType::Comma,
            QueryToken::Semicolon => QueryTokenType::Semicolon,
            QueryToken::Dot => QueryTokenType::Dot,
            QueryToken::Identifier(_) => QueryTokenType::Identifier,
            QueryToken::Value(value_type) => QueryTokenType::Value(match value_type {
                Value::Boolean(_) => ValueType::Boolean,
                Value::Integer(_) => ValueType::Integer,
                Value::Float(_) => ValueType::Float,
                Value::String(_) => ValueType::String,
            }),
            QueryToken::Parenthesis(parenthesis_type) => {
                QueryTokenType::Parenthesis(parenthesis_type)
            }
            QueryToken::LogicalOperator(logical_operator) => {
                QueryTokenType::LogicalOperator(logical_operator)
            }
            QueryToken::ComparisonOperator(comparison_operator) => {
                QueryTokenType::ComparisonOperator(comparison_operator)
            }
            QueryToken::NumericalOperator(numerical_operator) => {
                QueryTokenType::NumericalOperator(numerical_operator)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParenthesisType {
    Opening,
    Closing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComparisonOperatorType {
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogicalOperatorType {
    Not,
    Or,
    And,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumericalOperatorType {
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ValueType {
    Boolean,
    Integer,
    Float,
    String,
}

pub struct QueryTokenizer {
    tokenizer: tokenizer::Tokenizer<QueryToken, QueryTokenType>,
}

impl QueryTokenizer {
    pub fn new() -> Self {
        let mut grammar: Vec<(String, Option<QueryTokenType>)> = Vec::new();

        grammar.push((r"^(?<token>\s+)".into(), Option::None)); // whitespace
        grammar.push((r"^(?<token>;\-\-[^\n]*)".into(), Option::None)); // line comment
        grammar.push((
            r"^(?<token>\()".into(),
            Option::Some(QueryTokenType::Parenthesis(ParenthesisType::Opening)),
        ));
        grammar.push((
            r"^(?<token>\))".into(),
            Option::Some(QueryTokenType::Parenthesis(ParenthesisType::Opening)),
        ));
        grammar.push((
            r"^(?<token>\+)".into(),
            Option::Some(QueryTokenType::NumericalOperator(
                NumericalOperatorType::Add,
            )),
        ));
        grammar.push((
            r"^(?<token>\-)".into(),
            Option::Some(QueryTokenType::NumericalOperator(
                NumericalOperatorType::Sub,
            )),
        ));
        grammar.push((
            r"^(?<token>\*)".into(),
            Option::Some(QueryTokenType::NumericalOperator(
                NumericalOperatorType::Mult,
            )),
        ));
        grammar.push((
            r"^(?<token>/)".into(),
            Option::Some(QueryTokenType::NumericalOperator(
                NumericalOperatorType::Div,
            )),
        ));
        grammar.push((
            r"^(?<token>==)".into(),
            Option::Some(QueryTokenType::ComparisonOperator(
                ComparisonOperatorType::Equals,
            )),
        ));
        grammar.push((
            r"^(?<token>!=)".into(),
            Option::Some(QueryTokenType::ComparisonOperator(
                ComparisonOperatorType::NotEquals,
            )),
        ));
        grammar.push((
            r"^(?<token><)".into(),
            Option::Some(QueryTokenType::ComparisonOperator(
                ComparisonOperatorType::Less,
            )),
        ));
        grammar.push((
            r"^(?<token><=)".into(),
            Option::Some(QueryTokenType::ComparisonOperator(
                ComparisonOperatorType::LessEquals,
            )),
        ));
        grammar.push((
            r"^(?<token>>)".into(),
            Option::Some(QueryTokenType::ComparisonOperator(
                ComparisonOperatorType::Greater,
            )),
        ));
        grammar.push((
            r"^(?<token>>=)".into(),
            Option::Some(QueryTokenType::ComparisonOperator(
                ComparisonOperatorType::GreaterEquals,
            )),
        ));
        grammar.push((r"^(?<token>,)".into(), Option::Some(QueryTokenType::Comma)));
        grammar.push((
            r"^(?<token>;)[^--]".into(),
            Option::Some(QueryTokenType::Semicolon),
        ));
        grammar.push((
            "^(?<token>\"[^(\"|\n)]*\")".into(),
            Option::Some(QueryTokenType::Value(ValueType::String)),
        ));
        grammar.push((
            r"^(?<token>\d+\.\d*)[^\d]".into(),
            Option::Some(QueryTokenType::Value(ValueType::Float)),
        ));
        grammar.push((
            r"^(?<token>\d+)[^\d]".into(),
            Option::Some(QueryTokenType::Value(ValueType::Integer)),
        ));
        grammar.push((
            r"^(?<token>true|false)[^\w]".into(),
            Option::Some(QueryTokenType::Value(ValueType::Boolean)),
        ));
        grammar.push((
            r"^(?<token>select)[^\w]".into(),
            Option::Some(QueryTokenType::Select),
        ));
        grammar.push((
            r"^(?<token>insert)[^\w]".into(),
            Option::Some(QueryTokenType::Insert),
        ));
        grammar.push((
            r"^(?<token>where)[^\w]".into(),
            Option::Some(QueryTokenType::Where),
        ));
        grammar.push((
            r"^(?<token>values)[^\w]".into(),
            Option::Some(QueryTokenType::Values),
        ));
        grammar.push((
            r"^(?<token>from)[^\w]".into(),
            Option::Some(QueryTokenType::From),
        ));
        grammar.push((
            r"^(?<token>into)[^\w]".into(),
            Option::Some(QueryTokenType::Into),
        ));
        grammar.push((
            r"^(?<token>delete)[^\w]".into(),
            Option::Some(QueryTokenType::Delete),
        ));
        grammar.push((
            r"^(?<token>not)[^\w]".into(),
            Option::Some(QueryTokenType::LogicalOperator(LogicalOperatorType::Not)),
        ));
        grammar.push((
            r"^(?<token>and)[^\w]".into(),
            Option::Some(QueryTokenType::LogicalOperator(LogicalOperatorType::And)),
        ));
        grammar.push((
            r"^(?<token>or)[^\w]".into(),
            Option::Some(QueryTokenType::LogicalOperator(LogicalOperatorType::Or)),
        ));
        grammar.push((r"^(?<token>\.)".into(), Option::Some(QueryTokenType::Dot)));
        grammar.push((
            r"^(?<token>[\w_]+)[^\w_]".into(),
            Option::Some(QueryTokenType::Identifier),
        ));

        let matching_function = Box::new(
            |captured_group_of_token: &Match,
             inferred_type: &QueryTokenType|
             -> Result<QueryToken, Box<dyn Error>> {
                Ok(match inferred_type {
                    QueryTokenType::Select => QueryToken::Select,
                    QueryTokenType::Insert => QueryToken::Insert,
                    QueryTokenType::Delete => QueryToken::Delete,
                    QueryTokenType::From => QueryToken::From,
                    QueryTokenType::Into => QueryToken::Into,
                    QueryTokenType::Values => QueryToken::Values,
                    QueryTokenType::Where => QueryToken::Where,
                    QueryTokenType::Comma => QueryToken::Comma,
                    QueryTokenType::Semicolon => QueryToken::Semicolon,
                    QueryTokenType::Identifier => {
                        QueryToken::Identifier(captured_group_of_token.as_str().into())
                    }
                    QueryTokenType::Value(value_type) => match value_type {
                        ValueType::Boolean => QueryToken::Value(Value::Boolean(
                            captured_group_of_token.as_str() == "true",
                        )),
                        ValueType::Integer => QueryToken::Value(Value::Integer(
                            captured_group_of_token.as_str().parse()?,
                        )),
                        ValueType::Float => QueryToken::Value(Value::Float(
                            captured_group_of_token.as_str().parse()?,
                        )),
                        ValueType::String => QueryToken::Value(Value::String(
                            // we leave out the first and the last characters, as they would be the "" characters
                            captured_group_of_token.as_str()[1..captured_group_of_token.len() - 1]
                                .into(),
                        )),
                    },
                    QueryTokenType::Parenthesis(parent_type) => match parent_type {
                        ParenthesisType::Opening => {
                            QueryToken::Parenthesis(ParenthesisType::Opening)
                        }
                        ParenthesisType::Closing => {
                            QueryToken::Parenthesis(ParenthesisType::Closing)
                        }
                    },
                    QueryTokenType::LogicalOperator(operator_type) => match operator_type {
                        LogicalOperatorType::Not => {
                            QueryToken::LogicalOperator(LogicalOperatorType::Not)
                        }
                        LogicalOperatorType::Or => {
                            QueryToken::LogicalOperator(LogicalOperatorType::Or)
                        }
                        LogicalOperatorType::And => {
                            QueryToken::LogicalOperator(LogicalOperatorType::And)
                        }
                    },
                    QueryTokenType::ComparisonOperator(comparison_operator_type) => {
                        match comparison_operator_type {
                            ComparisonOperatorType::Equals => {
                                QueryToken::ComparisonOperator(ComparisonOperatorType::Equals)
                            }
                            ComparisonOperatorType::NotEquals => {
                                QueryToken::ComparisonOperator(ComparisonOperatorType::NotEquals)
                            }
                            ComparisonOperatorType::Greater => {
                                QueryToken::ComparisonOperator(ComparisonOperatorType::Greater)
                            }
                            ComparisonOperatorType::GreaterEquals => {
                                QueryToken::ComparisonOperator(
                                    ComparisonOperatorType::GreaterEquals,
                                )
                            }
                            ComparisonOperatorType::Less => {
                                QueryToken::ComparisonOperator(ComparisonOperatorType::Less)
                            }
                            ComparisonOperatorType::LessEquals => {
                                QueryToken::ComparisonOperator(ComparisonOperatorType::LessEquals)
                            }
                        }
                    }
                    QueryTokenType::NumericalOperator(numerical_operator_type) => {
                        match numerical_operator_type {
                            NumericalOperatorType::Add => {
                                QueryToken::NumericalOperator(NumericalOperatorType::Add)
                            }
                            NumericalOperatorType::Sub => {
                                QueryToken::NumericalOperator(NumericalOperatorType::Sub)
                            }
                            NumericalOperatorType::Mult => {
                                QueryToken::NumericalOperator(NumericalOperatorType::Mult)
                            }
                            NumericalOperatorType::Div => {
                                QueryToken::NumericalOperator(NumericalOperatorType::Div)
                            }
                        }
                    }
                    QueryTokenType::Dot => QueryToken::Dot,
                })
            },
        );

        Self {
            tokenizer: tokenizer::Tokenizer::build(&grammar, matching_function, "token"),
        }
    }

    pub fn tokenize(&self, input: &str) -> Result<Vec<QueryToken>, Box<dyn Error>> {
        self.tokenizer.tokenize(input)
    }
}
