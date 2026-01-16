# Quick Reference: Iterator Navigation

## Das Problem
`chars()` in der Doku gefunden, aber `next()` nicht gesehen?

## Die Lösung

### Option 1: In der Doku navigieren
1. https://doc.rust-lang.org/std/primitive.str.html
2. Finde: `chars() -> Chars<'_>`
3. **Klicke auf `Chars`** (ist verlinkt)
4. Scrolle zu: "Trait Implementations"
5. **Klicke auf `Iterator`** (ist verlinkt)
6. Finde: `next()` in "Required Methods"

### Option 2: Direkt zum Iterator Trait
https://doc.rust-lang.org/std/iter/trait.Iterator.html
→ Alle Iterator-Methoden auf einen Blick!

### Option 3: IDE nutzen (am einfachsten!)
```rust
"hello".chars().  // ← Tippe Punkt
                  // IDE zeigt ALLE Methoden!
```

## Wichtige Erkenntnis
Viele Methoden kommen von **Traits**, nicht direkt vom Typ!
- `next()` kommt vom `Iterator` Trait
- Nicht direkt von `str` oder `Chars`

## Bookmark diese Seite!
https://doc.rust-lang.org/std/iter/trait.Iterator.html
→ Du wirst sie sehr oft brauchen!
