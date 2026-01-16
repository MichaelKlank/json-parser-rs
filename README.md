# JSON Parser in Rust

Ein professionell implementierter JSON-Parser in Rust, der als Lernprojekt für die [Coding Challenges](https://codingchallenges.fyi/challenges/challenge-json-parser) erstellt wurde.

## 🎯 Überblick

Dieses Projekt implementiert einen vollständigen JSON-Parser von Grund auf, der die JSON-Spezifikation (RFC 7159) unterstützt. Der Parser demonstriert professionelle Rust-Entwicklungspraktiken:

- ✅ **Type-Safe Parsing** mit Enums statt Strings
- ✅ **Separation of Concerns** (Lexer/Parser getrennt)
- ✅ **Custom Error Types** mit Position-Informationen
- ✅ **Iterator-basiertes Parsing** für Memory-Effizienz
- ✅ **Umfassende Fehlerbehandlung**
- ✅ **Vollständige Testabdeckung**

## 🚀 Features

- ✅ Parsing von JSON-Objekten
- ✅ Parsing von JSON-Arrays
- ✅ Unterstützung für alle JSON-Datentypen:
  - Strings
  - Numbers (Integer & Float)
  - Booleans (`true`, `false`)
  - `null`
  - Nested Objects
  - Nested Arrays
- ✅ Präzise Fehlermeldungen mit Zeilen- und Spaltenangabe
- ✅ CLI-Tool für Validierung von JSON-Dateien

## 📦 Installation

### Voraussetzungen

- Rust 1.70 oder höher
- Cargo (wird mit Rust installiert)

### Build

```bash
# Repository klonen
git clone git@github.com:MichaelKlank/json-parser-rs.git
cd json-parser-rs

# Projekt bauen
cargo build --release

# Tests ausführen
cargo test

# Dokumentation generieren
cargo doc --open
```

## 💻 Verwendung

### Als CLI-Tool

```bash
# Validierung einer JSON-Datei
./target/release/json-parser-rs path/to/file.json

# Bei gültigem JSON: Exit-Code 0
# Bei ungültigem JSON: Exit-Code 1 mit Fehlermeldung
```

### Als Library

```rust
use json_parser_rs::parse_json;

fn main() {
    let json = r#"{"key": "value", "number": 42}"#;
    
    match parse_json(json) {
        Ok(value) => {
            println!("Parsing erfolgreich!");
            // Arbeite mit dem JsonValue...
        }
        Err(e) => {
            eprintln!("Parse-Fehler: {}", e);
        }
    }
}
```

## 🏗️ Architektur

Das Projekt ist in mehrere Module aufgeteilt:

```
src/
├── lib.rs          # Öffentliche API
├── main.rs         # CLI-Einstiegspunkt
├── error.rs        # Custom Error Types
├── json.rs         # JSON-Wert-Datentypen (Enum)
├── lexer.rs        # Tokenisierung (Lexer)
└── parser.rs       # Syntaktische Analyse (Parser)
```

### Datenfluss

```
Input String → Lexer → Tokens → Parser → JsonValue Tree
```

1. **Lexer**: Konvertiert den Input-String in Tokens
2. **Parser**: Konvertiert Tokens in einen strukturierten JSON-Wert-Baum
3. **Error Handling**: Präzise Fehlermeldungen mit Position-Informationen

## 📚 Dokumentation

Das Projekt enthält umfassende Dokumentation:

- **[COMPLETE_GUIDE.md](COMPLETE_GUIDE.md)**: Vollständiger Guide zu Rust-Entwicklung und Standard Library Exploration
- **[ARCHITECTURE_DIAGRAM.md](ARCHITECTURE_DIAGRAM.md)**: Detaillierte Architektur-Diagramme
- **[PROFESSIONAL_RUST_GUIDE.md](PROFESSIONAL_RUST_GUIDE.md)**: Best Practices für professionelle Rust-Entwicklung

## 🧪 Tests

```bash
# Alle Tests ausführen
cargo test

# Tests mit Output
cargo test -- --nocapture

# Spezifischen Test ausführen
cargo test test_parse_empty_object
```

Das Projekt enthält Tests für alle Implementierungsstufen:
- Step 1: Einfache Objekte `{}`
- Step 2: Objekte mit String-Keys und -Values
- Step 3: Alle Datentypen (String, Number, Boolean, Null)
- Step 4: Verschachtelte Objekte und Arrays

## 📖 Lernressourcen

Dieses Projekt wurde als Lernprojekt erstellt und demonstriert:

- **Type Safety**: Verwendung von Enums für Type-Safety
- **Error Handling**: Custom Error Types mit Kontext
- **Iterator Pattern**: Lazy Evaluation und Memory-Effizienz
- **Separation of Concerns**: Klare Trennung von Lexer und Parser
- **Rust Best Practices**: Idiomatischer Rust-Code

### Empfohlene Ressourcen

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Coding Challenges - JSON Parser](https://codingchallenges.fyi/challenges/challenge-json-parser)

## 🛠️ Entwicklung

### Projektstruktur

```
json-parser-rs/
├── src/              # Quellcode
├── tests/            # Test-JSON-Dateien
│   ├── step1/
│   ├── step2/
│   ├── step3/
│   └── step4/
├── Cargo.toml        # Projekt-Konfiguration
├── README.md         # Diese Datei
├── LICENSE           # MIT-Lizenz
└── COMPLETE_GUIDE.md # Vollständiger Entwicklungsguide
```

### Code-Formatierung

```bash
# Code formatieren
cargo fmt

# Linter ausführen
cargo clippy

# Type-Check
cargo check
```

## 📝 Implementierungsstatus

- [x] Step 1: Einfache Objekte `{}`
- [x] Step 2: Objekte mit String-Keys und -Values
- [x] Step 3: Alle Datentypen (String, Number, Boolean, Null)
- [x] Step 4: Verschachtelte Objekte und Arrays
- [x] Umfassende Fehlerbehandlung
- [x] CLI-Tool
- [x] Vollständige Testabdeckung

## 🤝 Beitragen

Beiträge sind willkommen! Bitte:

1. Fork das Repository
2. Erstelle einen Feature-Branch (`git checkout -b feature/AmazingFeature`)
3. Committe deine Änderungen (`git commit -m 'Add some AmazingFeature'`)
4. Push zum Branch (`git push origin feature/AmazingFeature`)
5. Öffne einen Pull Request

## 📄 Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert - siehe [LICENSE](LICENSE) Datei für Details.

## 🙏 Danksagungen

- [Coding Challenges](https://codingchallenges.fyi/) für die inspirierende Challenge
- Rust Community für die hervorragende Dokumentation und Tools

## 📧 Kontakt

Bei Fragen oder Anregungen, öffne bitte ein Issue im Repository.

---

**Viel Erfolg beim Lernen von Rust! 🦀**
