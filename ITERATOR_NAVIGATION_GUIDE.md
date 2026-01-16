# Iterator Navigation Guide: Von chars() zu next()

## Das Problem

Du findest `chars()` in der Doku, aber siehst dort nicht `next()`. Wie kommst du zu den richtigen Informationen?

---

## 🎯 Die Lösung: Iterator Traits verstehen

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

## 🔍 Praktisches Beispiel: Dein Lexer-Code

### Dein Code:
```rust
fn advance(&mut self) {
    if self.position < self.input.len() {
        self.current_char = self.input[self.position..].chars().next();
        // ↑ Wie findest du .next()?
    }
}
```

### Navigation in der Doku:

**Schritt 1: Du siehst `chars()`**
```rust
self.input[self.position..]  // &str
    .chars()                   // ← Was gibt das zurück?
```

**Schritt 2: In der Doku nachschauen**
- Gehe zu: https://doc.rust-lang.org/std/primitive.str.html
- Finde: `pub fn chars(&self) -> Chars<'_>`
- **Klicke auf `Chars`** → springt zu: https://doc.rust-lang.org/std/str/struct.Chars.html

**Schritt 3: Auf der Chars-Seite**
- Du siehst: `impl Iterator for Chars`
- **Klicke auf `Iterator`** → springt zu: https://doc.rust-lang.org/std/iter/trait.Iterator.html

**Schritt 4: Auf der Iterator-Seite**
- Scrolle zu "Required Methods"
- Finde: `fn next(&mut self) -> Option<Self::Item>`
- **Das ist es!** `next()` ist eine Methode des Iterator Traits

**Schritt 5: Alle verfügbaren Methoden sehen**
- Scrolle weiter auf der Iterator-Seite
- Siehst du: `map()`, `filter()`, `collect()`, `take()`, `skip()`, etc.
- **Alle diese Methoden funktionieren auf `chars()`!**

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

## 💡 Trick 2: Typ-Annotation nutzen

### Wenn du unsicher bist, welcher Typ zurückgegeben wird:

```rust
let s = "hello";
let chars: std::str::Chars = s.chars();
// ↑ IDE zeigt dir den Typ
// ↑ Dann kannst du in der Doku nach "Chars" suchen
```

### Oder: dbg!() nutzen

```rust
let s = "hello";
let chars = s.chars();
dbg!(std::any::type_name_of_val(&chars));
// Output: "core::str::iter::Chars"
// → Jetzt weißt du: Es ist ein Chars Iterator
```

---

## 💡 Trick 3: Trait-Implementierungen in der Doku finden

### Auf der Chars-Seite in der Doku:

1. Gehe zu: https://doc.rust-lang.org/std/str/struct.Chars.html
2. Scrolle zu "Trait Implementations"
3. Du siehst:

```
impl Iterator for Chars
```

4. **Klicke darauf** → springt zum Iterator Trait
5. Dort siehst du alle Methoden!

### Allgemein: Jeder Iterator-Typ hat diese Sektion

- `Vec<T>` → `impl IntoIterator for Vec<T>`
- `&str` → `impl Iterator for Chars`
- `String` → `impl Iterator for Chars` (über `chars()`)

**Alle zeigen dir, dass sie Iterator implementieren!**

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

## 📖 Praktische Übung: Alle Iterator-Methoden entdecken

### Übung 1: Von str zu Iterator

```rust
let s = "hello world";

// Schritt 1: chars() aufrufen
let chars = s.chars();
// ↑ IDE zeigt: Chars<'_>

// Schritt 2: IDE Auto-Complete nutzen
chars.  // ← Tippe Punkt
        // IDE zeigt dir ALLE Iterator-Methoden!

// Schritt 3: Experimentiere
chars.next()                    // Option<char>
chars.take(5).collect::<String>() // "hello"
chars.filter(|c| c != ' ').collect::<String>() // "helloworld"
```

### Übung 2: Verschiedene Iteratoren

```rust
// String → Iterator
"hello".chars()           // Chars Iterator
"hello".bytes()          // Bytes Iterator
"a,b,c".split(',')       // Split Iterator

// Vec → Iterator
vec![1, 2, 3].iter()     // Iter Iterator
vec![1, 2, 3].into_iter() // IntoIter Iterator

// Alle haben .next(), .map(), .filter(), etc.!
```

### Übung 3: Iterator-Methoden kombinieren

