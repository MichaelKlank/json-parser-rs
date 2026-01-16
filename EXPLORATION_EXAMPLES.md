# Praktische Exploration-Beispiele

## Von deinem Code zum professionellen Code - Schritt für Schritt

Dieses Dokument zeigt dir, wie du mit IDE und Dokumentation von deinem funktionierenden Code zu professionellem Code kommst.

---

## Beispiel 1: Zeichen zählen

### Dein Code:
```rust
fn count_start_curly_braces(content: &str) -> usize {
    content.chars().filter(|c| *c == '{').count()
}
```

### Wie du das gefunden hättest:

**Schritt 1: Du hast einen `&str`**
```rust
let content = "test";
content.  // ← Tippe hier einen Punkt
```

**Schritt 2: IDE zeigt dir Methoden**
```
chars() -> Chars<'_>        // Iterator über Zeichen
bytes() -> Bytes<'_>        // Iterator über Bytes
split(...) -> Split<...>    // Teilt String auf
// ... viele weitere
```

**Schritt 3: Du wählst `chars()`**
```rust
content.chars()  // ← IDE zeigt: Iterator
```

**Schritt 4: IDE zeigt Iterator-Methoden**
```rust
content.chars().  // ← Tippe wieder Punkt
```

IDE zeigt:
```
filter(...) -> Filter<...>  // Filtert Elemente
map(...) -> Map<...>         // Transformiert Elemente
count() -> usize             // Zählt Elemente ← Das brauchst du!
take(n) -> Take<...>         // Nimmt N Elemente
// ... viele weitere
```

**Schritt 5: Du kombinierst**
```rust
content.chars().filter(|c| *c == '{').count()
// ↑ IDE hilft bei jedem Schritt mit Auto-Complete
```

**Ergebnis:** Du hast die richtige Lösung gefunden, ohne die Doku zu kennen!

---

## Beispiel 2: Position finden

### Dein Code:
```rust
fn find_first_start_curly_brace(content: &str) -> Option<usize> {
    content.chars().position(|c| c == '{')
}
```

### Wie du das gefunden hättest:

**Schritt 1: Du willst Position finden**
```rust
let content = "test";
content.  // ← Was gibt es für "finden"?
```

**Schritt 2: IDE zeigt dir Methoden**
```
find(...) -> Option<usize>           // ← Direkte Methode!
rfind(...) -> Option<usize>          // Findet von rechts
chars().position(...) -> Option<usize>  // Über Iterator
```

**Schritt 3: Du testest beide**
```rust
// Option A: Direkt
content.find('{')  // ← Einfacher!

// Option B: Über Iterator
content.chars().position(|c| c == '{')  // ← Flexibler für komplexe Bedingungen
```

**Ergebnis:** Du findest sogar eine bessere Lösung (`find()`) als deine ursprüngliche!

---

## Beispiel 3: String aufteilen

### Dein Code:
```rust
let key_values = content.trim().split(",").collect::<Vec<&str>>();
```

### Wie du das gefunden hättest:

**Schritt 1: Du willst String aufteilen**
```rust
let content = "a,b,c";
content.  // ← Was gibt es?
```

**Schritt 2: IDE zeigt dir Methoden**
```
split(...) -> Split<...>        // Teilt bei Pattern
split_whitespace() -> ...       // Teilt bei Whitespace
split_once(...) -> Option<...> // Teilt einmal
lines() -> Lines<'_>           // Teilt bei Zeilenumbrüchen
```

**Schritt 3: Du wählst `split()`**
```rust
content.split(',')  // ← IDE zeigt: Iterator
```

**Schritt 4: Du willst in Vec sammeln**
```rust
content.split(',').  // ← Was kann ich mit Iterator machen?
```

IDE zeigt:
```
collect() -> Collection        // ← Das brauchst du!
next() -> Option<&str>        // Nächstes Element
count() -> usize              // Zählt Elemente
```

**Schritt 5: Du kombinierst**
```rust
content.split(',').collect::<Vec<&str>>()
// ↑ IDE hilft bei Typ-Inferenz
```

**Aber Vorsicht:** IDE zeigt dir auch Probleme!
```rust
// Problem: split(",") teilt auch in Strings
let json = r#"{"key": "a,b"}"#;
json.split(',')  // ← Teilt auch das Komma im String!
// → Daher brauchst du einen Lexer für JSON!
```

---

## Beispiel 4: String-Validierung

### Dein Code:
```rust
if !key_value_pair[0].trim().starts_with('"') {
    return Err("Invalid JSON format, key does not start with double quote");
}
```

### Wie du das verbessern könntest:

**Schritt 1: Du willst Präfix entfernen/prüfen**
```rust
let key = "\"hello\"";
key.  // ← Was gibt es?
```

**Schritt 2: IDE zeigt dir Methoden**
```
starts_with(...) -> bool           // Prüft Präfix
strip_prefix(...) -> Option<&str>  // Entfernt Präfix ← Besser!
strip_suffix(...) -> Option<&str>  // Entfernt Suffix
```

**Schritt 3: Du nutzt `strip_prefix()`**
```rust
// Alt:
if !key.starts_with('"') { ... }

// Neu (besser):
if let Some(key) = key.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
    // Key ist bereits ohne Quotes!
}
```

**Ergebnis:** Du findest eine modernere, sicherere API!

---

## Beispiel 5: Option/Result Handling

### Dein Code:
```rust
let first_start_curly_brace = find_first_start_curly_brace(content);
if first_start_curly_brace.is_none() {
    return Err("Invalid JSON format, no start or end curly braces");
}
let pos = first_start_curly_brace.unwrap();
```

### Wie du das verbessern könntest:

