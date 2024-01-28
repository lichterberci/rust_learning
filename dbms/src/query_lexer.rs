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
    let input = input.trim();

    let mut pattern_map: HashMap<String, Option<QuerySymbol>> = HashMap::new();
    pattern_map.insert(r"^\s+".into(), Option::None); // whitespace
    pattern_map.insert(r"^;--[^\n]*".into(), Option::None); // line comment
    pattern_map.insert(
        r"^\(".into(),
        Option::Some(QuerySymbol::Parenthesis(ParenthesisType::Opening)),
    );
    pattern_map.insert(
        r"^\)".into(),
        Option::Some(QuerySymbol::Parenthesis(ParenthesisType::Opening)),
    );
    pattern_map.insert(
        r"^\+".into(),
        Option::Some(QuerySymbol::NumericalOperator(NumericalOperatorType::Add)),
    );
    pattern_map.insert(
        r"^\-".into(),
        Option::Some(QuerySymbol::NumericalOperator(NumericalOperatorType::Sub)),
    );
    pattern_map.insert(
        r"^\*".into(),
        Option::Some(QuerySymbol::NumericalOperator(NumericalOperatorType::Mult)),
    );
    pattern_map.insert(
        r"^/".into(),
        Option::Some(QuerySymbol::NumericalOperator(NumericalOperatorType::Div)),
    );
    pattern_map.insert(
        r"^==".into(),
        Option::Some(QuerySymbol::ComparisonOperator(
            ComparisonOperatorType::Equals,
        )),
    );
    pattern_map.insert(
        r"^!=".into(),
        Option::Some(QuerySymbol::ComparisonOperator(
            ComparisonOperatorType::NotEquals,
        )),
    );
    pattern_map.insert(
        r"^<".into(),
        Option::Some(QuerySymbol::ComparisonOperator(
            ComparisonOperatorType::Less,
        )),
    );
    pattern_map.insert(
        r"^<=".into(),
        Option::Some(QuerySymbol::ComparisonOperator(
            ComparisonOperatorType::LessEquals,
        )),
    );
    pattern_map.insert(
        r"^>".into(),
        Option::Some(QuerySymbol::ComparisonOperator(
            ComparisonOperatorType::Greater,
        )),
    );
    pattern_map.insert(
        r"^>=".into(),
        Option::Some(QuerySymbol::ComparisonOperator(
            ComparisonOperatorType::GreaterEquals,
        )),
    );
    pattern_map.insert(r"^,".into(), Option::Some(QuerySymbol::Comma));
    pattern_map.insert(r"^;".into(), Option::Some(QuerySymbol::Semicolon));
    pattern_map.insert(
        "^\"[^(\"|\n)]*\"".into(),
        Option::Some(QuerySymbol::Value(Value::String("".into()))),
    );
    pattern_map.insert(
        r"^\d+\.\d*[^\d]".into(),
        Option::Some(QuerySymbol::Value(Value::Float(0.0))),
    );
    pattern_map.insert(
        r"^\d+[^\d]".into(),
        Option::Some(QuerySymbol::Value(Value::Integer(0))),
    );
    pattern_map.insert(
        r"^(true|false)[^\w]".into(),
        Option::Some(QuerySymbol::Value(Value::Boolean(false))),
    );

    let mut head_index = 0;
    let mut output = QuerySymbolStream::new();

    'token_loop: while head_index < input.len() {
        let input = &input[head_index..];

        for (pattern, inferred_type) in &pattern_map {
            let pattern = Regex::new(pattern)?;

            let captures = pattern.captures(input);

            // no matching here
            if captures.is_none() {
                continue;
            }

            let captures = captures.expect("There should be a capture here!");

            let first_capture = captures.get(0).expect("There should be a capture here!");

            // this is a separator or comment
            if inferred_type.is_none() {
                head_index += first_capture.as_str().len();
                continue 'token_loop;
            }

            println!("first capture: {:?}", first_capture);

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
                        QuerySymbol::Identifier(first_capture.as_str().into())
                    }
                    QuerySymbol::Value(value_type) => match value_type {
                        Value::Boolean(_) => QuerySymbol::Value(Value::Boolean(
                            first_capture.as_str().starts_with("true"),
                        )),
                        Value::Integer(_) => QuerySymbol::Value(Value::Integer(
                            first_capture.as_str()[..first_capture.as_str().len() - 1].parse()?,
                        )),
                        Value::Float(_) => QuerySymbol::Value(Value::Float(
                            first_capture.as_str()[..first_capture.as_str().len() - 1].parse()?,
                        )),
                        Value::String(_) => QuerySymbol::Value(Value::String(
                            // we leave out the first and the last characters, as they would be the "" characters
                            first_capture.as_str()[1..first_capture.len() - 1].into(),
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
                },
                None => return Err("Inferred type should not be None here!".into()),
            };

            head_index += first_capture.as_str().len();

            println!(
                "Extracted symbol: \"{}\" {:?}",
                &first_capture.as_str(),
                &extracted_symbol
            );

            output.append(extracted_symbol);

            continue 'token_loop;
        }

        return Err(format!("Sequence not recognized: {input}").into());
    }

    Ok(output)
}