```rust
let s = "hello world";

s.chars()
    .filter(|c| !c.is_whitespace())  // ← IDE zeigt: is_whitespace()
    .map(|c| c.to_uppercase())       // ← IDE zeigt: to_uppercase()
    .take(5)                         // ← IDE zeigt: take()
    .collect::<String>();            // ← IDE zeigt: collect()
```

**Wie findest du jede Methode?**
- `filter()` → Iterator Trait → Provided Methods
- `map()` → Iterator Trait → Provided Methods
- `take()` → Iterator Trait → Provided Methods
- `collect()` → Iterator Trait → Provided Methods

**Alle kommen vom gleichen Trait!**

---

## 🔍 Häufige Iterator-Typen und wo du sie findest

### 1. Chars (von str.chars())

**Doku:** https://doc.rust-lang.org/std/str/struct.Chars.html
- `impl Iterator for Chars`
- Klicke auf Iterator → siehst alle Methoden

### 2. Split (von str.split())

**Doku:** https://doc.rust-lang.org/std/str/struct.Split.html
- `impl Iterator for Split<'_, Pattern>`
- Klicke auf Iterator → siehst alle Methoden

### 3. Iter (von Vec.iter())

**Doku:** https://doc.rust-lang.org/std/slice/struct.Iter.html
- `impl Iterator for Iter<'_, T>`
- Klicke auf Iterator → siehst alle Methoden

### 4. Map, Filter, etc. (Adapter)

**Doku:** https://doc.rust-lang.org/std/iter/struct.Map.html
- Alle sind Iteratoren selbst!
- Haben auch `impl Iterator`
- Klicke auf Iterator → siehst alle Methoden

**Pattern:** Fast alles in Rust, was iteriert, implementiert den Iterator Trait!

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

## 💻 Praktisches Beispiel: Dein Lexer

### Dein Code:
```rust
self.current_char = self.input[self.position..].chars().next();
```

### Wie du das gefunden hättest:

**Schritt 1: Du hast `&str`**
```rust
self.input[self.position..]  // &str
```

**Schritt 2: Du willst Zeichen bekommen**
```rust
self.input[self.position..].  // ← Tippe Punkt
```

**IDE zeigt:**
```
chars() -> Chars<'_>    // ← Das brauchst du!
bytes() -> Bytes<'_>
split(...) -> Split<...>
```

**Schritt 3: Du wählst `chars()`**
```rust
self.input[self.position..].chars()  // Chars<'_>
```

**Schritt 4: Du willst das erste Zeichen**
```rust
self.input[self.position..].chars().  // ← Tippe Punkt
```

**IDE zeigt:**
```
next() -> Option<char>     // ← Das brauchst du!
nth(n) -> Option<char>     // Alternative
take(1) -> Take<...>       // Alternative
```

**Schritt 5: Du wählst `next()`**
```rust
self.input[self.position..].chars().next()  // Option<char>
```

**Fertig!** IDE hat dir alles gezeigt, ohne dass du in der Doku suchen musstest!

---

## 🎓 Zusammenfassung: Die wichtigsten Erkenntnisse

### 1. **Viele Methoden kommen von Traits**
- `next()` kommt vom `Iterator` Trait
- Nicht direkt von `str` oder `Chars`

### 2. **IDE zeigt dir alles automatisch**
- Tippe `.` nach einem Iterator
- Siehst alle verfügbaren Methoden
- Keine Doku-Suche nötig!

### 3. **In der Doku navigieren**
- Klicke auf Typ-Namen (sind verlinkt)
- Finde "Trait Implementations"
- Klicke auf den Trait (z.B. `Iterator`)
- Siehst alle Methoden

### 4. **Pattern erkennen**
- Wenn etwas einen Iterator zurückgibt
- → Alle Iterator-Methoden funktionieren darauf
- → Gehe zum Iterator Trait für Details

### 5. **Praktisch: Nutze die IDE!**
- Auto-Complete ist dein bester Freund
- Zeigt dir alle Methoden
- Mit Hover siehst du Dokumentation
- Mit Cmd+Click springst du zur Definition

---

## 🚀 Nächste Schritte

1. **Übe mit deinem Code:**
   - Öffne deinen Lexer
   - Tippe `.` nach jedem Iterator
   - Schaue dir an, was die IDE zeigt

2. **Experimentiere:**
   ```rust
   let s = "hello";
   s.chars().  // ← Probiere alle Methoden aus!
   ```

3. **Lese Iterator Trait Doku:**
   - https://doc.rust-lang.org/std/iter/trait.Iterator.html
   - Lerne die wichtigsten Methoden kennen

**Viel Erfolg! 🦀**
