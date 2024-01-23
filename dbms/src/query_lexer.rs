use std::{error::Error, vec};

use crate::value_type::Value;

pub trait QueryLexer {
    fn lex_string(&self, input: &str) -> Result<QuerySymbolStream, Box<dyn Error>>;
}

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
    TableName(String),
    ColumnName(String),
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
