use std::{collections::HashMap, error::Error};

use regex::Regex;

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

pub enum QuerySymbol {
    Select,
    Insert,
    Delete,
    From,
    Into,
    Values,
    Where,
    Identifier(String),
    Value(Value),
    Parenthesis(ParenthesisType),
    LogicalOperator(LogicalOperatorType),
    ComparisonOperatorType(ComparisonOperatorType),
}

pub enum ParenthesisType {
    Opening,
    Closing,
}

pub enum ComparisonOperatorType {
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
}

pub enum LogicalOperatorType {
    Not,
    Or,
    And,
}

pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

pub fn lex_string(input: &str) -> Result<QuerySymbolStream, Box<dyn Error>> {
    let input = input.to_lowercase().trim();

    let mut pattern_map: HashMap<String, Option<QuerySymbol>> = HashMap::new();
    pattern_map.insert(r"^\s+".into(), Option::None);

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

            // this is a separator
            if inferred_type.is_none() {
                head_index += first_capture.as_str().len();
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
                    QuerySymbol::Identifier(_) => {
                        QuerySymbol::Identifier(first_capture.as_str().into())
                    }
                    QuerySymbol::Value(value_type) => match value_type {
                        Value::Boolean(_) => {
                            QuerySymbol::Value(Value::Boolean(first_capture.as_str() == "true"))
                        }
                        Value::Integer(_) => {
                            QuerySymbol::Value(Value::Integer(first_capture.as_str().parse()?))
                        }
                        Value::Float(_) => {
                            QuerySymbol::Value(Value::Float(first_capture.as_str().parse()?))
                        }
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
                    QuerySymbol::ComparisonOperatorType(comparison_operator_type) => {
                        match comparison_operator_type {
                            ComparisonOperatorType::Equals => {
                                QuerySymbol::ComparisonOperatorType(ComparisonOperatorType::Equals)
                            }
                            ComparisonOperatorType::NotEquals => {
                                QuerySymbol::ComparisonOperatorType(
                                    ComparisonOperatorType::NotEquals,
                                )
                            }
                            ComparisonOperatorType::Greater => {
                                QuerySymbol::ComparisonOperatorType(ComparisonOperatorType::Greater)
                            }
                            ComparisonOperatorType::GreaterEquals => {
                                QuerySymbol::ComparisonOperatorType(
                                    ComparisonOperatorType::GreaterEquals,
                                )
                            }
                            ComparisonOperatorType::Less => {
                                QuerySymbol::ComparisonOperatorType(ComparisonOperatorType::Less)
                            }
                            ComparisonOperatorType::LessEquals => {
                                QuerySymbol::ComparisonOperatorType(
                                    ComparisonOperatorType::LessEquals,
                                )
                            }
                        }
                    }
                },
                None => return Err("Inferred type should not be None here!".into()),
            };

            head_index += first_capture.as_str().len();

            output.append(extracted_symbol);
        }
    }

    Ok(output)
}
