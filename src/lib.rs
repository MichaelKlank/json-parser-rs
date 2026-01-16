/// Professional Rust JSON Parser
///
/// This module demonstrates professional Rust development practices:
/// - Proper error handling with custom error types
/// - Type-safe data structures using enums
/// - Separation of concerns (lexer, parser, error handling)
/// - Comprehensive documentation
/// - Iterator-based parsing
pub mod error;
pub mod json;
pub mod lexer;
pub mod parser;

pub use error::ParseError;
pub use json::JsonValue;
pub use parser::Parser;

/// Parse a JSON string into a JsonValue
///
/// # Examples
///
/// ```
/// use json_parser_rs::parse_json;
///
/// let json = r#"{"key": "value"}"#;
/// let value = parse_json(json).unwrap();
/// ```
pub fn parse_json(input: &str) -> Result<JsonValue, ParseError> {
    let mut parser = Parser::new(input)?;
    parser.parse()
}
