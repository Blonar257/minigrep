# minigrep - Ein einfaches Grep-ähnliches CLI Tool in Rust

Ein schnelles und benutzerfreundliches Kommandozeilen-Tool zum Durchsuchen von Textdateien nach Suchstrings. Geschrieben in Rust mit dem `clap` Framework für CLI-Argument-Parsing.

## 🎯 Features

- **Case-sensitive und Case-insensitive Suche** (-i Flag)
- **Zeigt Zeilennummern und Spaltenpositionen** an
- **Kontext-Anzeige** (-C Flag) zeigt umliegende Zeilen
- **Treffer-Zähler** (-c Flag) zeigt nur die Anzahl der Matches
- **Mehrere Vorkommen pro Zeile** werden alle gefunden
- **Fehlerbehandlung** für fehlende Dateien

## 📦 Installation

### Voraussetzungen
- Rust 1.56 oder neuer
- Cargo

### Projekt bauen

```bash
cd minigrep
cargo build --release
```

Die ausführbare Datei befindet sich dann unter `target/release/minigrep`.

## 🚀 Verwendung

### Basis-Syntax

```bash
minigrep [OPTIONS] <SUCHSTRING> <DATEI>
```

### Beispiele

#### 1. Einfache Suche
```bash
cargo run -- "Rust" test_datei.txt
```

Findet alle Vorkommen von "Rust" und zeigt:
- Zeilennummer
- Spaltennummer (Position in der Zeile)
- Den Zeileninhalt

#### 2. Case-insensitive Suche
```bash
cargo run -- -i "rust" test_datei.txt
```

Findet "Rust", "rust", "RUST", etc.

#### 3. Nur Treffer zählen
```bash
cargo run -- -c "Rust" test_datei.txt
```

Output:
```
Anzahl der Treffer: 10
```

#### 4. Mit Kontext-Zeilen
```bash
cargo run -- -C 2 "Community" test_datei.txt
```

Zeigt 2 Zeilen vor und nach dem Treffer.

#### 5. Mehrere Optionen kombinieren
```bash
cargo run -- -i -n -C 1 "rust" test_datei.txt
```

- `-i`: Case-insensitive
- `-n`: Zeilennummern anzeigen
- `-C 1`: 1 Kontextzeile anzeigen

## 📋 Optionen

| Option | Langform | Beschreibung |
|--------|----------|-------------|
| `-i` | `--ignore-case` | Ignoriert Groß- und Kleinschreibung |
| `-c` | `--count` | Zeigt nur die Anzahl der Treffer an |
| `-n` | `--line-numbers` | Zeigt Zeilennummern an |
| `-C <N>` | `--context <N>` | Zeigt N Kontextzeilen um Treffer an |
| `-h` | `--help` | Zeigt Hilfe an |
| `-V` | `--version` | Zeigt Version an |

## 📊 Output-Format

Das Tool zeigt Suchergebnisse in folgendem Format:

```
📋 Suchergebnisse für 'Rust' (insgesamt: 10 Treffer)

1. Zeile    1, Spalte   1: Rust ist eine großartig moderne Programmiersprache.
            ^~~~

2. Zeile    2, Spalte   1: Rust bietet Speichersicherheit ohne Garbage Collection.
            ^~~~

✅ Suche abgeschlossen!
```

### Erklärung der Ausgabe:
- **Zeilennummer**: In welcher Zeile der Datei der Treffer gefunden wurde (1-basiert)
- **Spaltennummer**: An welcher Position in der Zeile der Treffer beginnt (1-basiert)
- **^~~~**: Visueller Indikator für die Position und Länge des Suchstrings

## 🏗️ Projektstruktur

```
minigrep/
├── Cargo.toml          # Projekt-Konfiguration und Dependencies
├── Cargo.lock          # Lock-Datei für Abhängigkeitsversionen
├── src/
│   ├── main.rs         # CLI-Entry Point mit clap Integration
│   └── lib.rs          # Core-Logik (Suchfunktionen)
├── test_datei.txt      # Beispiel-Datei zum Testen
└── README.md           # Diese Datei
```

## 🔧 Technische Details

### main.rs
- Definiert CLI-Argumente mit `clap` Derive-Macros
- Parst Kommandozeilen-Parameter
- Formatiert und zeigt Ergebnisse an
- Behandelt Fehler und zeigt aussagekräftige Meldungen

### lib.rs
- Definiert `Suchergebnis` Struct
- Definiert `Suchkonfiguration` Struct
- Implementiert `suche_case_sensitive()`
- Implementiert `suche_case_insensitive()`
- Enthält Unit-Tests

## ✅ Tests

Das Projekt enthält umfassende Unit-Tests:

```bash
cargo test
```

Tests prüfen:
- ✅ Case-sensitive Suche funktioniert
- ✅ Case-insensitive Suche funktioniert
- ✅ Mehrere Vorkommen in einer Zeile werden gefunden
- ✅ Korrekte Spaltennummern-Berechnung
- ✅ Keine Treffer (leeres Ergebnis)

## 🎨 Code-Highlight

### Suchfunktion (vereinfacht)
```rust
fn suche_case_sensitive(inhalt: &str, suchmuster: &str) -> Result<Vec<Suchergebnis>> {
    let mut ergebnisse = Vec::new();
    
    for (zeilenindex, zeile) in inhalt.lines().enumerate() {
        let zeilennummer = zeilenindex + 1;
        
        // Alle Vorkommen des Musters finden
        let mut start_position = 0;
        while let Some(position) = zeile[start_position..].find(suchmuster) {
            let absolute_position = start_position + position;
            let spaltennummer = absolute_position + 1;
            
            ergebnisse.push(Suchergebnis {
                zeilennummer,
                spaltennummer,
                zeileninhalt: zeile.to_string(),
            });
            
            start_position = absolute_position + suchmuster.len();
        }
    }
    
    Ok(ergebnisse)
}
```

## 🚀 Performance

Das Tool ist optimiert für:
- **Schnelle Suche**: Benutzt Rusts String-Matching
- **Speichereffizienz**: Iteriert zeilenweise durch die Datei
- **Fehlerbehandlung**: Gibt aussagekräftige Fehlermeldungen

## 🔜 Mögliche Erweiterungen

- [ ] Reguläre Ausdrücke (regex) unterstützen
- [ ] Mehrere Dateien durchsuchen
- [ ] Wildcard-Patterns
- [ ] Farb-Ausgabe für Terminal
- [ ] Export in verschiedene Formate (CSV, JSON)
- [ ] Performance-Optimierungen für große Dateien
- [ ] Rekursive Verzeichnis-Suche

## 📝 Lizenz

Dieses Projekt ist Open Source und frei verwendbar.

## 👨‍💻 Autor

Erstellt als Lernprojekt für Rust CLI-Entwicklung mit clap.

---

**Viel Spaß beim Verwenden von minigrep!** 🎉
