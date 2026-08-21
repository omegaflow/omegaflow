# Handover: concepts/-Benennung — kebab-case-Rename + Header-Backfill + Archiv-Aufräumung

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext. Ausgeführt
wird erst auf das Wort „umsetzen" der Kybernautin/des Operators; dieses
Dokument ist der Auftrag, nicht die Ausführung.

## Best Practice — `concepts/`-Benennung

Kurz: **kebab-case Dateinamen** sind der Standard für Markdown im Repo
(URL-/Shell-sicher, case-filesystem-sicher, sortierbar). UPPER_SNAKE ist eine
Code-Konvention, keine Doc-Konvention. ABER — und das ist der Knackpunkt —
die Namen sind an ~50 Stellen **Eigenname**, nicht Pfad: „SOURCES_V2_SPEC §1",
„PARSER_EVALUATION_MATRIX ist SUPERSEDED", „Kontroll-Spec". Das ist kein
Dateiname, das ist ein **Konzeptname**.

Empfehlung: Datei → kebab-case, Konzeptname in Prosa → bleibt UPPER_SNAKE.
Also `docs/concepts/sources-v2-spec.md`, im Text weiterhin „SOURCES_V2_SPEC §1"
(wie `rfc-2616.md` ↔ „RFC 2616"). Präzise mechanische Regel für den Rename:

- **Nur Pfad-/Dateiform umschreiben:** `docs/concepts/NAME.md` → `docs/concepts/name.md`
  und `NAME.md` → `name.md`.
- **Bare Eigennamen unangetastet:** „SOURCES_V2_SPEC §1",
  „PARSER_EVALUATION_MATRIX" (ohne `.md`/Pfad) bleiben.

Kosten ist bekannt und begrenzt: ~34 Umbenennungen in `concepts/` +
Referenz-Pfade in `TODO.md`, `SOURCE_PORT.md`, `reference/*`, `AGENTS.md`,
`concepts/*` (Selbstzitate), `katalog/archeology_gaps_index.φ`. Die
Bare-Name-Stellen (dutzende) werden nicht angefasst.

## Finaler Plan (Entscheidungen: git-only, sha256-Header, kein Helfer, arena → Archiv)

### 1. Konvention (AGENTS.md + docs/README.md)

- **Klassen:** `handover-YYYY-MM-DD-<slug>.md` (→ `archive/handover/`),
  `survey-<slug>.md` bzw. `survey-YYYY-MM-DD-<slug>.md`, `ref-<slug>.md`,
  `concepts/<kebab>.md`, `reference/` nativ.
- **Versionierung:** Git-only. Kein `vN`/`_ancestral`/Hash im Namen.
  Meilensteine → `version:` im Header.
- **Header-Block** (jedes Prosa-Doc, oben):
  ```
  <!--
    title: …
    class: handover | survey | ref | concept
    date: YYYY-MM-DD
    version: <n>          (nur Meilenstein)
    sha256: <hex>
    status: live | consumed | archived
    see-also: …
  -->
  ```
- **sha256-Regel (ohne Helfer):** Hash über den Dateikörper **ohne den
  Header-Block**: `sed '/^<!--/,/^-->/d' <datei> | sha256sum` → Wert eintragen.
  Damit vergleicht man zwei lokale Kopien in einem Befehl.

### 2. Aufräum-Aktion

| Aktion | Ziel |
|---|---|
| 10 Arena-Transkripte | `archive/arena/` (cp + git rm); Zitate in `THE_SEVEN_SPHERES.md`/`KYBERNETISCHE_ASTROPHYSIK.md` auf `archive/arena/` umbiegen |
| `source_curation.md` | `archive/` (verwaister Redirect) |
| `POINTCLOUD-RENDERING_v1_ancestral.md` | `archive/concepts/` |
| `DIE_VIER_SCHILDE` (ohne `.md`) | → `concepts/die-vier-schilde.md` |
| `funding_erkundung.md` | → `docs/surveys/survey-funding-erkundung.md` |
| 34 `concepts/`-Dateien | kebab-case-Rename (Regel oben, Bare-Namen bleiben) |

### 3. Header-Backfill

Pflicht für neue Docs; in diesem Zug nur `docs/handover/` + die 6 `survey-*` +
die neu benannten `concepts/`-Köpfe (der 34er-Rename fügt beim Durchgehen den
Header gleich mit ein). Stehende `reference/`-Drittanbieter (NIST/NAIF/PDFs)
bekommen keinen Header (nativ).

### 4. Reihenfolge

AGENTS+README-Regel → Renames (git mv) → Referenz-Pfade (sed nur auf
Pfad-/`.md`-Form) → Archivierungen → Header-Backfill → `cargo check`
(Doku-only) → Commit.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                        # sauber oder fremde Arbeit nennen
ls docs/concepts/                 # der 34er-Bestand, UPPER_SNAKE heute
cargo check                       # Doku-only, muss 0/0 sein
```

Referenzen (stehend): `AGENTS.md` (Session-Handover-Konvention), `docs/README.md`,
`TODO.md` (Querverweise), `docs/SOURCE_PORT.md`, `reference/*`,
`phi/pipeline/katalog/archeology_gaps_index.φ`.

## Session-Fragen

- Die 34er-Rename-Menge vor dem Zug mit `ls docs/concepts/` verifizieren und
  die exakte Liste im Commit nennen.
- `THE_SEVEN_SPHERES.md`/`KYBERNETISCHE_ASTROPHYSIK.md` auf Arena-Zitate
  durchsuchen und nur die **Pfadform** (`arena/…`/`.md`) umbiegen, keine
  Bare-Namen.
- `DIE_VIER_SCHILDE` (ohne Endung): Inhalt prüfen, ob er ein Concept ist
  (dann `concepts/die-vier-schilde.md` + Header) — benannt, nicht geraten.

## Gates

- cargo check 0/0 (Doku-only — es ändert sich kein Rust, nur Pfade/Prosa).
- Referenz-Pfade vollständig umgebogen; `grep -rn 'SOURCES_V2_SPEC\.md\|\.\./concepts/[A-Z]'` leer für die Pfadform.
- sha256-Header der backfilled Docs via `sed`-Regel selbst gesetzt (kein Helfer).
- Ein Commit; die Registerzeile im TODO, die diesen Auftrag trug, wird geschlossen.

## Nicht anfassen

Bare UPPER_SNAKE-Eigennamen in Prosa, `reference/`-Drittanbieter,
`src/` (Code-Konvention bleibt UPPER_SNAKE), `phi/` außer
`katalog/archeology_gaps_index.φ`.

## Offen für künftige Sessions (registriert, kein Verlust)

- Sample-Budget-Messung (Vorbedingung für GLADE+)
- ω-Loop-Fetch-Sturm-Reparatur (Retry-Exponent / Pausen)
- GLADE+/NED/2MASS-Atome
- ein voller grüner `chunk_catalogs`-Lauf
