# minigrep - Quick Start Guide 🚀

Schnelle Anleitung zum Starten mit minigrep!

## Installation & Kompilierung

```bash
# Projekt kompilieren
cargo build

# Oder: Optimierter Release-Build
cargo build --release
```

## Häufigste Befehle

### 1. Einfache Suche
```bash
cargo run -- "Suchstring" datei.txt
```
Findet alle Vorkommen von "Suchstring" in der Datei.

### 2. Case-insensitive Suche
```bash
cargo run -- -i "suchstring" datei.txt
```
Findet "Suchstring", "SUCHSTRING", "suchstring", etc.

### 3. Nur Anzahl der Treffer
```bash
cargo run -- -c "Suchstring" datei.txt
```
Gibt nur die Anzahl zurück: `Anzahl der Treffer: 5`

### 4. Mit Kontextzeilen
```bash
cargo run -- -C 2 "Suchstring" datei.txt
```
Zeigt 2 Zeilen vor und nach dem Treffer.

### 5. Mit Zeilennummern formatiert
```bash
cargo run -- -n "Suchstring" datei.txt
```
Bessere Formatierung mit Zeilennummern.

### 6. Alle Optionen kombinieren
```bash
cargo run -- -i -n -C 1 "suchstring" datei.txt
```
Case-insensitive + Zeilennummern + 1 Kontextzeile

## Vollständige Optionen

| Option | Bedeutung |
|--------|-----------|
| `-i, --ignore-case` | Groß-/Kleinschreibung ignorieren |
| `-c, --count` | Nur Treffer-Anzahl anzeigen |
| `-n, --line-numbers` | Mit Zeilennummern formatieren |
| `-C <N>, --context <N>` | N Kontextzeilen anzeigen |
| `-h, --help` | Hilfe anzeigen |
| `-V, --version` | Version anzeigen |

## Tests ausführen

```bash
# Alle Tests
cargo test

# Mit Ausgabe
cargo test -- --nocapture

# Spezifischen Test
cargo test case_sensitive_suche_funktioniert
```

## Release-Binary verwenden

Für bessere Performance einen Release-Build nutzen:

```bash
# 1. Release-Build erstellen
cargo build --release

# 2. Binary direkt nutzen (viel schneller!)
./target/release/minigrep "Suchstring" datei.txt
```

Der Release-Binary ist ~50% schneller als der Debug-Build.

## Praktische Beispiele

### Beispiel 1: Fehler in Log-Dateien suchen
```bash
cargo run -- "ERROR" application.log
```

### Beispiel 2: Funktionsdefinitionen finden
```bash
cargo run -- -n "fn " src/main.rs
```

### Beispiel 3: Alle Vorkommen mit Kontext
```bash
cargo run -- -i -C 3 "wichtiger_string" datei.txt
```

### Beispiel 4: Nur zählen wie viele mal
```bash
cargo run -- -c "TODO" src/lib.rs
```

### Beispiel 5: In Code-Kommentare suchen
```bash
cargo run -- "//" src/main.rs
```

## Typische Fehler beheben

### "Datei nicht gefunden"
```
Fehler beim Durchsuchen der Datei 'datei.txt': No such file or directory
```
→ Stelle sicher, dass der Dateipfad korrekt ist. Nutze relative oder absolute Pfade.

### Keine Treffer gefunden
```
Kein Treffer für 'Suchstring' in Datei 'datei.txt'
```
→ Das ist normal! Der String existiert nicht in der Datei. 
→ Versuche `-i` für Case-insensitive Suche.

### Zu viele Treffer
```bash
# Nutze -c um nur zu zählen
cargo run -- -c "der" datei.txt

# Oder nutze spezifischere Strings
cargo run -- "der Satz" datei.txt
```

## Tipps & Tricks

### Tip 1: Mit der Test-Datei üben
```bash
# Test-Datei ist enthalten
cargo run -- "Rust" test_datei.txt
```

### Tip 2: Kombiniere mit grep (Shell-Pipeline)
```bash
# Finde "Rust" und filtere dann nach "Community"
./target/release/minigrep "Rust" test_datei.txt | grep "Community"
```

### Tip 3: Output in Datei speichern
```bash
cargo run -- "Suchstring" datei.txt > ergebnis.txt
```

### Tip 4: Mehrere Dateien durchsuchen (Workaround)
```bash
# Kombiniere alle Dateien
cat datei1.txt datei2.txt | cargo run -- "Suchstring"
```

### Tip 5: Größere Dateien sind kein Problem
Das Tool kann auch große Dateien effizient durchsuchen, da es zeilenweise arbeitet.

## Performance-Vergleich

Debug vs Release:
```bash
# Langsamer (Development-Build)
time cargo run -- "test" große_datei.txt

# Schneller (Release-Build)
time ./target/release/minigrep "test" große_datei.txt
```

Release ist typisch 50% schneller! ⚡

## Weiterführende Ressourcen

- `README.md` - Umfassende Dokumentation
- `IMPLEMENTIERUNG.md` - Technische Details
- `cargo test --help` - Mehr Test-Optionen
- `cargo run -- --help` - CLI-Hilfe im Tool selbst

## Shortcuts

```bash
# Kompilieren + Tests + Ausführen
cargo build && cargo test && cargo run -- "test" test_datei.txt

# Nur Release bauen und testen
cargo build --release && cargo test --release
```

---

**Viel Spaß! Bei Fragen: Siehe README.md oder IMPLEMENTIERUNG.md** 🎉