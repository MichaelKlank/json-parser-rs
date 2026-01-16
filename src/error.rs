/// Custom error types for JSON parsing
/// 
/// Professional Rust developers use custom error types instead of string literals
/// because they provide:
/// 1. Type safety
/// 2. Better error messages with context
/// 3. Position information for debugging
/// 4. Ability to chain errors

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
    pub line: usize,
    pub column: usize,
}

impl ParseError {
    pub fn new(message: impl Into<String>, position: usize, input: &str) -> Self {
        let (line, column) = Self::calculate_position(position, input);
        Self {
            message: message.into(),
            position,
            line,
            column,
        }
    }

    fn calculate_position(pos: usize, input: &str) -> (usize, usize) {
        let before = &input[..pos.min(input.len())];
        let line = before.matches('\n').count() + 1;
        let column = before
            .rfind('\n')
            .map(|i| pos - i - 1)
            .unwrap_or(pos)
            + 1;
        (line, column)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parse error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for ParseError {}