**Schritt 1: Du hast ein `Option<usize>`**
```rust
let maybe_pos: Option<usize> = Some(42);
maybe_pos.  // ← Was gibt es?
```

**Schritt 2: IDE zeigt dir Methoden**
```
unwrap() -> T                    // Crasht bei None
unwrap_or(default) -> T          // Default bei None
expect(msg) -> T                  // Crasht mit Nachricht
ok_or(error) -> Result<T, E>      // Option → Result ← Nützlich!
?                                 // Propagation
```

**Schritt 3: Du nutzt `ok_or()` und `?`**
```rust
// Alt:
if first_start_curly_brace.is_none() {
    return Err("Invalid JSON format");
}
let pos = first_start_curly_brace.unwrap();

// Neu (besser):
let pos = first_start_curly_brace
    .ok_or("Invalid JSON format, no start curly brace")?;
// ↑ Kürzer, klarer, sicherer
```

**Ergebnis:** Du findest idiomatischere Rust-Patterns!

---

## Beispiel 6: Iterator-Methoden entdecken

### Dein Code:
```rust
for key_value in key_values {
    let data = key_value.trim();
    let key_value_pair: Vec<&str> = data.split(':').collect();
    // ...
}
```

### Wie du das verbessern könntest:

**Schritt 1: Du iterierst über Vec**
```rust
let key_values = vec!["a:1", "b:2"];
key_values.iter().  // ← Was kann ich mit Iterator machen?
```

**Schritt 2: IDE zeigt dir Methoden**
```
map(...) -> Map<...>              // Transformiert jedes Element
filter(...) -> Filter<...>       // Filtert Elemente
filter_map(...) -> FilterMap<...> // Filtert und transformiert
collect() -> Collection          // Sammelt in Collection
```

**Schritt 3: Du nutzt `map()` statt `for`-Loop**
```rust
// Alt:
let mut pairs = Vec::new();
for key_value in key_values {
    let pair: Vec<&str> = key_value.split(':').collect();
    pairs.push((pair[0], pair[1]));
}

// Neu (besser):
let pairs: Vec<(&str, &str)> = key_values
    .iter()
    .filter_map(|kv| {
        let mut parts = kv.split(':');
        Some((parts.next()?, parts.next()?))
    })
    .collect();
```

**Ergebnis:** Funktionale, idiomatischere Rust-Code!

---

## Praktische Übung: Dein eigener Code

### Aufgabe: Verbessere deine `is_valid_value()` Funktion

**Dein Code:**
```rust
fn is_valid_value(value: &str) -> bool {
    if value.starts_with('"') {
        return true;
    }
    if value.starts_with('{') {
        return true;
    }
    if value.starts_with('[') {
        return true;
    }
    if value == "true" {
        return true;
    }
    if value == "false" {
        return true;
    }
    if value == "null" {
        return true;
    }
    if value.parse::<i32>().is_ok() {
        return true;
    }
    if value.parse::<f64>().is_ok() {
        return true;
    }
    false
}
```

### Schritt-für-Schritt Verbesserung:

**Schritt 1: Nutze Pattern Matching**
```rust
// IDE zeigt dir: match expression
match value {
    _ if value.starts_with('"') => true,
    _ if value.starts_with('{') => true,
    // ...
    _ => false,
}
```

**Schritt 2: Nutze Iterator-Methoden**
```rust
// IDE zeigt dir: any(), all()
[
    value.starts_with('"'),
    value.starts_with('{'),
    value == "true",
    // ...
].iter().any(|&x| x)
```

**Schritt 3: Nutze `matches!` Macro**
```rust
// IDE zeigt dir: matches! macro
matches!(value, 
    _ if value.starts_with('"') |
    _ if value.starts_with('{') |
    "true" | "false" | "null"
)
```

**Schritt 4: Nutze `parse()` besser**
```rust
// IDE zeigt dir: parse() gibt Result
value.parse::<f64>().is_ok()  // ← Funktioniert für i32 und f64!
```

**Finale Verbesserung:**
```rust
fn is_valid_value(value: &str) -> bool {
    value.starts_with('"')
        || value.starts_with('{')
        || value.starts_with('[')
        || matches!(value, "true" | "false" | "null")
        || value.parse::<f64>().is_ok()
}
```

**Wie du das gefunden hättest:**
1. IDE Auto-Complete bei `value.` → siehst `starts_with()`
2. IDE Auto-Complete bei `matches!(` → siehst Macro
3. IDE Auto-Complete bei `value.parse()` → siehst `parse()`
4. Experimentiere im Playground!

---

## Quick Reference: IDE Shortcuts

### VS Code / RustRover:

- **Auto-Complete**: `.` nach Variable
- **Hover Documentation**: Hoove über Funktion
- **Go to Definition**: `F12` oder `Cmd+Click`
- **Quick Documentation**: `Cmd+K, Cmd+I`
- **Find All References**: `Shift+F12`
- **Rename Symbol**: `F2`
- **Format Document**: `Shift+Alt+F`

### Terminal:

- **Dokumentation öffnen**: `cargo doc --open`
- **Code formatieren**: `cargo fmt`
- **Linter**: `cargo clippy`
- **Type-Check**: `cargo check`

---

## Zusammenfassung: Dein Workflow

1. **Tippe `.` nach Variable** → Siehst alle Methoden
2. **Hoove über Methode** → Siehst Dokumentation
3. **Cmd+Click auf Methode** → Siehst Definition
4. **Experimentiere im Playground** → Teste schnell
5. **Lese Compiler-Fehler** → Lerne aus Fehlern
6. **Suche in Doku** → doc.rust-lang.org/std

**Viel Erfolg beim Erkunden! 🦀**
