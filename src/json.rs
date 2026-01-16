/// JSON value representation
/// 
/// Professional Rust developers use enums for representing data structures
/// because they provide:
/// 1. Type safety - compiler ensures all cases are handled
/// 2. Pattern matching - elegant handling of different value types
/// 3. Memory efficiency - enum variants share the same memory space
/// 4. No runtime type errors - types are checked at compile time

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    /// JSON null value
    Null,
    /// JSON boolean value
    Boolean(bool),
    /// JSON number (using f64 to handle both integers and floats)
    Number(f64),
    /// JSON string value
    String(String),
    /// JSON array
    Array(Vec<JsonValue>),
    /// JSON object (using Vec for simplicity, HashMap would be better for lookups)
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    /// Format JSON value back to JSON string (for debugging/output)
    pub fn to_json_string(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Boolean(b) => b.to_string(),
            JsonValue::Number(n) => {
                // Format numbers without unnecessary decimal points
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    n.to_string()
                }
            }
            JsonValue::String(s) => format!("\"{}\"", escape_string(s)),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_json_string()).collect();
                format!("[{}]", items.join(", "))
            }
            JsonValue::Object(obj) => {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", escape_string(k), v.to_json_string()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }
}

fn escape_string(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '"' => vec!['\\', '"'],
            '\\' => vec!['\\', '\\'],
            '\n' => vec!['\\', 'n'],
            '\r' => vec!['\\', 'r'],
            '\t' => vec!['\\', 't'],
            _ => vec![c],
        })
        .collect()
}
