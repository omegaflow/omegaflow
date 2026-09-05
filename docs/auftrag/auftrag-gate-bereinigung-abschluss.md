<!--
  title: Auftrag — Gate-Bereinigung abschließen: die Fabrication-Verstöße beheben
  class: auftrag
  date: 2026-09-05
  status: pending
  sha256: 6b6b1ba528e059ee6a5b3cb615c474425952529d12bc78ba0f309ce9d147c361
  see-also: docs/auftrag/auftrag-richtungs-transient-atom.md
-->

# Auftrag: den zero-cost Commit-Check zum Grün bringen — die Fabrication-Stellen bereinigen

## Zweck

Der deterministische Commit-Check (commit_gate, Pre-Commit-Hook) ist verdrahtet.
Er blockiert jetzt jeden Commit mit Fabrication-Stellen. Der Commit der
Umbenennung schlägt fehl, weil der Hook vorbestehenden Ballast findet. Dieser
Auftrag: jede gemeldete Stelle **ehrlich beheben**, bis `git commit` durch den
Hook grün ist. Keine Regel lockern, kein Muster umgehen, kein Ausblenden.

## Stand (bereits erledigt, uncommittet im Working-Tree)

- `llm_gate` → `commit_gate` umbenannt (Datei, Feature, Referenzen)
- Teurer Proxy entfernt: `tools/gate/src/bin/llm_interceptor.rs`,
  `scripts/llm_gate.sh` (ins `../archive/omegaflow-legacy/` verschoben)
- `tools/gate/src/bin/commit_check.rs`: dünner Einstieg über die bestehende
  `check_tool_call`-Logik (zero Tokens, kein LLM)
- Pre-Commit-Hook `.git/hooks/pre-commit`
- comment-Regel erweitert: blockiert jetzt alle `//`-Kommentare (nicht nur `///`)
- 291 Kommentarzeilen entfernt (fetch/membrane/thermochem/te)
- 49 Gate-Tests grün, `cargo check` 0/0

## Offene Arbeit — jede vom Hook gemeldete Stelle ehrlich beurteilen und beheben

Der Hook meldet beim Commit (ganze gestagte Datei):

| Datei | Regel | Stelle |
|---|---|---|
| `src/archivar/fetch.rs` | zero-fabrication | `unwrap_or(0)` (510) |
| `src/archivar/thermochem.rs` | fabrication | `unwrap_or_else(|| panic!("... absent"))` (1725, 1906, 2076, 2300, 2329) |
| `src/mathematikerin/te.rs` | fabrication | `.max(1)` (1744) |
| `tools/register/src/bin/claim_verify.rs` | german-in-code | (deutsche Wörter im Code) |
| `tools/register/src/bin/home_scan.rs` | fabrication | `unwrap_or_default()` (39) |
| `tools/register/src/bin/register_verify.rs` | fabrication | `unwrap_or_default()` (168, 267) |
| `src/gate/commit_gate.rs` | single-path | (nennt branch/strand-Modell) |

## Kernregel (0 honored, A=A)

Jede Stelle **einzeln lesen und beurteilen** — nicht blind ersetzen. Zwei Klassen:

1. **Echte Fabrication** (erfundener Wert/Floor/Default): die Stelle so
   umschreiben, dass absent ehrlich bleibt. `.max(1)` → Guard `if meas > 0`
   (kein leerer Assert als Durchfall); `unwrap_or_default()` auf
   `read_to_string` → Option/skip statt `""`. Kein erfundener Wert.
2. **Legitime 0-honored-Form, die das pauschale Muster trifft**: z. B.
   `unwrap_or_else(|| panic!("{name} absent"))` (benennt absent, fabriciert
   nichts) und `unwrap_or_else(|| source_name_from_url(u))` (berechneter Name,
   kein erfundener Wert). Diese so umschreiben, dass die Absicht explizit ohne
   das verbotene Muster ausgedrückt wird (match/if-let statt unwrap_or_else),
   **ohne** das Verhalten zu ändern. Die Absicht bleibt: absent → benannt,
   nie eine erfundene Zahl.

## Lieferung

Der Commit der Umbenennung geht durch den Pre-Commit-Hook grün (exit 0). Alle
Fabrication-Stellen sind ehrlich aufgelöst (keine erfundenen Werte mehr, keine
gemeldeten Stellen unkommentiert). `cargo check` 0/0, Gate-Tests grün. Der
Commit enthält die Umbenennung + Bereinigung, nicht die fremden Dateien
(`tools/measure/src/bin/lsst_anomaly_probe.rs`, `galileo_floor_external_te.rs`,
`lsst_fp_*.json` — paralleler anderer Arbeitsstand).
