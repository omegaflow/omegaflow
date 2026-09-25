<!--
  title: Handover — Mountain-Folge 162 (2026-09-25)
  session: Mountain-Folge 162
  class: handover
  date: 2026-09-25
  sha256: 3b08a7f0bdb5785ccda5178ea2ea610f1c6d5023bca836bbbe739691426c2521
  status: live
-->
# Handover — Mountain-Folge 162 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Sortierung: erst Akteur (Linie | Rat | Operator | Dritter), dann
chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt: Trigger / Lage /
Blockade / Braucht.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### CI-Red `clippy` — identische if-Blöcke `src/archivar/port.rs:1731`
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `read`/`git blame`) die zwei Zweige
  `kl.contains("eccentricity")` und `kl == "rho_cos_phi" || kl == "rho_sin_phi"`
  tragen identisch `("gravity","1",604800.0)`; clippy `if_same_then_else` rot in
  `ci-check 36152720260`. Ursprung `c4592e347a` (Mountain folge159).
- **Blockade:** keine
- **Braucht:** beide Bedingungen in einen Zweig mergen
  (`kl.contains("eccentricity") || kl == "rho_cos_phi" || kl == "rho_sin_phi"`);
  `cargo check` 0 Fehler/0 Warnungen; clippy grün im CI-Lauf (lokal denied).

#### CI-Red `test` — `test_diagnose_no_samples` `src/archivar/tests.rs:6836`
- **Status:** autonom (Urteil nötig) | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `read`) das Fixture
  `"\u{1f}\u{8b}gzip payload"` kodiert `\u{8b}` als UTF-8 `C2 8B`; die gzip-Magic
  `[0x1f,0x8b]` (`fetch.rs:590`) matcht nicht → Assertion erwartet `format-gap`,
  bekommt `data-present`. Ursprung `0881a2751` (Mountain folge161). **Konfund:**
  `&str` kann nie mit `1f 8b` beginnen (0x8b = Continuation-Byte); `body` entsteht
  via `String::from_utf8_lossy` (`fetch.rs:57/78/234/1065`), also `1f EF BF BD` —
  der gzip-Byte-Zweig in `diagnose_no_samples` (wie `fetch.rs:1061`) ist aus dem
  `&str`-Pfad unerreichbar; gzip greift real nur über `url.ends_with(".gz")`.
- **Blockade:** Entscheidung toter Pfad vs. Byte-Pfad
- **Braucht:** (a) `diagnose_no_samples` auf `&[u8]` umstellen **oder** (b) toten
  gzip-Zweig + Assertion entfernen bzw. `format-gap` über erreichbares
  `PK\x03\x04` (`fetch.rs:593`) testen; gefundener toter Pfad wird Gate-Fixture +
  Gate-Test im selben Atom.

#### Klassen-Träger (Register-Ledger)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `sgrep -c "gap <token>" phi/blocked_sources.φ`)
  `phi/blocked_sources.φ::gap:unit-auto-detect ×168`, `::gap:force-undetermined ×16`,
  `::gap:konverter ×4`, `::gap:curation ×15`; `::gap:votable-reader ×2` und
  `::gap:astrometry-reader ×6` sind aufgelöst (die 2 VLASS-Einträge tragen den arm-genauen
  Token `curation` — der Reader-Arm steht; die 6 VizieR-Astrometrie-Serien sind gemessen
  `descoped`). Die Registerkopf-Legende der `gap`-Token steht (Zeilen 2–8).
- **Blockade:** keine
- **Braucht:** beim nächsten Dispatch je Klasse den Arm bauen oder den Eintrag mit Messung
  `descoped` stellen; Klassen-Träger-Form `phi/blocked_sources.φ::gap:<token> ×N` mit
  N = live count (`sgrep -c`, Count-Drift meldet der Scanner).

### Wartend

#### `arxiv` HTTP 406 — serverseitig
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** arXiv schließt die API-Migration ab
- **Lage:** (gemessen 2026-09-25) 406 mit leerem Body, UA-unabhängig, für jede ungecachte
  Query; gecachte Queries 200. Retry für 406 entfernt.
- **Blockade:** arXiv-Edge
- **Braucht:** Wiedervorlage bei Trigger; kein Code.
