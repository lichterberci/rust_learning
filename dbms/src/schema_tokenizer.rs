use std::error::Error;

use regex::Match;

use crate::{rel_alg_ast::Identifier, tokenizer};

#[derive(Debug, PartialEq)]
pub enum SchemaToken {
    Identifier(String),
    Type(Type),
    CurlyBrace(CurlyBraceType),
    Key,
    Colon,
    Comma,
    Ordered,
    Primary,
    Secondary,
    None,
    Index,
    Single,
    Multi,
    Level,
    Hash,
}

impl SchemaToken {
    pub fn get_type(&self) -> SchemaTokenType {
        SchemaTokenType::from(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SchemaTokenType {
    Identifier,
    Type(Type),
    CurlyBrace(CurlyBraceType),
    Key,
    Colon,
    Comma,
    Ordered,
    Primary,
    Secondary,
    None,
    Index,
    Single,
    Multi,
    Level,
    Hash,
}

impl From<&SchemaToken> for SchemaTokenType {
    fn from(value: &SchemaToken) -> Self {
        match value {
            SchemaToken::Identifier(_) => SchemaTokenType::Identifier,
            SchemaToken::Type(att_type) => SchemaTokenType::Type(*att_type),
            SchemaToken::CurlyBrace(curly_brace_type) => {
                SchemaTokenType::CurlyBrace(*curly_brace_type)
            }
            SchemaToken::Key => SchemaTokenType::Key,
            SchemaToken::Colon => SchemaTokenType::Colon,
            SchemaToken::Comma => SchemaTokenType::Comma,
            SchemaToken::Ordered => SchemaTokenType::Ordered,
            SchemaToken::Primary => SchemaTokenType::Primary,
            SchemaToken::Secondary => SchemaTokenType::Secondary,
            SchemaToken::None => SchemaTokenType::None,
            SchemaToken::Index => SchemaTokenType::Index,
            SchemaToken::Single => SchemaTokenType::Single,
            SchemaToken::Multi => SchemaTokenType::Multi,
            SchemaToken::Level => SchemaTokenType::Level,
            SchemaToken::Hash => SchemaTokenType::Hash,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurlyBraceType {
    Opening,
    Closing,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
}

pub struct SchemaTokenizer {
    tokenizer: tokenizer::Tokenizer<SchemaToken, SchemaTokenType>,
}

impl SchemaTokenizer {
    pub fn new() -> Self {
        let mut grammar: Vec<(String, Option<SchemaTokenType>)> = Vec::new();

        grammar.push((r"^(?<token>\s+)".into(), Option::None)); // whitespace
        grammar.push((r"^(?<token>;\-\-[^\n]*)".into(), Option::None)); // line comment
        grammar.push((
            r"^(?<token>int)[^\w]".into(),
            Option::Some(SchemaTokenType::Type(Type::Int)),
        ));
        grammar.push((
            r"^(?<token>float)[^\w]".into(),
            Option::Some(SchemaTokenType::Type(Type::Float)),
        ));
        grammar.push((
            r"^(?<token>string)[^\w]".into(),
            Option::Some(SchemaTokenType::Type(Type::String)),
        ));
        grammar.push((
            r"^(?<token>bool)[^\w]".into(),
            Option::Some(SchemaTokenType::Type(Type::Bool)),
        ));
        grammar.push((
            r"^(?<token>key)[^\w]".into(),
            Option::Some(SchemaTokenType::Key),
        ));
        grammar.push((r"^(?<token>,)".into(), Option::Some(SchemaTokenType::Comma)));
        grammar.push((r"^(?<token>:)".into(), Option::Some(SchemaTokenType::Colon)));
        grammar.push((
            r"^(?<token>\{)".into(),
            Option::Some(SchemaTokenType::CurlyBrace(CurlyBraceType::Opening)),
        ));
        grammar.push((
            r"^(?<token>\})".into(),
            Option::Some(SchemaTokenType::CurlyBrace(CurlyBraceType::Closing)),
        ));
        grammar.push((
            r"^(?<token>primary)[^\w]".into(),
            Option::Some(SchemaTokenType::Primary),
        ));
        grammar.push((
            r"^(?<token>secondary)[^\w]".into(),
            Option::Some(SchemaTokenType::Secondary),
        ));
        grammar.push((
            r"^(?<token>none)[^\w]".into(),
            Option::Some(SchemaTokenType::None),
        ));
        grammar.push((
            r"^(?<token>index)[^\w]".into(),
            Option::Some(SchemaTokenType::Index),
        ));
        grammar.push((
            r"^(?<token>ordered)[^\w]".into(),
            Option::Some(SchemaTokenType::Ordered),
        ));
        grammar.push((
            r"^(?<token>single)[^\w]".into(),
            Option::Some(SchemaTokenType::Single),
        ));
        grammar.push((
            r"^(?<token>multi)[^\w]".into(),
            Option::Some(SchemaTokenType::Multi),
        ));
        grammar.push((
            r"^(?<token>level)[^\w]".into(),
            Option::Some(SchemaTokenType::Level),
        ));
        grammar.push((
            r"^(?<token>hash)[^\w]".into(),
            Option::Some(SchemaTokenType::Hash),
        ));
        grammar.push((
            r"^(?<token>[\w_]+)[^\w_]".into(),
            Option::Some(SchemaTokenType::Identifier),
        ));

        let matching_function = Box::new(
            |captured_group_of_token: &Match,
             inferred_type: &SchemaTokenType|
             -> Result<SchemaToken, Box<dyn Error>> {
                Ok(match inferred_type {
                    SchemaTokenType::Identifier => {
                        SchemaToken::Identifier(captured_group_of_token.as_str().into())
                    }
                    SchemaTokenType::Type(att_type) => SchemaToken::Type(*att_type),
                    SchemaTokenType::CurlyBrace(brace_type) => SchemaToken::CurlyBrace(*brace_type),
                    SchemaTokenType::Key => SchemaToken::Key,
                    SchemaTokenType::Colon => SchemaToken::Colon,
                    SchemaTokenType::Comma => SchemaToken::Comma,
                    SchemaTokenType::Ordered => SchemaToken::Ordered,
                    SchemaTokenType::Primary => SchemaToken::Primary,
                    SchemaTokenType::Secondary => SchemaToken::Secondary,
                    SchemaTokenType::None => SchemaToken::None,
                    SchemaTokenType::Index => SchemaToken::Index,
                    SchemaTokenType::Single => SchemaToken::Single,
                    SchemaTokenType::Multi => SchemaToken::Multi,
                    SchemaTokenType::Level => SchemaToken::Level,
                    SchemaTokenType::Hash => SchemaToken::Hash,
                })
            },
        );

        Self {
            tokenizer: tokenizer::Tokenizer::build(&grammar, matching_function, "token"),
        }
    }

    pub fn tokenize(&self, input: &str) -> Result<Vec<SchemaToken>, Box<dyn Error>> {
        self.tokenizer.tokenize(input)
    }
}
