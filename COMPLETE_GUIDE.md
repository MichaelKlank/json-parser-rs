# Complete Rust Development Guide

Ein umfassender Guide für professionelle Rust-Entwicklung, Standard Library Exploration und Best Practices.

---

# Teil 1: Professional Rust Development

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

---

## 🔄 Iterator Pattern

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

## 📝 Best Practices

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

### 4. **Pattern Matching statt if/else**
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

### 5. **Module und Visibility**
```rust
// Öffentlich für andere Module
pub fn parse_json(...) { ... }

// Nur innerhalb des Moduls
fn advance(&mut self) { ... }

// Nur für Tests
#[cfg(test)]
mod tests { ... }
```

---

# Teil 2: Standard Library Exploration

## 📚 Offizielle Dokumentation nutzen

### **doc.rust-lang.org/std**

Die offizielle Standard Library Dokumentation ist dein bester Freund!

**Tipp 1: Direkte Suche**
```
https://doc.rust-lang.org/std/?search=chars
```

**Tipp 2: Nach Typen suchen**
- Du hast einen `&str`? → Suche nach "str" in der Dokumentation
- Du hast einen `Vec<T>`? → Suche nach "Vec"
- Du hast einen `Iterator`? → Suche nach "Iterator"

**Beispiel:**
```rust
let s = "hello";
// Du willst wissen: "Wie iteriere ich über die Zeichen?"
// → Gehe zu: https://doc.rust-lang.org/std/primitive.str.html
// → Schaue unter "Methods" → finde: chars(), bytes(), etc.
```

---

## 🔍 IDE-Features nutzen (VS Code / RustRover)

### **Trick 1: Auto-Complete Exploration**

```rust
let s = "hello";
s.  // ← Tippe hier einen Punkt und warte
    // IDE zeigt dir ALLE verfügbaren Methoden!
```

**Was du siehst:**
- Methoden-Namen
- Signatur (Parameter, Return-Type)
- Kurze Beschreibung
- Dokumentation (wenn du hooverst)

### **Trick 2: "Go to Definition" (F12 / Cmd+Click)**

```rust
let s = "hello";
let chars = s.chars();  // ← Cmd+Click auf "chars"
                        // → Springt zur Definition in std lib
                        // → Siehst alle Details, Dokumentation, Beispiele
```

### **Trick 3: Quick Documentation (Cmd+K, Cmd+I)**

```rust
let s = "hello";
s.chars()  // ← Cursor auf chars, dann Cmd+K, Cmd+I
           // → Popup mit vollständiger Dokumentation
```

---

## 🧪 Rust Playground für Experimente

### **rust-lang.org/play**

**Trick: Schnell testen ohne lokales Projekt**

```rust
fn main() {
    let s = "hello world";
    
    // Experimentiere mit verschiedenen Methoden:
    println!("{:?}", s.chars().collect::<Vec<_>>());
    println!("{:?}", s.split(' ').collect::<Vec<_>>());
    println!("{:?}", s.trim());
}
```

**Vorteile:**
- Keine lokale Installation nötig
- Schnelle Experimente
- Kann Code teilen

---

## 🔎 Pattern: "Ich will X machen mit Y"

### **Systematische Suche**

**Beispiel 1: "Ich will einen String trimmen"**
```
1. Du hast: &str
2. Suche in Doku: https://doc.rust-lang.org/std/primitive.str.html
3. Suche nach "trim" → finde: trim(), trim_start(), trim_end()
4. Oder: Tippe s.trim() in IDE → Auto-Complete zeigt es dir
```

**Beispiel 2: "Ich will über Zeichen iterieren"**
```
1. Du hast: &str
2. Frage: "Wie iteriere ich?"
3. Antwort: Iteratoren! → Suche nach "iterator" oder "iter"
4. Finde: chars(), bytes(), split(), lines()
5. Teste in Playground oder IDE
```

**Beispiel 3: "Ich will einen String parsen"**
```
1. Du hast: &str
2. Frage: "Wie konvertiere ich zu Zahl?"
3. Suche nach: "parse" → finde: parse::<T>()
4. Oder: Suche nach "FromStr" Trait
5. Finde: "123".parse::<i32>()
```

