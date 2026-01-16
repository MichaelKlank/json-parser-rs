/// Lexer (Tokenizer) - converts input string into tokens
/// 
/// Professional Rust developers separate lexing from parsing because:
/// 1. Single Responsibility Principle - each component has one job
/// 2. Easier to test and debug
/// 3. Can reuse lexer for different parsers
/// 4. Better error messages - know exactly which token caused the issue

use crate::error::ParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Structural tokens
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Colon,        // :
    
    // Value tokens
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    
    // End of input
    Eof,
}

/// Lexer that converts input string into tokens
/// 
/// Uses iterator pattern - professional Rust developers prefer iterators
/// because they are:
/// 1. Lazy - only process what you need
/// 2. Composable - can chain operations
/// 3. Memory efficient - no need to store all tokens at once
pub struct Lexer<'a> {
    pub input: &'a str,
    pub position: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.current_char = self.input[self.position..].chars().next();
            self.position += self.current_char.map(|c| c.len_utf8()).unwrap_or(0);
        } else {
            self.current_char = None;
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Read the next token from input
    pub fn next_token(&mut self) -> Result<Token, ParseError> {
        self.skip_whitespace();

        let start_pos = self.position;

        match self.current_char {
            None => Ok(Token::Eof),
            Some('{') => {
                self.advance();
                Ok(Token::LeftBrace)
            }
            Some('}') => {
                self.advance();
                Ok(Token::RightBrace)
            }
            Some('[') => {
                self.advance();
                Ok(Token::LeftBracket)
            }
            Some(']') => {
                self.advance();
                Ok(Token::RightBracket)
            }
            Some(',') => {
                self.advance();
                Ok(Token::Comma)
            }
            Some(':') => {
                self.advance();
                Ok(Token::Colon)
            }
            Some('"') => self.read_string(),
            Some(ch) if ch.is_ascii_digit() || ch == '-' => self.read_number(),
            Some(ch) if ch.is_alphabetic() => self.read_keyword(),
            Some(ch) => Err(ParseError::new(
                format!("Unexpected character: '{}'", ch),
                start_pos,
                self.input,
            )),
        }
    }

    fn read_string(&mut self) -> Result<Token, ParseError> {
        let start_pos = self.position;
        self.advance(); // Skip opening quote

        let mut string = String::new();
        let mut escaped = false;

        while let Some(ch) = self.current_char {
            if escaped {
                match ch {
                    '"' => string.push('"'),
                    '\\' => string.push('\\'),
                    'n' => string.push('\n'),
                    'r' => string.push('\r'),
                    't' => string.push('\t'),
                    _ => {
                        return Err(ParseError::new(
                            format!("Invalid escape sequence: \\{}", ch),
                            self.position,
                            self.input,
                        ))
                    }
                }
                escaped = false;
                self.advance();
            } else if ch == '\\' {
                escaped = true;
                self.advance();
            } else if ch == '"' {
                self.advance(); // Skip closing quote
                return Ok(Token::String(string));
            } else {
                string.push(ch);
                self.advance();
            }
        }

        Err(ParseError::new(
            "Unterminated string",
            start_pos,
            self.input,
        ))
    }

    fn read_number(&mut self) -> Result<Token, ParseError> {
        let start_pos = self.position;
        let mut number_str = String::new();
        let mut has_dot = false;

        // Handle negative sign
        if self.current_char == Some('-') {
            number_str.push('-');
            self.advance();
        }

        // Read digits before decimal point
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                number_str.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot {
                number_str.push('.');
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        // Read digits after decimal point
        if has_dot {
            while let Some(ch) = self.current_char {
                if ch.is_ascii_digit() {
                    number_str.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        number_str
            .parse::<f64>()
            .map(Token::Number)
            .map_err(|_| ParseError::new("Invalid number", start_pos, self.input))
    }

    fn read_keyword(&mut self) -> Result<Token, ParseError> {
        let start_pos = self.position;
        let mut keyword = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() {
                keyword.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match keyword.as_str() {
            "true" => Ok(Token::Boolean(true)),
            "false" => Ok(Token::Boolean(false)),
            "null" => Ok(Token::Null),
            _ => Err(ParseError::new(
                format!("Unexpected keyword: {}", keyword),
                start_pos,
                self.input,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_basic_tokens() {
        let mut lexer = Lexer::new("{}[],:");
        assert_eq!(lexer.next_token().unwrap(), Token::LeftBrace);
        assert_eq!(lexer.next_token().unwrap(), Token::RightBrace);
        assert_eq!(lexer.next_token().unwrap(), Token::LeftBracket);
        assert_eq!(lexer.next_token().unwrap(), Token::RightBracket);
        assert_eq!(lexer.next_token().unwrap(), Token::Comma);
        assert_eq!(lexer.next_token().unwrap(), Token::Colon);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_lexer_string() {
        let mut lexer = Lexer::new(r#""hello world""#);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::String("hello world".to_string())
        );
    }

    #[test]
    fn test_lexer_number() {
        let mut lexer = Lexer::new("123");
        assert_eq!(lexer.next_token().unwrap(), Token::Number(123.0));

        let mut lexer = Lexer::new("-42.5");
        assert_eq!(lexer.next_token().unwrap(), Token::Number(-42.5));
    }

    #[test]
    fn test_lexer_keywords() {
        let mut lexer = Lexer::new("true");
        assert_eq!(lexer.next_token().unwrap(), Token::Boolean(true));

        let mut lexer = Lexer::new("false");
        assert_eq!(lexer.next_token().unwrap(), Token::Boolean(false));

        let mut lexer = Lexer::new("null");
        assert_eq!(lexer.next_token().unwrap(), Token::Null);
    }
}
