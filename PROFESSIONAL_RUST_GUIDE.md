# Professional Rust Development Guide: JSON Parser

## Übersicht: Wie ein Profi-Rust-Entwickler vorgeht

Dieses Dokument erklärt Schritt für Schritt, wie ein professioneller Rust-Entwickler einen JSON-Parser implementieren würde und worauf du in Zukunft achten solltest.

---

## 🏗️ Architektur: Separation of Concerns

### 1. **Modulare Struktur**

Ein Profi teilt Code in logische Module auf:

```
src/
├── lib.rs          # Öffentliche API und Modulexport
├── main.rs         # CLI-Einstiegspunkt
├── error.rs        # Fehlerbehandlung
├── json.rs         # Datentypen (JSON-Werte)
├── lexer.rs        # Tokenisierung (Lexer)
└── parser.rs       # Parsing (Parser)
```

**Warum?**
- **Single Responsibility**: Jedes Modul hat eine klare Aufgabe
- **Testbarkeit**: Module können isoliert getestet werden
- **Wiederverwendbarkeit**: Lexer/Parser können in anderen Projekten genutzt werden
- **Wartbarkeit**: Änderungen sind lokalisiert

### 2. **Library vs. Binary**

Profis trennen Library-Code (`lib.rs`) von CLI-Code (`main.rs`):

```rust
// lib.rs - kann von anderen Projekten genutzt werden
pub fn parse_json(input: &str) -> Result<JsonValue, ParseError> { ... }

// main.rs - nur für CLI
fn main() { ... }
```

**Warum?**
- Library kann in Tests, Benchmarks, anderen Tools genutzt werden
- Bessere Testbarkeit
- Kann als Dependency in anderen Projekten verwendet werden

---

## 🎯 Type Safety: Enums statt Strings

### Dein alter Ansatz:
```rust
fn extract_key_value_pair(content: &str) -> Result<Vec<(String, String)>, &'static str>
```

### Professioneller Ansatz:
```rust
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}
```

**Vorteile:**
1. **Compile-Time Type Safety**: Der Compiler prüft, dass alle Fälle behandelt werden
2. **Pattern Matching**: Elegante Behandlung verschiedener Werttypen
3. **Keine Runtime-Fehler**: Keine "ist das ein String oder eine Zahl?"-Probleme
4. **Dokumentation**: Der Typ dokumentiert selbst, welche Werte möglich sind

**Beispiel Pattern Matching:**
```rust
match json_value {
    JsonValue::String(s) => println!("String: {}", s),
    JsonValue::Number(n) => println!("Number: {}", n),
    JsonValue::Boolean(b) => println!("Boolean: {}", b),
    // Compiler warnt, wenn ein Fall fehlt!
}
```

---

## 🔍 Lexer/Parser Trennung

### Dein alter Ansatz:
- Alles in einer Funktion: String-Splitting, Parsing, Validierung vermischt
- Schwer zu debuggen
- Keine klare Fehlerpositionen

### Professioneller Ansatz:

#### **Lexer (Tokenisierung)**
Konvertiert Input-String in Tokens:

```rust
// Input:  {"key": "value"}
// Output: [LeftBrace, String("key"), Colon, String("value"), RightBrace]
```

**Warum getrennt?**
- **Einfacheres Debugging**: "Token X ist falsch" statt "irgendwo im String ist was falsch"
- **Wiederverwendbarkeit**: Lexer kann für verschiedene Parser genutzt werden
- **Bessere Fehlermeldungen**: Genau wissen, welches Token das Problem ist

#### **Parser (Syntaktische Analyse)**
Konvertiert Tokens in JSON-Wert-Baum:

```rust
// Input:  [LeftBrace, String("key"), Colon, String("value"), RightBrace]
// Output: JsonValue::Object(vec![("key".to_string(), JsonValue::String("value".to_string()))])
```

**Recursive Descent Parsing:**
- Direktes Abbild der JSON-Grammatik
- Einfach zu verstehen und zu erweitern
- Gute Fehlermeldungen möglich

---

## ⚠️ Error Handling: Custom Error Types

### Dein alter Ansatz:
```rust
Result<&str, &'static str>  // Keine Kontext-Informationen
```

### Professioneller Ansatz:
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
    pub line: usize,
    pub column: usize,
}
```

**Vorteile:**
1. **Kontext**: Zeile, Spalte, Position für Debugging
2. **Type Safety**: Keine String-Literale, die sich ändern können
3. **Erweiterbarkeit**: Kann später weitere Felder hinzufügen (z.B. Error-Codes)
4. **Display-Trait**: Schöne Fehlerausgabe

**Beispiel:**
```
Parse error at line 7, column 14: Unexpected token: Eof
```

Statt:
```
Invalid JSON format
```

---

## 🔄 Iterator Pattern

### Dein alter Ansatz:
```rust
let key_values = content.split(",").collect::<Vec<&str>>();
```

### Professioneller Ansatz:
```rust
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn next_token(&mut self) -> Result<Token, ParseError> { ... }
}
```

**Vorteile:**
1. **Lazy Evaluation**: Nur verarbeiten, was benötigt wird
2. **Memory Efficient**: Keine großen Vektoren im Speicher
3. **Composable**: Kann mit anderen Iteratoren kombiniert werden
4. **Rust-Idiomatisch**: Nutzt Rust's Stärken

---

## 📝 Best Practices: Was du in Zukunft beachten solltest

### 1. **Lifetimes verstehen**
```rust
pub struct Lexer<'a> {  // 'a = Lifetime-Parameter
    input: &'a str,      // Referenz auf Input, der mindestens so lange lebt wie Lexer
}
```

**Warum wichtig?**
- Verhindert Dangling References
- Compiler prüft zur Compile-Zeit
- Keine Runtime-Kosten

### 2. **Result statt unwrap()**
```rust
// ❌ Schlecht
let value = parse_json(input).unwrap();