---

## 🧩 Häufige Patterns erkennen

### **Pattern 1: Option/Result Handling**

```rust
// Statt unwrap(), nutze:
let value = option_value?;           // Propagation
let value = option_value.unwrap_or(default);
let value = option_value.ok_or(error)?;
```

**Wo findest du das?**
- Suche nach "Option" in Doku
- Suche nach "Result" in Doku
- IDE zeigt dir alle Methoden bei `option.` oder `result.`

### **Pattern 2: String Manipulation**

```rust
// Statt manuelles Slicing:
let s = "hello";
s.strip_prefix("he")      // Neuere API
s.strip_suffix("lo")
s.trim_start()
s.trim_end()
```

**Wo findest du das?**
- `&str` Doku → "Methods" → Suche nach "strip", "trim"

### **Pattern 3: Collection Operations**

```rust
let vec = vec![1, 2, 3];
vec.contains(&2)          // Prüft Existenz
vec.iter().find(|x| ...)  // Findet Element
vec.iter().position(...)  // Findet Position
```

**Wo findest du das?**
- `Vec<T>` Doku → "Methods"
- `Iterator` Doku → "Provided Methods"

---

# Teil 3: Iterator Navigation

## 🎯 Das Problem: Von chars() zu next()

Du findest `chars()` in der Doku, aber siehst dort nicht `next()`. Wie kommst du zu den richtigen Informationen?

## ✅ Die Lösung: Iterator Traits verstehen

### Das Konzept

```rust
let s = "hello";
s.chars()  // ← Gibt zurück: Chars<'_>
           // Chars ist ein Iterator!
           // next() kommt vom Iterator Trait, nicht von str!
```

**Wichtig:** In Rust kommen viele Methoden von **Traits**, nicht direkt vom Typ!

---

## 📚 Schritt-für-Schritt: In der Doku navigieren

### Schritt 1: chars() in der Doku finden

1. Gehe zu: https://doc.rust-lang.org/std/primitive.str.html
2. Suche nach "chars"
3. Du findest:

```rust
pub fn chars(&self) -> Chars<'_>
```

**Wichtig:** Der Return-Type ist `Chars<'_>` - das ist ein **Iterator**!

### Schritt 2: Zum Iterator Trait navigieren

**Option A: Klicke auf `Chars` in der Doku**
- Die Doku ist verlinkt!
- Klicke auf `Chars` → springt zur Definition
- Dort siehst du: `impl Iterator for Chars`
- Klicke auf `Iterator` → springt zum Iterator Trait

**Option B: Direkt zum Iterator Trait**
1. Gehe zu: https://doc.rust-lang.org/std/iter/trait.Iterator.html
2. Scrolle zu "Required Methods"
3. Dort findest du: `next() -> Option<Self::Item>`

### Schritt 3: Alle Iterator-Methoden sehen

Auf der Iterator-Trait-Seite findest du:
- **Required Methods**: `next()` (muss implementiert sein)
- **Provided Methods**: `map()`, `filter()`, `collect()`, etc. (haben Default-Implementierung)

---

## 💡 Trick 1: IDE zeigt dir alles automatisch

### In VS Code / RustRover:

```rust
let s = "hello";
s.chars().  // ← Tippe hier einen Punkt
```

**IDE zeigt dir SOFORT:**
```
next() -> Option<char>        // ← Kommt vom Iterator Trait
map(...) -> Map<...>          // ← Kommt vom Iterator Trait
filter(...) -> Filter<...>    // ← Kommt vom Iterator Trait
collect() -> Collection       // ← Kommt vom Iterator Trait
take(n) -> Take<...>         // ← Kommt vom Iterator Trait
// ... ALLE Iterator-Methoden!
```

**Vorteil:** Du musst nicht in der Doku suchen - IDE zeigt dir alles!

### Hover für Details:

```rust
s.chars().next()  // ← Hoove über "next"
```

