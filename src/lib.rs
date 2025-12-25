use std::error::Error;
use std::fs;

/// Datenstruktur für ein Suchergebnis
#[derive(Debug, Clone)]
pub struct Suchergebnis {
    /// Zeilennummer (1-basiert)
    pub zeilennummer: usize,
    /// Spaltennummer (1-basiert)
    pub spaltennummer: usize,
    /// Inhalt der gesamten Zeile
    pub zeileninhalt: String,
}

/// Konfiguration für die Suche
pub struct Suchkonfiguration {
    /// Der Suchstring
    pub suchmuster: String,
    /// Pfad zur Datei
    pub dateipfad: String,
    /// Case-insensitive Suche aktiviert
    pub gross_kleinschreibung_ignorieren: bool,
    /// Kontextzeilen anzeigen
    pub kontextzeilen: usize,
    /// Nur Treffer zählen
    pub nur_anzahl: bool,
    /// Zeilennummern anzeigen
    pub zeilennummern_anzeigen: bool,
}

/// Führt eine Suche in einer Datei durch
/// Gibt einen Fehler zurück wenn die Datei nicht gelesen werden kann
pub fn suche(konfiguration: &Suchkonfiguration) -> Result<Vec<Suchergebnis>, Box<dyn Error>> {
    // Dateiinhalt lesen
    let inhalt = fs::read_to_string(&konfiguration.dateipfad)?;

    // Richtige Suchfunktion basierend auf Konfiguration aufrufen
    if konfiguration.gross_kleinschreibung_ignorieren {
        suche_case_insensitive(&inhalt, &konfiguration.suchmuster)
    } else {
        suche_case_sensitive(&inhalt, &konfiguration.suchmuster)
    }
}

/// Case-sensitive Suche
fn suche_case_sensitive(
    inhalt: &str,
    suchmuster: &str,
) -> Result<Vec<Suchergebnis>, Box<dyn Error>> {
    let mut ergebnisse = Vec::new();

    // Durch jede Zeile iterieren
    for (zeilenindex, zeile) in inhalt.lines().enumerate() {
        let zeilennummer = zeilenindex + 1; // 1-basiert

        // Alle Vorkommen des Suchmusters in dieser Zeile finden
        let mut start_position = 0;
        while let Some(position) = zeile[start_position..].find(suchmuster) {
            let absolute_position = start_position + position;
            let spaltennummer = absolute_position + 1; // 1-basiert

            ergebnisse.push(Suchergebnis {
                zeilennummer,
                spaltennummer,
                zeileninhalt: zeile.to_string(),
            });

            // Nächste Suche nach diesem Treffer starten
            start_position = absolute_position + suchmuster.len();
        }
    }

    Ok(ergebnisse)
}

/// Case-insensitive Suche
fn suche_case_insensitive(
    inhalt: &str,
    suchmuster: &str,
) -> Result<Vec<Suchergebnis>, Box<dyn Error>> {
    let suchmuster_klein = suchmuster.to_lowercase();
    let mut ergebnisse = Vec::new();

    // Durch jede Zeile iterieren
    for (zeilenindex, zeile) in inhalt.lines().enumerate() {
        let zeilennummer = zeilenindex + 1; // 1-basiert
        let zeile_klein = zeile.to_lowercase();

        // Alle Vorkommen des Suchmusters in dieser Zeile finden
        let mut start_position = 0;
        while let Some(position) = zeile_klein[start_position..].find(&suchmuster_klein) {
            let absolute_position = start_position + position;
            let spaltennummer = absolute_position + 1; // 1-basiert

            ergebnisse.push(Suchergebnis {
                zeilennummer,
                spaltennummer,
                zeileninhalt: zeile.to_string(),
            });

            // Nächste Suche nach diesem Treffer starten
            start_position = absolute_position + suchmuster_klein.len();
        }
    }

    Ok(ergebnisse)
}
