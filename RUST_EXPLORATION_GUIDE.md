# Rust Standard Library Exploration Guide

## 🎯 Wie finde ich die richtigen Funktionen?

Dieser Guide zeigt dir praktische Tricks, um die Rust Standard Library zu erkunden und die richtigen Funktionen für deine Aufgaben zu finden.

---

## 1. 📚 Offizielle Dokumentation nutzen

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

### **Praktische Übung:**

1. Öffne: https://doc.rust-lang.org/std/primitive.str.html
2. Scrolle zu "Methods"
3. Schaue dir an:
   - `chars()` - Iterator über Unicode-Skalarwerte
   - `bytes()` - Iterator über Bytes
   - `split()` - Teilt String auf
   - `trim()` - Entfernt Whitespace
   - `starts_with()` - Prüft Präfix
   - `strip_prefix()` - Entfernt Präfix (neuere API)

---

## 2. 🔍 IDE-Features nutzen (VS Code / RustRover)

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

**Beispiel:**
```rust
let s = "hello";
s.chars()  // ← IDE zeigt: "pub fn chars(&self) -> Chars<'_>"
           // ← Hoovere für Details: "Returns an iterator over the chars..."
```

### **Trick 2: "Go to Definition" (F12 / Cmd+Click)**

```rust
let s = "hello";
let chars = s.chars();  // ← Cmd+Click auf "chars"
                        // → Springt zur Definition in std lib
                        // → Siehst alle Details, Dokumentation, Beispiele
```

### **Trick 3: "Find All References" (Shift+F12)**

```rust
// Finde heraus, wo eine Funktion verwendet wird
// → Siehst Beispiele aus der Standard Library selbst!
```

### **Trick 4: Quick Documentation (Cmd+K, Cmd+I)**

```rust
let s = "hello";
s.chars()  // ← Cursor auf chars, dann Cmd+K, Cmd+I
           // → Popup mit vollständiger Dokumentation
```

---

## 3. 🧪 Rust Playground für Experimente

### **rust-lang.org/play**

**Trick: Schnell testen ohne lokales Projekt**

```rust
fn main() {
    let s = "hello world";
    
    // Experimentiere mit verschiedenen Methoden:
    println!("{:?}", s.chars().collect::<Vec<_>>());
    println!("{:?}", s.split(' ').collect::<Vec<_>>());
    println!("{:?}", s.trim());
    
    // Schaue dir die Typen an:
    let chars: std::str::Chars = s.chars();
    // ↑ IDE zeigt dir den Typ, dann kannst du in der Doku nachschauen
}
```

**Vorteile:**
- Keine lokale Installation nötig
- Schnelle Experimente
- Kann Code teilen

---

## 4. 🎓 Iterator-Methoden entdecken

### **Das Iterator-Trait ist dein Schlüssel!**

Fast alles in Rust ist ein Iterator oder kann zu einem werden.

**Trick: Iterator-Methoden durchprobieren**

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Tippe: numbers.iter().
// IDE zeigt dir ALLE Iterator-Methoden:
numbers.iter()
    .map(|x| x * 2)      // Transformiere jedes Element
    .filter(|x| x > &5)  // Filtere Elemente
    .take(2)             // Nimm nur die ersten N
    .collect::<Vec<_>>(); // Sammle in Collection
```

**Wichtige Iterator-Methoden zum Erkunden:**

1. **Transformation:**
   - `map()` - Transformiere jedes Element
   - `filter()` - Filtere Elemente
   - `filter_map()` - Filtere und transformiere gleichzeitig
   - `flat_map()` - Flache verschachtelte Iteratoren

2. **Aggregation:**
   - `collect()` - Sammle in Collection
   - `fold()` - Reduziere zu einem Wert
   - `sum()` - Summiere Zahlen
   - `count()` - Zähle Elemente

3. **Navigation:**
   - `next()` - Nächstes Element
   - `take(n)` - Nimm N Elemente
   - `skip(n)` - Überspringe N Elemente
   - `enumerate()` - Füge Index hinzu

4. **Prüfung:**
   - `any()` - Prüft ob irgendein Element passt
   - `all()` - Prüft ob alle Elemente passen
   - `find()` - Finde erstes passendes Element

**Praktische Übung:**
```rust
let s = "hello world";
s.chars()
    .filter(|c| !c.is_whitespace())  // ← IDE zeigt: is_whitespace()
    .map(|c| c.to_uppercase())        // ← IDE zeigt: to_uppercase()
    .collect::<String>();             // ← IDE zeigt: collect()