**IDE zeigt:**
```
fn next(&mut self) -> Option<Self::Item>

Trait: Iterator
Required method

Advances the iterator and returns the next value.
```

**Perfekt!** Du siehst sofort, dass es vom Iterator Trait kommt!

---

## 🎓 Pattern: "Wie finde ich Methoden auf Iteratoren?"

### Das Muster:

1. **Du hast einen Typ** (z.B. `&str`)
2. **Du rufst eine Methode auf** (z.B. `chars()`)
3. **Die gibt einen Iterator zurück** (z.B. `Chars`)
4. **Du willst wissen, was du damit machen kannst**
5. **→ Gehe zum Iterator Trait!**

### Konkrete Schritte:

```rust
// 1. Du hast:
let s: &str = "hello";

// 2. Du rufst auf:
s.chars()  // ← Was ist der Return-Type?

// 3. IDE zeigt: Chars<'_>
//    Oder Doku zeigt: -> Chars<'_>

// 4. Du suchst nach "Chars" in der Doku
//    Oder: IDE zeigt dir direkt alle Methoden

// 5. Du siehst: "impl Iterator for Chars"
//    → Gehe zum Iterator Trait!

// 6. Auf Iterator-Trait-Seite findest du ALLE Methoden:
//    - next()
//    - map()
//    - filter()
//    - collect()
//    - etc.
```

---

## 🎯 Quick Reference: Navigation in der Doku

### Wenn du `chars()` findest:

1. **Siehst du:** `pub fn chars(&self) -> Chars<'_>`
2. **Klicke auf:** `Chars` (ist verlinkt)
3. **Auf Chars-Seite:** Scrolle zu "Trait Implementations"
4. **Finde:** `impl Iterator for Chars`
5. **Klicke auf:** `Iterator` (ist verlinkt)
6. **Auf Iterator-Seite:** Siehst du ALLE Methoden!

### Wenn du einen Iterator-Typ hast:

1. **Suche nach dem Typ** in der Doku (z.B. "Chars", "Split")
2. **Scrolle zu:** "Trait Implementations"
3. **Finde:** `impl Iterator for ...`
4. **Klicke auf:** `Iterator`
5. **Fertig!** Du siehst alle Methoden

### Wenn du in der IDE bist:

1. **Tippe:** `.` nach dem Iterator
2. **IDE zeigt:** Alle Methoden automatisch!
3. **Hoove:** Für Details und Dokumentation
4. **Cmd+Click:** Springt zur Definition

---

## 🚀 Pro-Tipp: Bookmark setzen

**Bookmarke diese Seite:**
https://doc.rust-lang.org/std/iter/trait.Iterator.html

**Warum?**
- Fast alles in Rust ist ein Iterator
- Du wirst diese Seite sehr oft brauchen
- Alle Iterator-Methoden sind hier dokumentiert

---

# Teil 4: Quick Reference

## 🎯 Häufige Aufgaben

### String Operationen

```rust
// Finden
"hello".find('l')                    // Option<usize>
"hello".contains("ll")               // bool

// Aufteilen
"a,b,c".split(',')                  // Iterator<&str>
"hello\nworld".lines()               // Iterator<&str>

// Manipulation
"  hello  ".trim()                   // &str
"hello".to_uppercase()                // String
"hello".replace("l", "L")            // String
"hello".strip_prefix("he")           // Option<&str>

// Prüfung
"hello".starts_with("he")            // bool
"hello".ends_with("lo")              // bool
"".is_empty()                        // bool
```

### Iterator Operationen

```rust
vec![1, 2, 3].iter()
    .map(|x| x * 2)                  // Transformieren
    .filter(|x| x > &2)              // Filtern
    .take(2)                         // N Elemente nehmen
    .skip(1)                         // N Elemente überspringen
    .enumerate()                     // (index, value) Tupel
    .find(|(i, x)| *x > 5)           // Erstes passendes Element
    .collect::<Vec<_>>();            // In Collection sammeln
```

### Option/Result Operationen

