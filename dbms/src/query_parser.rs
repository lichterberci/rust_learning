use std::error::Error;

use crate::{query_lexer, rel_alg_ast};

pub trait QueryParser {
    fn parse(
        &self,
        symbol_stream: &query_lexer::QuerySymbolStream,
    ) -> Result<rel_alg_ast::RelAlgAST, Box<dyn Error>>;
}