// ✅ Gut
match parse_json(input) {
    Ok(value) => { /* handle success */ },
    Err(e) => { /* handle error */ },
}
```

**Warum?**
- `unwrap()` crasht das Programm bei Fehlern
- `Result` zwingt dich, Fehler zu behandeln
- Bessere User Experience

### 3. **Dokumentation mit `///`**
```rust
/// Parse a JSON string into a JsonValue
/// 
/// # Examples
/// 
/// ```
/// use json_parser_rs::parse_json;
/// let json = r#"{"key": "value"}"#;
/// let value = parse_json(json).unwrap();
/// ```
pub fn parse_json(input: &str) -> Result<JsonValue, ParseError>
```

**Warum?**
- `cargo doc` generiert automatisch Dokumentation
- Beispiele werden getestet (`cargo test --doc`)
- Bessere IDE-Unterstützung

### 4. **Tests strukturieren**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_empty_object() {
        let mut parser = Parser::new("{}").unwrap();
        let result = parser.parse().unwrap();
        assert_eq!(result, JsonValue::Object(vec![]));
    }
}
```

**Warum?**
- Unit Tests für einzelne Funktionen
- Integration Tests für End-to-End
- `#[cfg(test)]` kompiliert nur bei Tests

### 5. **Minimale Allokationen**
```rust
// ❌ Schlecht: Mehrfache Allokationen
let key = key_value_pair[0].trim().replace('"', "").to_string();

// ✅ Gut: Nur eine Allokation
let key = key_value_pair[0]
    .trim()
    .strip_prefix('"')
    .and_then(|s| s.strip_suffix('"'))
    .unwrap_or(key_value_pair[0].trim())
    .to_string();
```

**Warum?**
- Bessere Performance
- Weniger Memory-Overhead
- Wichtig für Hot Paths

### 6. **Pattern Matching statt if/else**
```rust
// ❌ Schlecht
if value == "true" { true }
else if value == "false" { false }
else { ... }

// ✅ Gut
match value {
    "true" => Token::Boolean(true),
    "false" => Token::Boolean(false),
    "null" => Token::Null,
    _ => return Err(...),
}
```

**Warum?**
- Exhaustive: Compiler prüft alle Fälle
- Klarer und lesbarer
- Bessere Performance (oft optimiert zu Jump-Tables)

### 7. **Module und Visibility**
```rust
// Öffentlich für andere Module
pub fn parse_json(...) { ... }

// Nur innerhalb des Moduls
fn advance(&mut self) { ... }

// Nur für Tests
#[cfg(test)]
mod tests { ... }
```

**Warum?**
- Klare API-Grenzen
- Verhindert ungewollte Nutzung
- Bessere Encapsulation

---

## 🚀 Performance-Optimierungen (für später)

### 1. **String Interning**
Für wiederholte Keys in großen JSON-Dateien:
```rust
use std::collections::HashMap;

struct StringInterner {
    map: HashMap<String, usize>,
    strings: Vec<String>,
}
```

### 2. **Zero-Copy Parsing**
Für sehr große Dateien:
```rust
// Statt String zu kopieren, Referenzen nutzen
pub enum JsonValue<'a> {
    String(&'a str),  // Referenz statt owned String
    // ...
}
```

### 3. **SIMD für Lexing**
Für sehr große Dateien können SIMD-Instruktionen helfen, Whitespace schneller zu überspringen.

---

## 📚 Weiterführende Ressourcen

1. **Rust Book**: https://doc.rust-lang.org/book/
2. **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
3. **Error Handling**: https://doc.rust-lang.org/book/ch09-00-error-handling.html
4. **Lifetimes**: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
5. **Iterator Pattern**: https://doc.rust-lang.org/book/ch13-02-iterators.html

---

## 🎓 Zusammenfassung: Die wichtigsten Punkte

1. ✅ **Type Safety**: Enums statt Strings
2. ✅ **Error Handling**: Custom Error Types mit Kontext
3. ✅ **Separation of Concerns**: Lexer/Parser getrennt
4. ✅ **Iterator Pattern**: Lazy, memory-efficient
5. ✅ **Dokumentation**: `///` für öffentliche APIs
6. ✅ **Tests**: Unit + Integration Tests
7. ✅ **Module Structure**: Klare API-Grenzen
8. ✅ **Pattern Matching**: Statt if/else-Ketten

**Dein Code funktioniert - das ist super!** Aber mit diesen Prinzipien wird er:
- Wartbarer
- Testbarer
- Performanter
- Professioneller
- Team-tauglich

Viel Erfolg beim weiteren Lernen! 🦀
