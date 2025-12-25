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
cargo run -- -i -C 1 "rust" test_datei.txt
```

- `-i`: Case-insensitive
- `-C 1`: 1 Kontextzeile anzeigen

## 📋 Optionen

| Option | Langform | Beschreibung |
|--------|----------|-------------|
| `-i` | `--ignore-case` | Ignoriert Groß- und Kleinschreibung |
| `-c` | `--count` | Zeigt nur die Anzahl der Treffer an |
| `-C <N>` | `--context <N>` | Zeigt N Kontextzeilen um Treffer an |
| `-h` | `--help` | Zeigt Hilfe an |
| `-V` | `--version` | Zeigt Version an |

## 📊 Output-Format

Das Tool zeigt Suchergebnisse in folgendem Format:

```
📋 Suchergebnisse für 'Prog' (insgesamt: 3 Treffer)

Rust ist eine großartig moderne Programmiersprache.
  → Zeile 1, Spalte 34

Mit Rust können wir schnelle und sichere Programme schreiben.
  → Zeile 5, Spalte 43

Rust macht Programmieren spaßig und produktiv.
  → Zeile 9, Spalte 12

✅ Suche abgeschlossen!
```

### Erklärung der Ausgabe:
- **Zeilennummer**: In welcher Zeile der Datei der Treffer gefunden wurde (1-basiert)
- **Spaltennummer**: An welcher Position in der Zeile der Treffer beginnt (1-basiert)

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


**Viel Spaß beim Verwenden von minigrep!** 🎉
