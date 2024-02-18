use std::{error::Error, fmt::Debug};

pub struct TokenSupplier<Token>
where
    Token: Debug,
{
    tokens: Vec<Token>,
    head: usize,
}

impl<Token> TokenSupplier<Token>
where
    Token: Debug,
{
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, head: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.head)
    }

    pub fn consume(&mut self) -> Result<&Token, Box<dyn Error>> {
        let result = self.tokens.get(self.head);
        println!("Consumed {:?}", result);
        self.head += 1;
        match result {
            Some(result) => Ok(result),
            None => Err("Token-stream ended unexpectedly!".into()),
        }
    }

    pub fn get(&self) -> Result<&Token, Box<dyn Error>> {
        match self.tokens.get(self.head) {
            Some(result) => Ok(result),
            None => Err("Token-stream ended unexpectedly!".into()),
        }
    }

    pub fn consume_with_assert<TokenType>(
        &mut self,
        token_type: TokenType,
    ) -> Result<&Token, Box<dyn Error>>
    where
        TokenType: for<'a> From<&'a Token> + Debug + PartialEq,
    {
        let current_token = self.consume()?;

        if TokenType::from(current_token) == token_type {
            Ok(current_token)
        } else {
            Err(format!(
                "Expected a {:?} token but found {:?}!",
                token_type, current_token
            )
            .into())
        }
    }
}
