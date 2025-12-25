use clap::Parser;
use minigrep::{suche, Suchergebnis, Suchkonfiguration};
use owo_colors::OwoColorize;
use std::process;

/// Ein einfaches Grep-ähnliches Tool zum Durchsuchen von Dateien
#[derive(Parser, Debug)]
#[command(name = "minigrep")]
#[command(version = "0.1.0")]
#[command(about = "Sucht nach einem Suchstring in einer Datei und gibt die Positionen zurück", long_about = None)]
struct Argumente {
    /// Der zu suchende String
    #[arg(value_name = "SUCHSTRING")]
    suchstring: String,

    /// Die zu durchsuchende Datei
    #[arg(value_name = "DATEI")]
    dateipath: String,

    /// Ignoriert Groß- und Kleinschreibung
    #[arg(short = 'i', long = "ignore-case")]
    gross_kleinschreibung_ignorieren: bool,

    /// Zeigt nur die Anzahl der Treffer an
    #[arg(short = 'c', long = "count")]
    nur_anzahl: bool,

    /// Zeigt N Zeilen Kontext um die Treffer herum
    #[arg(short = 'C', long = "context", value_name = "N")]
    kontextzeilen: Option<usize>,
}

fn main() {
    // CLI-Argumente parsen
    let argumente = Argumente::parse();

    // Suchkonfiguration erstellen
    let konfiguration = Suchkonfiguration {
        suchmuster: argumente.suchstring,
        dateipfad: argumente.dateipath,
        gross_kleinschreibung_ignorieren: argumente.gross_kleinschreibung_ignorieren,
        kontextzeilen: argumente.kontextzeilen.unwrap_or(0),
        nur_anzahl: argumente.nur_anzahl,
        zeilennummern_anzeigen: false, // Wird nicht mehr verwendet
    };

    // Suche durchführen
    match suche(&konfiguration) {
        Ok(ergebnisse) => {
            // Nur Anzahl anzeigen?
            if konfiguration.nur_anzahl {
                println!("Anzahl der Treffer: {}", ergebnisse.len());
                return;
            }

            // Keine Treffer?
            if ergebnisse.is_empty() {
                println!(
                    "Kein Treffer für '{}' in Datei '{}'",
                    konfiguration.suchmuster, konfiguration.dateipfad
                );
                return;
            }

            // Ergebnisse anzeigen
            zeige_ergebnisse(&ergebnisse, &konfiguration);
        }
        Err(fehler) => {
            eprintln!(
                "Fehler beim Durchsuchen der Datei '{}': {}",
                konfiguration.dateipfad, fehler
            );
            process::exit(1);
        }
    }
}

/// Formatiert und zeigt die Suchergebnisse an
fn zeige_ergebnisse(ergebnisse: &[Suchergebnis], konfiguration: &Suchkonfiguration) {
    let gesamt_treffer = ergebnisse.len();

    println!(
        "\n{} Suchergebnisse für '{}' (insgesamt: {} Treffer)\n",
        "📋".bright_white(),
        konfiguration.suchmuster.bright_cyan(),
        gesamt_treffer.bright_yellow()
    );

    for (index, ergebnis) in ergebnisse.iter().enumerate() {
        // Zuerst: Die komplette Zeile mit grünem Suchstring
        let gefaerbte_zeile = faerbe_suchstring(
            &ergebnis.zeileninhalt,
            &konfiguration.suchmuster,
            konfiguration.gross_kleinschreibung_ignorieren,
        );

        println!("{}", gefaerbte_zeile);

        // Dann: Info mit Zeile und Spalte
        println!(
            "  {} Zeile {}, Spalte {}",
            "→".bright_cyan(),
            ergebnis.zeilennummer.bright_blue(),
            ergebnis.spaltennummer.bright_blue()
        );

        // Kontextzeilen anzeigen?
        if konfiguration.kontextzeilen > 0 {
            zeige_kontext(ergebnis, konfiguration);
        }

        // Leerzeile zwischen Treffern
        if index < gesamt_treffer - 1 {
            println!();
        }
    }

    println!("\n{} Suche abgeschlossen!", "✅".bright_green());
}

/// Färbt den Suchstring im Text grün ein
fn faerbe_suchstring(zeile: &str, suchmuster: &str, case_insensitive: bool) -> String {
    if case_insensitive {
        // Case-insensitive Färbung
        let suchmuster_klein = suchmuster.to_lowercase();
        let zeile_klein = zeile.to_lowercase();
        let mut result = String::new();
        let mut last_end = 0;

        let mut pos = 0;
        while let Some(index) = zeile_klein[pos..].find(&suchmuster_klein) {
            let absolute_index = pos + index;

            // Text vor dem Match hinzufügen
            result.push_str(&zeile[last_end..absolute_index]);

            // Match in Grün hinzufügen - als String für OwoColorize
            let gefaerbtes_match = zeile[absolute_index..absolute_index + suchmuster.len()]
                .to_string()
                .green()
                .to_string();
            result.push_str(&gefaerbtes_match);

            last_end = absolute_index + suchmuster.len();
            pos = absolute_index + suchmuster.len();
        }

        // Restlichen Text hinzufügen
        result.push_str(&zeile[last_end..]);
        result
    } else {
        // Case-sensitive Färbung
        let mut result = String::new();
        let mut last_end = 0;

        let mut pos = 0;
        while let Some(index) = zeile[pos..].find(suchmuster) {
            let absolute_index = pos + index;

            // Text vor dem Match hinzufügen
            result.push_str(&zeile[last_end..absolute_index]);

            // Match in Grün hinzufügen - als String für OwoColorize
            let gefaerbtes_match = zeile[absolute_index..absolute_index + suchmuster.len()]
                .to_string()
                .green()
                .to_string();
            result.push_str(&gefaerbtes_match);

            last_end = absolute_index + suchmuster.len();
            pos = absolute_index + suchmuster.len();
        }

        // Restlichen Text hinzufügen
        result.push_str(&zeile[last_end..]);
        result
    }
}

/// Zeigt Kontextzeilen um den Treffer
fn zeige_kontext(ergebnis: &Suchergebnis, konfiguration: &Suchkonfiguration) {
    if konfiguration.kontextzeilen == 0 {
        return;
    }

    // Datei erneut lesen (vereinfachte Implementierung)
    match std::fs::read_to_string(&konfiguration.dateipfad) {
        Ok(inhalt) => {
            let zeilen: Vec<&str> = inhalt.lines().collect();
            let aktuelle_zeile = ergebnis.zeilennummer - 1; // 0-basiert

            println!("  {}:", "Kontext".bright_cyan());

            // Zeilen davor
            let start = if aktuelle_zeile > konfiguration.kontextzeilen {
                aktuelle_zeile - konfiguration.kontextzeilen
            } else {
                0
            };

            for i in start..aktuelle_zeile {
                if i < zeilen.len() {
                    println!("    {} {:4}: {}", ">".bright_cyan(), i + 1, zeilen[i]);
                }
            }

            // Zeilen danach
            let ende = std::cmp::min(
                aktuelle_zeile + konfiguration.kontextzeilen + 1,
                zeilen.len(),
            );
            for i in (aktuelle_zeile + 1)..ende {
                if i < zeilen.len() {
                    println!("    {} {:4}: {}", ">".bright_cyan(), i + 1, zeilen[i]);
                }
            }
        }
        Err(_) => {
            // Fehler beim erneuten Lesen ignorieren
        }
    }
}
