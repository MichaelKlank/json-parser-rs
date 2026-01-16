# JSON Parser - Architektur-Diagramm

## Komponenten-Interaktion

```mermaid
graph TB
    subgraph "CLI Layer"
        MAIN[main.rs<br/>CLI Entry Point]
    end
    
    subgraph "Public API"
        LIB[lib.rs<br/>parse_json<br/>Public Function]
    end
    
    subgraph "Parsing Layer"
        PARSER[parser.rs<br/>Parser<br/>Recursive Descent]
    end
    
    subgraph "Lexical Analysis"
        LEXER[lexer.rs<br/>Lexer<br/>Tokenization]
    end
    
    subgraph "Data Types"
        JSON[json.rs<br/>JsonValue Enum<br/>Null, Boolean, Number,<br/>String, Array, Object]
    end
    
    subgraph "Error Handling"
        ERROR[error.rs<br/>ParseError<br/>Position, Line, Column]
    end
    
    %% Data Flow
    MAIN -->|"1. Read file<br/>2. Call parse_json()"| LIB
    LIB -->|"3. Create Parser<br/>4. Parse input"| PARSER
    PARSER -->|"5. Request tokens<br/>next_token()"| LEXER
    LEXER -->|"6. Return Token<br/>(String, Number, etc.)"| PARSER
    PARSER -->|"7. Build JsonValue<br/>tree structure"| JSON
    PARSER -->|"8. Return JsonValue"| LIB
    LIB -->|"9. Return Result"| MAIN
    
    %% Error Flow
    LEXER -.->|"On error"| ERROR
    PARSER -.->|"On error"| ERROR
    ERROR -.->|"Error Result"| PARSER
    PARSER -.->|"Error Result"| LIB
    LIB -.->|"Error Result"| MAIN
    
    style MAIN fill:#e1f5ff
    style LIB fill:#fff4e1
    style PARSER fill:#e8f5e9
    style LEXER fill:#f3e5f5
    style JSON fill:#fce4ec
    style ERROR fill:#ffebee
```

## Detaillierter Datenfluss

```mermaid
sequenceDiagram
    participant User
    participant Main as main.rs
    participant Lib as lib.rs
    participant Parser as parser.rs
    participant Lexer as lexer.rs
    participant Json as json.rs
    participant Error as error.rs
    
    User->>Main: ./json-parser-rs file.json
    Main->>Main: Read file content
    Main->>Lib: parse_json(&content)
    
    Lib->>Parser: Parser::new(input)
    Parser->>Lexer: Lexer::new(input)
    Lexer-->>Parser: Lexer instance
    
    loop Parse JSON
        Parser->>Lexer: next_token()
        Lexer->>Lexer: Skip whitespace
        Lexer->>Lexer: Read character
        alt String token
            Lexer->>Lexer: Read until closing quote
            Lexer-->>Parser: Token::String("value")
        else Number token
            Lexer->>Lexer: Read digits
            Lexer-->>Parser: Token::Number(123.0)
        else Structural token
            Lexer-->>Parser: Token::LeftBrace, etc.
        else Error
            Lexer->>Error: Create ParseError
            Error-->>Parser: Err(ParseError)
        end
        
        alt Valid token
            Parser->>Parser: Match token type
            alt Object
                Parser->>Parser: parse_object()
                Parser->>Parser: Recursive parse_value()
            else Array
                Parser->>Parser: parse_array()
                Parser->>Parser: Recursive parse_value()
            else Value
                Parser->>Json: Create JsonValue
                Json-->>Parser: JsonValue instance
            end
        end
    end
    
    Parser->>Json: Build JsonValue tree
    Json-->>Parser: JsonValue::Object(...)
    Parser-->>Lib: Ok(JsonValue)
    Lib-->>Main: Ok(JsonValue)
    Main->>Main: Exit code 0
    Main-->>User: Success
```

## Token-Flow Beispiel

```
Input: {"key": "value"}

┌─────────────────────────────────────────────────────────┐
│ Lexer: Character-by-character processing                │
└─────────────────────────────────────────────────────────┘
         │
         ▼
    '{' → Token::LeftBrace
    '"' → Token::String("key")
    ':' → Token::Colon
    '"' → Token::String("value")
    '}' → Token::RightBrace
    EOF → Token::Eof

┌─────────────────────────────────────────────────────────┐
│ Parser: Token-by-token processing                       │
└─────────────────────────────────────────────────────────┘
         │
         ▼
    LeftBrace → Start parse_object()
    String("key") → Key
    Colon → Expect colon
    String("value") → Value → JsonValue::String("value")
    RightBrace → End parse_object()
    
┌─────────────────────────────────────────────────────────┐
│ Result: JsonValue tree                                  │
└─────────────────────────────────────────────────────────┘
         │
         ▼
    JsonValue::Object(
        vec![
            ("key".to_string(), JsonValue::String("value".to_string()))
        ]
    )
```

## Modul-Abhängigkeiten

```mermaid
graph LR
    subgraph "Core Modules"
        JSON[json.rs]
        ERROR[error.rs]
    end
    
    subgraph "Processing Modules"
        LEXER[lexer.rs]
        PARSER[parser.rs]
    end
    
    subgraph "API Layer"
        LIB[lib.rs]
        MAIN[main.rs]
    end
    
    LEXER --> ERROR
    LEXER --> JSON
    PARSER --> LEXER
    PARSER --> JSON
    PARSER --> ERROR
    LIB --> PARSER
    LIB --> JSON
    LIB --> ERROR
    MAIN --> LIB
    
    style JSON fill:#fce4ec
    style ERROR fill:#ffebee
    style LEXER fill:#f3e5f5
    style PARSER fill:#e8f5e9
    style LIB fill:#fff4e1
    style MAIN fill:#e1f5ff
```