```

---

## 5. 🔎 Pattern: "Ich will X machen mit Y"

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

## 6. 🧩 Häufige Patterns erkennen

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

## 7. 🎯 Konkrete Beispiele aus deinem Code

### **Dein alter Code vs. Standard Library**

#### **Beispiel 1: Zeichen zählen**

**Dein Code:**
```rust
fn count_start_curly_braces(content: &str) -> usize {
    content.chars().filter(|c| *c == '{').count()
}
```

**Exploration:**
```rust
// 1. Du hast: &str
// 2. Frage: "Wie zähle ich Zeichen?"
// 3. IDE: content.chars() → zeigt Iterator
// 4. IDE: .filter() → zeigt Filter-Methode
// 5. IDE: .count() → zeigt Count-Methode
// → Perfekt! Du hast es gefunden!
```

**Alternative (wenn du es nicht wusstest):**
```rust
// Suche in Doku: "str methods"
// → Finde: chars() → gibt Iterator
// → Suche: "Iterator methods"
// → Finde: count() → zählt Elemente
```

#### **Beispiel 2: Position finden**

**Dein Code:**
```rust
fn find_first_start_curly_brace(content: &str) -> Option<usize> {
    content.chars().position(|c| c == '{')
}
```

**Exploration:**
```rust
// 1. Du hast: &str
// 2. Frage: "Wie finde ich Position eines Zeichens?"
// 3. Option A: content.find('{')  // ← Direkte Methode auf str!
// 4. Option B: content.chars().position(|c| c == '{')
// 
// Wie findest du Option A?
// → IDE: content.find(  → zeigt: find() Methode
// → Oder Doku: str → Methods → find()
```

**Besser:**
```rust
// str hat direkt find() für einfache Fälle:
content.find('{')  // Schneller, einfacher
```

#### **Beispiel 3: String aufteilen**

**Dein Code:**
```rust
let key_values = content.trim().split(",").collect::<Vec<&str>>();
```

**Exploration:**
```rust
// 1. Du hast: &str
// 2. Frage: "Wie teile ich String auf?"
// 3. IDE: content.split(  → zeigt: split() Methode
// 4. IDE zeigt Signatur: split<Pattern>(&self, pat: Pattern) -> Split<'_, Pattern>
// 5. IDE zeigt Return-Type: Iterator!
// 6. Frage: "Wie sammle ich Iterator?"
// 7. IDE: .collect() → zeigt collect() Methode
```

**Aber Vorsicht:**
```rust
// Dein Code hat Problem: split(",") teilt bei jedem Komma
// Aber JSON hat Kommas auch in Strings: {"key": "a,b"}
// → Brauchst kontextbewusstes Parsing (daher Lexer!)
```

---

## 8. 🛠️ Praktische Übungen

### **Übung 1: String-Methoden entdecken**

```rust
fn main() {
    let s = "  hello world  ";
    
    // Aufgabe: Finde Methoden für:
    // 1. Whitespace entfernen
    // 2. In Großbuchstaben konvertieren
    // 3. Ersetzen von Zeichen
    // 4. Prüfen ob String leer ist
    
    // Lösung: Nutze IDE Auto-Complete!
    println!("{}", s.trim());
    println!("{}", s.to_uppercase());
    println!("{}", s.replace("world", "Rust"));
    println!("{}", s.is_empty());
}
```

### **Übung 2: Iterator-Methoden entdecken**

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Aufgabe: Finde Methoden für:
    // 1. Nur gerade Zahlen
    // 2. Jede Zahl verdoppeln
    // 3. Summe berechnen
    // 4. Erstes Element > 5 finden
    
    // Lösung: Nutze IDE!
    let evens: Vec<_> = numbers.iter().filter(|n| n % 2 == 0).collect();
    let doubled: Vec<_> = numbers.iter().map(|n| n * 2).collect();
    let sum: i32 = numbers.iter().sum();
    let first_large = numbers.iter().find(|n| **n > 5);
    
    println!("{:?}", evens);
    println!("{:?}", doubled);
    println!("{}", sum);
    println!("{:?}", first_large);
}
```

### **Übung 3: Option/Result Methoden**

```rust
fn main() {
    let maybe_number: Option<i32> = Some(42);
    let maybe_string: Option<String> = None;
    
    // Aufgabe: Finde Methoden für:
    // 1. Default-Wert wenn None
    // 2. Transformieren wenn Some
    // 3. Prüfen ob Some
    // 4. Unwrappen mit Fehlermeldung
    
    // Lösung: Nutze IDE!
    let num = maybe_number.unwrap_or(0);
    let transformed = maybe_number.map(|n| n * 2);
    let is_some = maybe_number.is_some();
    let string = maybe_string.expect("String sollte vorhanden sein");
}
```

---

## 9. 📖 Lern-Ressourcen

### **Offizielle Dokumentation**

1. **Standard Library:**
   - https://doc.rust-lang.org/std/
   - Suche nach Typen: `str`, `Vec`, `Option`, `Result`, `Iterator`

2. **Rust Book:**
   - https://doc.rust-lang.org/book/
   - Kapitel 8: Common Collections
   - Kapitel 13: Iterators
   - Kapitel 9: Error Handling

3. **Rust by Example:**
   - https://doc.rust-lang.org/rust-by-example/
   - Praktische Beispiele für alle Konzepte

### **Tools**

1. **rust-analyzer (VS Code Extension):**
   - Auto-Complete
   - Go to Definition
   - Hover Documentation
   - Find All References

2. **cargo doc --open:**
   ```bash
   cargo doc --open
   # Öffnet lokale Dokumentation mit allen Dependencies
   ```

3. **docs.rs:**
   - https://docs.rs
   - Dokumentation für alle Crates
   - Suche nach Funktionen, Typen, Traits

---

## 10. 🎯 Quick Reference: Häufige Aufgaben

### **String Operationen**

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

### **Iterator Operationen**

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

### **Option/Result Operationen**

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

## 11. 💡 Pro-Tipps

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

### **Tipp 5: Anderen Code lesen**

- GitHub: Suche nach Rust-Projekten
- crates.io: Schaue dir populäre Crates an
- Rust Standard Library Source Code selbst lesen!

---

## 12. 🎓 Zusammenfassung: Dein Workflow

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

## 🚀 Nächste Schritte

1. **Übe mit deinem Code:**
   - Öffne deinen alten Parser
   - Für jede Funktion: "Gibt es eine Standard-Library-Methode?"
   - Nutze IDE Auto-Complete

2. **Lese Standard Library Code:**
   - Viele Funktionen sind gut dokumentiert
   - Lerne Patterns von Profis

3. **Baue kleine Projekte:**
   - Übe verschiedene String-Operationen
   - Übe Iterator-Methoden
   - Übe Option/Result Handling

**Viel Erfolg beim Erkunden! 🦀**
