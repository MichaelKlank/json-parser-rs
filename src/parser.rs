/// Parser - converts tokens into JSON value tree
/// 
/// Professional Rust developers use recursive descent parsing because:
/// 1. Easy to understand and maintain
/// 2. Directly mirrors the grammar structure
/// 3. Good error messages - can pinpoint exact location
/// 4. No external dependencies needed

use crate::error::ParseError;
use crate::json::JsonValue;
use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token()?;
        let peek_token = if current_token != Token::Eof {
            Some(lexer.next_token().unwrap_or(Token::Eof))
        } else {
            Some(Token::Eof)
        };

        Ok(Self {
            lexer,
            current_token,
            peek_token,
        })
    }

    fn advance(&mut self) -> Result<(), ParseError> {
        self.current_token = self.peek_token.take().unwrap_or(Token::Eof);
        if self.current_token != Token::Eof {
            self.peek_token = Some(self.lexer.next_token().unwrap_or(Token::Eof));
        } else {
            self.peek_token = Some(Token::Eof);
        }
        Ok(())
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        if std::mem::discriminant(&self.current_token) == std::mem::discriminant(&expected) {
            self.advance()?;
            Ok(())
        } else {
            Err(ParseError::new(
                format!(
                    "Expected {:?}, found {:?}",
                    expected, self.current_token
                ),
                self.lexer.position,
                self.lexer.input,
            ))
        }
    }

    /// Parse JSON value from input
    pub fn parse(&mut self) -> Result<JsonValue, ParseError> {
        let value = self.parse_value()?;
        
        // Ensure we've consumed all input
        if self.current_token != Token::Eof {
            return Err(ParseError::new(
                format!("Unexpected token after JSON value: {:?}", self.current_token),
                self.lexer.position,
                self.lexer.input,
            ));
        }
        
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<JsonValue, ParseError> {
        match &self.current_token {
            Token::String(s) => {
                let value = JsonValue::String(s.clone());
                self.advance()?;
                Ok(value)
            }
            Token::Number(n) => {
                let value = JsonValue::Number(*n);
                self.advance()?;
                Ok(value)
            }
            Token::Boolean(b) => {
                let value = JsonValue::Boolean(*b);
                self.advance()?;
                Ok(value)
            }
            Token::Null => {
                self.advance()?;
                Ok(JsonValue::Null)
            }
            Token::LeftBrace => self.parse_object(),
            Token::LeftBracket => self.parse_array(),
            _ => Err(ParseError::new(
                format!("Unexpected token: {:?}", self.current_token),
                self.lexer.position,
                self.lexer.input,
            )),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, ParseError> {
        self.expect_token(Token::LeftBrace)?;

        let mut pairs = Vec::new();

        // Handle empty object
        if matches!(self.current_token, Token::RightBrace) {
            self.advance()?;
            return Ok(JsonValue::Object(pairs));
        }

        loop {
            // Parse key (must be a string)
            let key = match &self.current_token {
                Token::String(s) => {
                    let key = s.clone();
                    self.advance()?;
                    key
                }
                _ => {
                    return Err(ParseError::new(
                        "Object key must be a string",
                        self.lexer.position,
                        self.lexer.input,
                    ))
                }
            };

            // Expect colon
            self.expect_token(Token::Colon)?;

            // Parse value
            let value = self.parse_value()?;
            pairs.push((key, value));

            // Check for comma or closing brace
            match self.current_token {
                Token::Comma => {
                    self.advance()?;
                    // Check for trailing comma
                    if matches!(self.current_token, Token::RightBrace) {
                        return Err(ParseError::new(
                            "Trailing comma not allowed",
                            self.lexer.position,
                            self.lexer.input,
                        ));
                    }
                }
                Token::RightBrace => {
                    self.advance()?;
                    break;
                }
                _ => {
                    return Err(ParseError::new(
                        format!("Expected ',' or '}}', found {:?}", self.current_token),
                        self.lexer.position,
                        self.lexer.input,
                    ))
                }
            }
        }

        Ok(JsonValue::Object(pairs))
    }

    fn parse_array(&mut self) -> Result<JsonValue, ParseError> {
        self.expect_token(Token::LeftBracket)?;

        let mut elements = Vec::new();

        // Handle empty array
        if matches!(self.current_token, Token::RightBracket) {
            self.advance()?;
            return Ok(JsonValue::Array(elements));
        }

        loop {
            // Parse element
            let element = self.parse_value()?;
            elements.push(element);

            // Check for comma or closing bracket
            match self.current_token {
                Token::Comma => {
                    self.advance()?;
                    // Check for trailing comma
                    if matches!(self.current_token, Token::RightBracket) {
                        return Err(ParseError::new(
                            "Trailing comma not allowed",
                            self.lexer.position,
                            self.lexer.input,
                        ));
                    }
                }
                Token::RightBracket => {
                    self.advance()?;
                    break;
                }
                _ => {
                    return Err(ParseError::new(
                        format!("Expected ',' or ']', found {:?}", self.current_token),
                        self.lexer.position,
                        self.lexer.input,
                    ))
                }
            }
        }

        Ok(JsonValue::Array(elements))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_object() {
        let mut parser = Parser::new("{}").unwrap();
        let result = parser.parse().unwrap();
        assert_eq!(result, JsonValue::Object(vec![]));
    }

    #[test]
    fn test_parse_simple_object() {
        let mut parser = Parser::new(r#"{"key": "value"}"#).unwrap();
        let result = parser.parse().unwrap();
        match result {
            JsonValue::Object(pairs) => {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0, "key");
                assert_eq!(pairs[0].1, JsonValue::String("value".to_string()));
            }
            _ => panic!("Expected object"),
        }
    }
}
