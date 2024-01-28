use std::{collections::HashMap, error::Error};

use regex::Regex;

#[derive(Debug)]
pub struct QuerySymbolStream {
    symbols: Vec<QuerySymbol>,
}

impl QuerySymbolStream {
    pub fn new() -> Self {
        QuerySymbolStream { symbols: vec![] }
    }

    pub fn append(&mut self, symbol: QuerySymbol) {
        self.symbols.push(symbol);
    }

    pub fn get_symbols(&self) -> &[QuerySymbol] {
        &self.symbols
    }

    pub fn last_symbol(&self) -> Option<&QuerySymbol> {
        self.symbols.last()
    }
}

#[derive(Debug)]
pub enum QuerySymbol {
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

#[derive(Debug)]
pub enum ParenthesisType {
    Opening,
    Closing,
}

#[derive(Debug)]
pub enum ComparisonOperatorType {
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
}

#[derive(Debug)]
pub enum LogicalOperatorType {
    Not,
    Or,
    And,
}

#[derive(Debug)]
pub enum NumericalOperatorType {
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

pub fn lex_string(input: &str) -> Result<QuerySymbolStream, Box<dyn Error>> {
    let input = input.to_lowercase();
    let input = input.trim().to_owned() + " "; // this space is crucial for the regex patterns to match

    let mut pattern_list: Vec<(String, Option<QuerySymbol>)> = Vec::new();

    pattern_list.push((r"^(?<token>\s+)".into(), Option::None)); // whitespace
    pattern_list.push((r"^(?<token>;\-\-[^\n]*)".into(), Option::None)); // line comment
    pattern_list.push((
        r"^(?<token>\()".into(),
        Option::Some(QuerySymbol::Parenthesis(ParenthesisType::Opening)),
    ));
    pattern_list.push((
        r"^(?<token>\))".into(),
        Option::Some(QuerySymbol::Parenthesis(ParenthesisType::Opening)),
    ));
    pattern_list.push((
        r"^(?<token>\+)".into(),
        Option::Some(QuerySymbol::NumericalOperator(NumericalOperatorType::Add)),
    ));
    pattern_list.push((
        r"^(?<token>\-)".into(),
        Option::Some(QuerySymbol::NumericalOperator(NumericalOperatorType::Sub)),
    ));
    pattern_list.push((
        r"^(?<token>\*)".into(),
        Option::Some(QuerySymbol::NumericalOperator(NumericalOperatorType::Mult)),
    ));
    pattern_list.push((
        r"^(?<token>/)".into(),
        Option::Some(QuerySymbol::NumericalOperator(NumericalOperatorType::Div)),
    ));
    pattern_list.push((
        r"^(?<token>==)".into(),
        Option::Some(QuerySymbol::ComparisonOperator(
            ComparisonOperatorType::Equals,
        )),
    ));
    pattern_list.push((
        r"^(?<token>!=)".into(),
        Option::Some(QuerySymbol::ComparisonOperator(
            ComparisonOperatorType::NotEquals,
        )),
    ));
    pattern_list.push((
        r"^(?<token><)".into(),
        Option::Some(QuerySymbol::ComparisonOperator(
            ComparisonOperatorType::Less,
        )),
    ));
    pattern_list.push((
        r"^(?<token><=)".into(),
        Option::Some(QuerySymbol::ComparisonOperator(
            ComparisonOperatorType::LessEquals,
        )),
    ));
    pattern_list.push((
        r"^(?<token>>)".into(),
        Option::Some(QuerySymbol::ComparisonOperator(
            ComparisonOperatorType::Greater,
        )),
    ));
    pattern_list.push((
        r"^(?<token>>=)".into(),
        Option::Some(QuerySymbol::ComparisonOperator(
            ComparisonOperatorType::GreaterEquals,
        )),
    ));
    pattern_list.push((r"^(?<token>,)".into(), Option::Some(QuerySymbol::Comma)));
    pattern_list.push((
        r"^(?<token>;)[^--]".into(),
        Option::Some(QuerySymbol::Semicolon),
    ));
    pattern_list.push((
        "^(?<token>\"[^(\"|\n)]*\")".into(),
        Option::Some(QuerySymbol::Value(Value::String("".into()))),
    ));
    pattern_list.push((
        r"^(?<token>\d+\.\d*)[^\d]".into(),
        Option::Some(QuerySymbol::Value(Value::Float(0.0))),
    ));
    pattern_list.push((
        r"^(?<token>\d+)[^\d]".into(),
        Option::Some(QuerySymbol::Value(Value::Integer(0))),
    ));
    pattern_list.push((
        r"^(?<token>true|false)[^\w]".into(),
        Option::Some(QuerySymbol::Value(Value::Boolean(false))),
    ));
    pattern_list.push((
        r"^(?<token>select)[^\w]".into(),
        Option::Some(QuerySymbol::Select),
    ));
    pattern_list.push((
        r"^(?<token>insert)[^\w]".into(),
        Option::Some(QuerySymbol::Insert),
    ));
    pattern_list.push((
        r"^(?<token>where)[^\w]".into(),
        Option::Some(QuerySymbol::Where),
    ));
    pattern_list.push((
        r"^(?<token>values)[^\w]".into(),
        Option::Some(QuerySymbol::Values),
    ));
    pattern_list.push((
        r"^(?<token>from)[^\w]".into(),
        Option::Some(QuerySymbol::From),
    ));
    pattern_list.push((
        r"^(?<token>into)[^\w]".into(),
        Option::Some(QuerySymbol::Into),
    ));
    pattern_list.push((
        r"^(?<token>delete)[^\w]".into(),
        Option::Some(QuerySymbol::Delete),
    ));
    pattern_list.push((
        r"^(?<token>not)[^\w]".into(),
        Option::Some(QuerySymbol::LogicalOperator(LogicalOperatorType::Not)),
    ));
    pattern_list.push((
        r"^(?<token>and)[^\w]".into(),
        Option::Some(QuerySymbol::LogicalOperator(LogicalOperatorType::And)),
    ));
    pattern_list.push((
        r"^(?<token>or)[^\w]".into(),
        Option::Some(QuerySymbol::LogicalOperator(LogicalOperatorType::Or)),
    ));
    pattern_list.push((r"^(?<token>\.)".into(), Option::Some(QuerySymbol::Dot)));
    pattern_list.push((
        r"^(?<token>[\w_]+)[^\w_]".into(),
        Option::Some(QuerySymbol::Identifier("".into())),
    ));

    let mut head_index = 0;
    let mut output = QuerySymbolStream::new();

    let parsed_pattern_map: Vec<(Regex, &Option<QuerySymbol>)> = pattern_list
        .iter()
        .map(|(pattern, symbol_type)| {
            (
                Regex::new(pattern).expect("Pattern should be valid regex!"),
                symbol_type,
            )
        })
        .collect();

    'token_loop: while head_index < input.len() {
        let input = &input[head_index..];

        for (pattern, inferred_type) in &parsed_pattern_map {
            let captures = pattern.captures(input);

            // no matching here
            if captures.is_none() {
                continue;
            }

            let captures = captures.expect("There should be a capture here!");

            let captured_group_of_token = captures
                .name("token")
                .expect("There should be a capture here!");

            // this is a separator or comment
            if inferred_type.is_none() {
                head_index += captured_group_of_token.as_str().len();
                continue 'token_loop;
            }

            let extracted_symbol = match inferred_type {
                Some(inferred_type) => match inferred_type {
                    QuerySymbol::Select => QuerySymbol::Select,
                    QuerySymbol::Insert => QuerySymbol::Insert,
                    QuerySymbol::Delete => QuerySymbol::Delete,
                    QuerySymbol::From => QuerySymbol::From,
                    QuerySymbol::Into => QuerySymbol::Into,
                    QuerySymbol::Values => QuerySymbol::Values,
                    QuerySymbol::Where => QuerySymbol::Where,
                    QuerySymbol::Comma => QuerySymbol::Comma,
                    QuerySymbol::Semicolon => QuerySymbol::Semicolon,
                    QuerySymbol::Identifier(_) => {
                        QuerySymbol::Identifier(captured_group_of_token.as_str().into())
                    }
                    QuerySymbol::Value(value_type) => match value_type {
                        Value::Boolean(_) => QuerySymbol::Value(Value::Boolean(
                            captured_group_of_token.as_str() == "true",
                        )),
                        Value::Integer(_) => QuerySymbol::Value(Value::Integer(
                            captured_group_of_token.as_str().parse()?,
                        )),
                        Value::Float(_) => QuerySymbol::Value(Value::Float(
                            captured_group_of_token.as_str().parse()?,
                        )),
                        Value::String(_) => QuerySymbol::Value(Value::String(
                            // we leave out the first and the last characters, as they would be the "" characters
                            captured_group_of_token.as_str()[1..captured_group_of_token.len() - 1]
                                .into(),
                        )),
                    },
                    QuerySymbol::Parenthesis(parent_type) => match parent_type {
                        ParenthesisType::Opening => {
                            QuerySymbol::Parenthesis(ParenthesisType::Opening)
                        }
                        ParenthesisType::Closing => {
                            QuerySymbol::Parenthesis(ParenthesisType::Closing)
                        }
                    },
                    QuerySymbol::LogicalOperator(operator_type) => match operator_type {
                        LogicalOperatorType::Not => {
                            QuerySymbol::LogicalOperator(LogicalOperatorType::Not)
                        }
                        LogicalOperatorType::Or => {
                            QuerySymbol::LogicalOperator(LogicalOperatorType::Or)
                        }
                        LogicalOperatorType::And => {
                            QuerySymbol::LogicalOperator(LogicalOperatorType::And)
                        }
                    },
                    QuerySymbol::ComparisonOperator(comparison_operator_type) => {
                        match comparison_operator_type {
                            ComparisonOperatorType::Equals => {
                                QuerySymbol::ComparisonOperator(ComparisonOperatorType::Equals)
                            }
                            ComparisonOperatorType::NotEquals => {
                                QuerySymbol::ComparisonOperator(ComparisonOperatorType::NotEquals)
                            }
                            ComparisonOperatorType::Greater => {
                                QuerySymbol::ComparisonOperator(ComparisonOperatorType::Greater)
                            }
                            ComparisonOperatorType::GreaterEquals => {
                                QuerySymbol::ComparisonOperator(
                                    ComparisonOperatorType::GreaterEquals,
                                )
                            }
                            ComparisonOperatorType::Less => {
                                QuerySymbol::ComparisonOperator(ComparisonOperatorType::Less)
                            }
                            ComparisonOperatorType::LessEquals => {
                                QuerySymbol::ComparisonOperator(ComparisonOperatorType::LessEquals)
                            }
                        }
                    }
                    QuerySymbol::NumericalOperator(numerical_operator_type) => {
                        match numerical_operator_type {
                            NumericalOperatorType::Add => {
                                QuerySymbol::NumericalOperator(NumericalOperatorType::Add)
                            }
                            NumericalOperatorType::Sub => {
                                QuerySymbol::NumericalOperator(NumericalOperatorType::Sub)
                            }
                            NumericalOperatorType::Mult => {
                                QuerySymbol::NumericalOperator(NumericalOperatorType::Mult)
                            }
                            NumericalOperatorType::Div => {
                                QuerySymbol::NumericalOperator(NumericalOperatorType::Div)
                            }
                        }
                    }
                    QuerySymbol::Dot => QuerySymbol::Dot,
                },
                None => return Err("Inferred type should not be None here!".into()),
            };

            head_index += captured_group_of_token.as_str().len();

            output.append(extracted_symbol);

            continue 'token_loop;
        }

        return Err(format!("Sequence not recognized: {input}").into());
    }

    Ok(output)
}