```rust
Some(42)
    .map(|x| x * 2)                  // Transformieren wenn Some
    .filter(|x| *x > 50)             // Filtern wenn Some
    .unwrap_or(0)                    // Default wenn None
    .ok_or("error")?;                // Option → Result

Ok(42)
    .map(|x| x * 2)                  // Transformieren wenn Ok
    .map_err(|e| format!("{}", e))   // Fehler transformieren
    .unwrap_or(0);                   // Default wenn Err
```

---

## 💡 Pro-Tipps

### **Tipp 1: "Ich weiß nicht, was ich suchen soll"**

**Lösung: Beschreibe dein Problem in natürlicher Sprache:**
- "Ich will einen String aufteilen" → `split()`
- "Ich will über Elemente iterieren" → `iter()`, `into_iter()`
- "Ich will etwas zählen" → `count()`, `len()`
- "Ich will etwas finden" → `find()`, `position()`
- "Ich will etwas prüfen" → `contains()`, `starts_with()`

### **Tipp 2: Trait-Methoden finden**

Viele Methoden kommen von Traits, nicht direkt vom Typ:

```rust
let vec = vec![1, 2, 3];
vec.iter()  // ← Kommt von Iterator Trait
vec.len()   // ← Kommt von len() Methode auf Vec
```

**Wie findest du das?**
- IDE zeigt dir, woher die Methode kommt
- Doku zeigt "Trait Implementations"

### **Tipp 3: Experimentieren im Playground**

```rust
// Wenn du unsicher bist, teste es!
fn main() {
    let s = "test";
    // Experimentiere hier:
    dbg!(s.chars().next());
    dbg!(s.find('e'));
    dbg!(s.split('s').collect::<Vec<_>>());
}
```

### **Tipp 4: Compiler-Fehlermeldungen lesen**

```rust
let s = "hello";
s.next();  // ← Fehler: "no method named `next`"
           // ← Compiler schlägt vor: "did you mean `chars().next()`?"
           // ← Perfekt! Du lernst die richtige API!
```

---

## 🎓 Zusammenfassung: Dein Workflow

### **Wenn du eine Funktion brauchst:**

1. **IDE Auto-Complete** → Tippe `.` und schaue dir Methoden an
2. **Hover Documentation** → Hoovere über Methoden für Details
3. **Go to Definition** → Cmd+Click für vollständige Doku
4. **Standard Library Doku** → doc.rust-lang.org/std
5. **Rust Playground** → Experimentiere schnell
6. **Compiler-Fehler** → Lies Vorschläge des Compilers

### **Wenn du nicht weißt, was du suchst:**

1. **Beschreibe Problem** → "Ich will X machen mit Y"
2. **Suche in Doku** → Nach Typ oder Konzept
3. **Iterator-Methoden** → Fast alles ist Iterator
4. **Trait-Methoden** → Viele Methoden kommen von Traits
5. **Experimentiere** → Teste im Playground

---

## 📚 Weiterführende Ressourcen

1. **Rust Book**: https://doc.rust-lang.org/book/
2. **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
3. **Standard Library**: https://doc.rust-lang.org/std/
4. **Iterator Trait**: https://doc.rust-lang.org/std/iter/trait.Iterator.html
5. **Error Handling**: https://doc.rust-lang.org/book/ch09-00-error-handling.html
6. **Lifetimes**: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

---

## 🎯 Die wichtigsten Punkte

1. ✅ **Type Safety**: Enums statt Strings
2. ✅ **Error Handling**: Custom Error Types mit Kontext
3. ✅ **Separation of Concerns**: Lexer/Parser getrennt
4. ✅ **Iterator Pattern**: Lazy, memory-efficient
5. ✅ **Dokumentation**: `///` für öffentliche APIs
6. ✅ **Tests**: Unit + Integration Tests
7. ✅ **Module Structure**: Klare API-Grenzen
8. ✅ **Pattern Matching**: Statt if/else-Ketten
9. ✅ **IDE nutzen**: Auto-Complete ist dein bester Freund
10. ✅ **Traits verstehen**: Viele Methoden kommen von Traits

**Viel Erfolg beim weiteren Lernen! 🦀**
