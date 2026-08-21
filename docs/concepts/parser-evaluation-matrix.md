> SUPERSEDED as controlling source by `docs/concepts/SOURCES_V2_SPEC.md`
> (§1 Directive Table) und dem lebenden Parser `src/main.rs`. Die
> 4-Token-Behauptung unten widerspricht dem heutigen Stand: der 3-Token-`field`
> wird laut abgelehnt (P01, Refused); gültig sind die 5-/6-/9-Token-Formen
> der Spec. Diese Datei ist Historie.

Die Syntax für Skalar-Extraktion in `sources.φ` ist ab sofort **exakt 4 Token**:
`<keyword> <key> <force> <unit>`

Wenn ein Wert keine physische Einheit hat (z.B. ein Index wie Sonnenflecken), ist das 4. Token explizit `scalar` (oder `-`). 

Der Parser in Rust hat nur exakt diesen einen Pfad:
```rust
"field" | "last" | "path" => {
    if parts.len() != 4 {
        // Ungültiger Block, wird verworfen
        continue; 
    }
    let key = parts[1];
    let force = parts[2];
    let unit = parts[3];
    // ... Validierung gegen die SI-Matrix
}
```

Die bestehenden 3500 Blöcke in `sources.φ` müssen für diesen neuen Parser einmalig migriert werden (z.B. per Regex: `field\s+(\S+)\s+(\S+)` -> `field $1 $2 scalar`). Danach ist der Code sauber, eindeutig und zukunftssicher.

Hier ist der **korrigierte, ultimative Handoff** für die nächste Session, der keine Kompromisse mehr kennt:

***

### 📋 HANDOFF FÜR DIE NÄCHSTE SESSION (Kopieren und als erste Nachricht einfügen)

**System Directive:** *A = A. The Kybernaut evaluates physics through the transfer entropy lens from a non-anthropomachinocentric position. The session is the atom — planning and implementation complete in the same context window. Phase-thinking is context-death. `cargo check` must produce zero errors AND zero warnings. `Cargo.toml` remains 100% empty (std-only). No backward compatibility. One syntax, one code path.*

**Projekt:** OmegaFlow (Rust, WebGPU, ICRS Block-Universum).
**Aufgabe:** Implementierung der "Strict 4-Token Per-Field Force & SI-Matrix" und des "Universal Anomaly Reporters".

Wir befinden uns im `Survey -> Deliberate -> Act` Zyklus. Lese zuerst `src/main.rs` (insb. `load_sources`, `extract_pending`, `ci_mode_main`, `fetch_one`), `AGENTS.md` und `TODO.md`. 

Hier ist der bindende Ratsbeschluss und der Bauplan für diese Session:

#### 1. Strict 4-Token Syntax (Keine Abwärtskompatibilität)
Die Syntax für skalare Extraktionen (`field`, `last`, `path`, `first`) in `sources.φ` wird radikal vereinheitlicht. Sie erfordert **immer exakt 4 Token**:
`<keyword> <key> <force> <unit>`
*   Beispiel: `last proton_speed advective km/s`
*   Beispiel: `field ssn em scalar` (dimensionslos)
*   Beispiel: `field 0.temp thermal K`

Der Parser (`load_sources`) akzeptiert keine 3-Token-Felder mehr. Blöcke, die nicht migriert wurden, werden beim Laden verworfen. Es gibt nur einen einzigen Code-Pfad im Parser.

#### 2. Die A = A Deduktions-Matrix
Es gibt keinen Fallback. Wenn eine Einheit nicht zur Kraft passt, wird das Feld an der Wurzel verworfen.
Implementiere in `main.rs` diese statische Matrix und die Konvertierung:

```rust
// 1. Matrix: Welche Einheiten sind für welche Kraft erlaubt?
fn allowed_units_for_force(force_id: u8) -> &'static [&'static str] {
    match force_id {
        0 | 1 => &["W/m2", "Wm-2", "nT", "uT", "T", "sfu", "scalar"],
        2 | 7 => &["m/s", "km/s", "km/h", "kph", "kn", "kts", "mph"],
        3 | 4 => &["m/s", "km/s"],
        5 => &["K", "C", "F"],
        6 | 8 => &["p/cm3", "cm-3", "1/cm3", "p/m3", "m-3", "hPa", "mb", "Pa", "scalar"],
        _ => &[],
    }
}

// 2. Konvertierung (wird nur aufgerufen, wenn die Matrix grünes Licht gab)
fn convert_to_si(val: f64, unit: &str) -> Option<f64> {
    match unit {
        "scalar" => Some(val),
        "km/s" => Some(val * 1000.0),
        // ... (alle Konvertierungen)
        _ => None,
    }
}
```

#### 3. Universal Anomaly Reporter (GitHub Issues)
Das System toleriert keine fehlerhaften Konfigurationen oder toten APIs. Es generiert automatisiert Reports, die über GitHub Copilot abgearbeitet werden können.
Implementiere `report_anomaly(category, url, details)`. Sie sammelt alle Anomalien während eines `--ci-mode` Laufs.
Kategorien:
*   `Physics Mismatch`: Kraft und Einheit passen nicht.
*   `API Unreachable`: `fetch_raw` liefert `None`.
*   `Empty Data`: API liefert 200 OK, aber `extract_pending` findet keine Samples.
*   `Malformed Data`: JSON-Parse Fehler.
*   `Invalid Syntax`: `sources.φ` Block hat falsche Token-Anzahl.

Am Ende des `ci_mode_main()` Durchlaufs (wenn `GH_TOKEN` env var vorhanden ist), wird **ein einziges kombiniertes GitHub Issue** erstellt (falls Anomalien vorhanden).
*   Titel: `[Automated CI Report] Omegaflow Anomalies ({datum})`
*   Body: Markdown-Tabelle mit Kategorien, URLs und Details.
*   Label: `anomaly-report`
Nutze `Command::new("gh").args(["issue", "create", ...])`.
Läuft der Archivar lokal (ohne Token), druckt er die Tabelle in die Konsole.

#### Aktionsplan für diese Session:
1.  **Act 1:** Schreibe ein kurzes Rust-Hilfsskript oder Regex-Anweisung, um `phi/sources.φ` auf die 4-Token-Syntax zu migrieren (finde 3-Token-Felder, füge `scalar` oder korrekte Einheit/Force hinzu).
2.  **Act 2:** Modifiziere `load_sources()` in `main.rs` *exklusiv* auf die 4-Token-Syntax. Kein Fallback.
3.  **Act 3:** Integriere `allowed_units_for_force` und `convert_to_si` in `extract_pending()`. Bei Mismatch -> Feld verwerfen, `report_anomaly` aufrufen.
4.  **Act 4:** Integriere `report_anomaly` in `fetch_raw`/`fetch_one` (bei HTTP Fehlern) und in `extract_pending` (bei leeren/fehlerhaften Daten).
5.  **Act 5:** Modifiziere `ci_mode_main()`: Sammle Anomalien, erstelle am Ende das GitHub Issue via `gh` CLI.
6.  **Act 6:** Passe `TODO.md` und `AGENTS.md` an.

Beginne mit dem Survey. Bestätige, dass du die Architektur verstanden hast, und lege los.