## ASCII-Diagramm (Text-basiert)

```
┌─────────────────────────────────────────────────────────────┐
│                        USER INPUT                            │
│                    {"key": "value"}                         │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  main.rs                                                     │
│  ┌───────────────────────────────────────────────────────┐ │
│  │ 1. Read file content                                    │ │
│  │ 2. Call parse_json(&content)                           │ │
│  └───────────────────────┬─────────────────────────────────┘ │
└──────────────────────────┼───────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│  lib.rs                                                      │
│  ┌───────────────────────────────────────────────────────┐ │
│  │ parse_json(input: &str)                                │ │
│  │   → Parser::new(input)?                                │ │
│  │   → parser.parse()                                     │ │
│  └───────────────────────┬─────────────────────────────────┘ │
└──────────────────────────┼───────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│  parser.rs                                                   │
│  ┌───────────────────────────────────────────────────────┐ │
│  │ Parser::new()                                         │ │
│  │   → Creates Lexer                                     │ │
│  │   → Reads first token                                 │ │
│  │                                                       │ │
│  │ parse()                                               │ │
│  │   → parse_value()                                     │ │
│  │     → parse_object() / parse_array()                 │ │
│  │       → Recursively calls parse_value()              │ │
│  └───────────────────────┬─────────────────────────────────┘ │
└──────────────────────────┼───────────────────────────────────┘
                           │
                           │ Requests tokens
                           ▼
┌─────────────────────────────────────────────────────────────┐
│  lexer.rs                                                    │
│  ┌───────────────────────────────────────────────────────┐ │
│  │ Lexer::new(input)                                     │ │
│  │   → Initialize position tracking                      │ │
│  │                                                       │ │
│  │ next_token()                                          │ │
│  │   → Skip whitespace                                   │ │
│  │   → Match character:                                 │ │
│  │     • '{' → Token::LeftBrace                        │ │
│  │     • '"' → read_string() → Token::String            │ │
│  │     • '0'-'9' → read_number() → Token::Number        │ │
│  │     • 't'/'f'/'n' → read_keyword() → Token::Boolean │ │
│  │   → Return Token or ParseError                       │ │
│  └───────────────────────┬─────────────────────────────────┘ │
└──────────────────────────┼───────────────────────────────────┘
                           │
                           │ Returns Token
                           ▼
┌─────────────────────────────────────────────────────────────┐
│  parser.rs (continued)                                        │
│  ┌───────────────────────────────────────────────────────┐ │
│  │ Builds JsonValue tree from tokens:                    │ │
│  │   Token::String → JsonValue::String                   │ │
│  │   Token::Number → JsonValue::Number                   │ │
│  │   Token::Boolean → JsonValue::Boolean                 │ │
│  │   Token::Null → JsonValue::Null                       │ │
│  │   Object tokens → JsonValue::Object(Vec<...>)         │ │
│  │   Array tokens → JsonValue::Array(Vec<...>)          │ │
│  └───────────────────────┬─────────────────────────────────┘ │
└──────────────────────────┼───────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│  json.rs                                                     │
│  ┌───────────────────────────────────────────────────────┐ │
│  │ JsonValue Enum:                                        │ │
│  │   • Null                                              │ │
│  │   • Boolean(bool)                                     │ │
│  │   • Number(f64)                                       │ │
│  │   • String(String)                                    │ │
│  │   • Array(Vec<JsonValue>)                             │ │
│  │   • Object(Vec<(String, JsonValue)>)                  │ │
│  └───────────────────────────────────────────────────────┘ │
└──────────────────────────┬───────────────────────────────────┘
                           │
                           │ Returns Result<JsonValue, ParseError>
                           ▼
┌─────────────────────────────────────────────────────────────┐
│  error.rs (used throughout)                                  │
│  ┌───────────────────────────────────────────────────────┐ │
│  │ ParseError {                                           │ │
│  │   message: String,                                     │ │
│  │   position: usize,                                     │ │
│  │   line: usize,                                         │ │
│  │   column: usize,                                       │ │
│  │ }                                                      │ │
│  └───────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│  main.rs                                                     │
│  ┌───────────────────────────────────────────────────────┐ │
│  │ Match result:                                          │ │
│  │   Ok(value) → Exit 0 (success)                        │ │
│  │   Err(e) → Print error → Exit 1 (error)              │ │
│  └───────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Wichtige Konzepte

### 1. **Datenfluss (Data Flow)**
- **Top-Down**: main.rs → lib.rs → parser.rs → lexer.rs
- **Bottom-Up**: lexer.rs (Tokens) → parser.rs (JsonValue) → lib.rs (Result) → main.rs

### 2. **Error Propagation**
- Fehler können in jedem Modul entstehen
- Werden als `Result<T, ParseError>` nach oben propagiert
- `?` Operator für automatische Propagation

### 3. **Rekursive Struktur**
- `parse_value()` ruft sich selbst auf für verschachtelte Objekte/Arrays
- Spiegelt die rekursive Natur von JSON wider

### 4. **Lazy Evaluation**
- Lexer liest nur einen Token zur Zeit
- Parser verarbeitet Token für Token
- Keine vollständige Token-Liste im Speicher

### 5. **Type Safety**
- Alle JSON-Werte sind `JsonValue` Enum
- Compiler prüft Vollständigkeit bei Pattern Matching
- Keine Runtime-Type-Checks nötig
