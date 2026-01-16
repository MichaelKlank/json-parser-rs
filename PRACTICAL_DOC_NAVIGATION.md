# Praktische Doku-Navigation: Von chars() zu next()

## Konkretes Beispiel: Wie du in der echten Doku navigierst

---

## 🎯 Das Problem nochmal

Du gehst zu: https://doc.rust-lang.org/std/primitive.str.html
Du findest: `pub fn chars(&self) -> Chars<'_>`
Aber: Wo ist `next()`? ❌

---

## ✅ Die Lösung: Schritt für Schritt

### Schritt 1: Öffne die str-Dokumentation

Gehe zu: **https://doc.rust-lang.org/std/primitive.str.html**

Scrolle zu den Methoden und finde:
```rust
pub fn chars(&self) -> Chars<'_>
```

**Wichtig:** `Chars<'_>` ist ein **Typ** - klicke darauf! (Es ist ein Link)

---

### Schritt 2: Klicke auf `Chars`

Du landest auf: **https://doc.rust-lang.org/std/str/struct.Chars.html**

**Was du siehst:**

```
Chars
A character iterator over a string slice.

This struct is created by the chars method on str.
```

**Wichtig:** Scrolle nach unten zu **"Trait Implementations"**

Du findest:
```
Trait Implementations
├── Clone
├── Debug
├── DoubleEndedIterator
├── FusedIterator
└── Iterator ← HIER IST ES!
```

---

### Schritt 3: Klicke auf `Iterator`

Du landest auf: **https://doc.rust-lang.org/std/iter/trait.Iterator.html**

**Das ist die Goldmine!** Hier findest du:

#### Required Methods (müssen implementiert sein):
```rust
fn next(&mut self) -> Option<Self::Item>
```
**← Das ist es! `next()` ist hier!**

#### Provided Methods (haben Default-Implementierung):
```rust
fn map<B, F>(self, f: F) -> Map<Self, F>
fn filter<P>(self, predicate: P) -> Filter<Self, P>
fn collect<B>(self) -> B
fn take(self, n: usize) -> Take<Self>
fn skip(self, n: usize) -> Skip<Self>
fn enumerate(self) -> Enumerate<Self>
// ... und viele mehr!
```

**Alle diese Methoden funktionieren auf `chars()`!**

---

## 🔍 Visueller Ablauf

```
1. doc.rust-lang.org/std/primitive.str.html
   │
   │ Finde: chars() -> Chars<'_>
   │
   ▼
2. doc.rust-lang.org/std/str/struct.Chars.html
   │
   │ Scrolle zu: "Trait Implementations"
   │ Finde: impl Iterator for Chars
   │
   ▼
3. doc.rust-lang.org/std/iter/trait.Iterator.html
   │
   │ Finde: next() in "Required Methods"
   │ Finde: map(), filter(), collect() in "Provided Methods"
   │
   ▼
   ✅ Du hast alle Methoden gefunden!
```

---

## 💡 Alternative: Direkt zum Iterator Trait

Wenn du weißt, dass du einen Iterator hast:

**Direkt gehen zu:** https://doc.rust-lang.org/std/iter/trait.Iterator.html

**Dort findest du:**
- Alle Required Methods (inkl. `next()`)
- Alle Provided Methods (inkl. `map()`, `filter()`, etc.)
- Beispiele für jede Methode
- Dokumentation

**Vorteil:** Du siehst sofort ALLE Möglichkeiten!

---

## 🎓 Pattern: Wie du es dir merken kannst

### Die Regel:

**Wenn eine Funktion einen Iterator zurückgibt:**
- `chars()` → `Chars` (Iterator)
- `split()` → `Split` (Iterator)
- `iter()` → `Iter` (Iterator)
- `lines()` → `Lines` (Iterator)

**→ Alle haben die gleichen Methoden vom Iterator Trait!**

### Praktisch:

1. **Du findest eine Funktion, die einen Iterator zurückgibt**
2. **→ Gehe direkt zum Iterator Trait**
3. **→ Siehst ALLE verfügbaren Methoden**

**Du musst nicht für jeden Iterator-Typ einzeln suchen!**

---

## 🔧 Praktisches Beispiel: Dein Code

### Dein Code:
```rust
self.current_char = self.input[self.position..].chars().next();
```

### Navigation:

**Schritt 1:** Du siehst `chars()` in deinem Code
**Schritt 2:** Du weißt: `chars()` gibt `Chars` zurück
**Schritt 3:** Du weißt: `Chars` ist ein Iterator
**Schritt 4:** Du gehst direkt zu: https://doc.rust-lang.org/std/iter/trait.Iterator.html
**Schritt 5:** Du findest: `next()` in "Required Methods"
**Fertig!**

---

## 🚀 Pro-Tipp: Bookmark setzen

**Bookmarke diese Seite:**
https://doc.rust-lang.org/std/iter/trait.Iterator.html

**Warum?**
- Fast alles in Rust ist ein Iterator
- Du wirst diese Seite sehr oft brauchen
- Alle Iterator-Methoden sind hier dokumentiert

---

## 📚 Weitere wichtige Traits

### Genauso wie Iterator:

1. **FromStr Trait**
   - Für: `"123".parse::<i32>()`
   - Doku: https://doc.rust-lang.org/std/str/trait.FromStr.html

2. **Display Trait**
   - Für: `format!("{}", value)`
   - Doku: https://doc.rust-lang.org/std/fmt/trait.Display.html

3. **Debug Trait**
   - Für: `println!("{:?}", value)`
   - Doku: https://doc.rust-lang.org/std/fmt/trait.Debug.html

4. **IntoIterator Trait**
   - Für: `for item in collection`
   - Doku: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html

**Pattern:** Wenn du eine Trait-Methode suchst, gehe direkt zum Trait!

---

## 🎯 Zusammenfassung

### Wenn du `chars()` findest:

1. ✅ Siehst: `chars() -> Chars<'_>`
2. ✅ Klicke auf: `Chars` (Link in der Doku)
3. ✅ Scrolle zu: "Trait Implementations"
4. ✅ Klicke auf: `Iterator`
5. ✅ Finde: `next()` und alle anderen Methoden!

### Oder schneller:

1. ✅ Du weißt: `chars()` gibt Iterator zurück
2. ✅ Gehe direkt zu: Iterator Trait Doku
3. ✅ Finde: Alle Methoden auf einen Blick!

### Oder am einfachsten:

1. ✅ Nutze IDE Auto-Complete
2. ✅ Tippe `.` nach `chars()`
3. ✅ Siehst alle Methoden sofort!

**Viel Erfolg! 🦀**
