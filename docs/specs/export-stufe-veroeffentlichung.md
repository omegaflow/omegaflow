<!--
  title: Export-Stufe — docs/paper/*.md → arXiv/Journal-LaTeX (Workflow)
  class: concept
  date: 2026-08-27
  status: live
  see-also: docs/concepts/exzellenz-konzept.md tools/science/src/bin/export_latex.rs
-->

# Export-Stufe: von omegaflow zur Welt

Der Maßstab ist das Exzellenz-Konzept §5.2: `docs/paper/*.md` ist das Kratzfeld
der Wahrheit; für arXiv/Journal entsteht daraus eine **Export-Form** (LaTeX/PDF),
die die Zahl unverändert trägt. Der Export ist selbst eine Messung — kein
Runden, kein Weglassen, keine Neuformulierung.

## Beleg-Erkennung (externes Register)

Zusätzlich zur Zahlen-Messung prüft die Export-Umgebung die zitierten
Referenz-Kennungen gegen die externe Welt — als **Beleg, nicht als Quelle für
Zahlen oder Verdikte** (A = A):

```bash
# arXiv-IDs gegen die arXiv-API, DOIs gegen den globalen doi.org-Handle-Auflöser
cargo run -p omegaflow-tools --bin reference_verify
```

Ein zitierter DOI oder eine arXiv-ID ist `resolved` (registriert, mit Titel/
Autoren), `absent` (nicht im Register) oder `pending` (kein Netz). Eine `absent`-
Kennung wird benannt, nie ersetzt. Die Welt misst mit — sie liefert den Beleg,
dass eine zitierte Kennung existiert, und bestätigt den arXiv-Referenz-Form
(§5): Referenzen sind belegt, nicht behauptet.

## arXiv-Anbindung (lesen + einreichen)

Die arXiv-Verbindung hat zwei Seiten:

- **Lesen/Ernten** (`arxiv`): `arxiv id <id>…`, `arxiv search <query> [--max N]`,
  `arxiv oai <set> [--from YYYY-MM-DD]`. Löst IDs/Suchen über die öffentliche
  Atom-API (`export.arxiv.org/api/query`) und erntet Metadaten über OAI-PMH
  (`export.arxiv.org/oai2`). Beleg + Ernte, nie Messung — ein leerer Treffer ist
  `absent`, ein Netzfehler `pending`. Rate-Limit: eine Anfrage je 3 s.
- **Einreichen** (`arxiv_submit <slug>`): bereitet ein Gate-konformes Paper zur
  Einreichung vor und verweigert das Erfinden. Ohne `ARXIV_TOKEN` (env oder
  `.secrets.local`) ist es `pending` — Konto, Endorsement und Token sind
  Leitstellen-Arbeit (§6, der externe arXiv-Konto-Autor-Akt). Der
  Einreichungs-Endpunkt ist `pending` benannt: die öffentliche arXiv-API-Doku
  trägt keinen Einreichungs-Endpunkt; der benannte Weg ist das Web-Verfahren
  („Submit TeX/LaTeX") oder die Dritt-Einreichung. Es wird nichts gesendet, bis
  die Leitstelle Konto und Endpunkt benennt.

## Das Werkzeug

`tools/science/src/bin/export_latex.rs` (Rust std-only, keine externen Crates) liest
einen Paper-Body, verwirft den HTML-Header und überführt den Rest 1:1 nach LaTeX.

```bash
# alle Paper nach docs/paper/export/<slug>.tex
cargo run -p omegaflow-tools --bin export_latex

# ein einzelnes Paper
cargo run -p omegaflow-tools --bin export_latex -- docs/paper/planet-nine-kbo-residue.md

# Gate-Modus: nur prüfen, nichts schreiben; Exit-Code != 0 bei einer benannten Differenz
cargo run -p omegaflow-tools --bin export_latex -- --check
```

Der Ausgang ist eine Zeile je Paper:

```
EXPORT  title<=75 abstract<=200 nums-match assets  sha-match
planet-nine-kbo-residue   139/139/long  238/long  520  ok  e4d75c59…
```

## Was der Export erzeugt

- **LaTeX-Dokument** (`docs/paper/export/<slug>.tex`, arXiv-konformer Name:
  kebab-Slug, nur `a-z A-Z 0-9 _ + - . , =`). Die Zahl steht im TeX wie im
  Markdown: `10⁻¹⁷` wird zu `10$^{17}$`, `5 × 10⁻³` zu `5 $\times$ 10$^{-3}$`,
  `ϖ` zu `$\varpi$`. Kein Wert wird gerundet oder geändert.
- **Pflicht-Statements** (generiert): Data Availability (CDN-Assets aus dem
  Body + gemessener Body-`sha256`), Code Availability (`src/` + `tools/`, Commit,
  Lizenz), Competing Interests, Author Contributions (pending, bis die Leitstelle
  die Autor:in benennt).
- **Titel aus dem `title`-Header**, **Abstract aus dem `## Abstract`-Abschnitt**
  — beide abgeleitet, nie neu formuliert. Längen werden gemessen und benannt.

## Verifikation (die Messung)

Die Export-Stufe prüft selbst vier Register und nennt jede Abweichung:

1. **Zahlen-Erhalt (A = A):** die Zahlen-Multimenge des Bodies gegen die des
   exportierten Bodys. Ungleich → `DIFF` (nie still behoben).
2. **Titel ≤ 75 Zeichen** und **Abstract ≤ 200 Wörter** — `ok`/`long`.
3. **Body-sha gegen Header-sha:** `sha256` des Bodys
   (`sed '/^<!--/,/^-->/d' | sha256sum`) gegen den `sha256` im Header.
   Ein `sha`-Mismatch benennt einen veralteten Header, kein automatischer Eingriff.

Der `--selftest`-Schalter prüft die eingebaute sha256-Implementierung an den
Testvektoren (leer, `abc`).

## Status-Lesen

Eine Export-Zeile ist `pending`, wenn eine der Zellen nicht `ok` ist:

- `long` unter Titel/Abstract → die Quelle trägt das Maß nicht; **Kürzung oder
  Abstract-Autorik sind Autoren-Leistung**, nie ein Maschinen-Eingriff (A = A,
  keine Neuformulierung).
- `DIFF` unter nums → eine Zahl wäre geändert worden; das ist ein Verstoß und
  bleibt benannt.
- `sha`-Mismatch → der Paper-Header ist nachzuziehen (operator-Entscheidung).

## Das Gate ist permanent aktiv

Die Export-Stufe ist nicht nur ein Einmal-Lauf, sondern ein **permanentes Gate**:
ein Paper, das eine benannte Differenz trägt, bricht den Lauf.

1. **Exit-Code.** `export_latex` verlässt mit Code 1, wenn ein Paper `long`
   (Titel/Abstract), `DIFF` (Zahlen) oder `sha`-Mismatch trägt; `reference_verify`
   verlässt mit Code 1 bei `absent`/`pending`. Der `--check`-Schalter prüft nur
   und schreibt nichts.
2. **CI (`paper-check.yml`).** Bei jedem Push/PR auf `docs/paper/**` (und die
   Gate-Werkzeuge) laufen Export-Stufe + Beleg-Erkennung; eine benannte Differenz
   macht den Job rot und öffnet ein `paper-check`-Issue.
3. **Beim Erstellen (`paper_new`).** `cargo run -p omegaflow-tools --bin paper_new
   -- <slug> [--title "…"] [--date YYYY-MM-DD]` erzeugt ein Paper, das **geboren
   konform** ist: korrekter Header (inkl. `sha256`), leerer `## Abstract`
   (Autoren-Werk, pending), Titel ≤ 75 geprüft. Sofort nach dem Schreiben läuft
   das Gate über das neue Paper; benennt es eine Differenz, wird das Gerüst
   entfernt — kein halbes Paper bleibt liegen.
4. **Pre-Commit-Hook (`.githooks/pre-commit`).** Vor jedem Commit prüft die
   schnelle Export-Stufe (Titel/Abstract/Zahlen/sha) alle gestagten
   `docs/paper/*.md`; eine Differenz blockiert den Commit. Aktiv per
   `git config core.hooksPath .githooks`. Die Beleg-Erkennung (Netz) läuft in CI.

Damit gilt das Gate für jedes Paper — bestehende, neue und künftige — an drei
Stellen: beim Schreiben (Hook), beim Erzeugen (`paper_new`) und permanent (CI).

## PDF-Rendering (nächster, benannter Schritt)

Auf dem aktuellen Baum ist kein TeX-Engine installiert (pandoc, tectonic,
latexmk, pdflatex, lualatex, xelatex — alle pending). Die arXiv-Form ist das
erzeugte `.tex`; die gerenderte PDF ist der nächste Schritt, sobald ein Engine
vorhanden ist (`tectonic docs/paper/export/<slug>.tex`), und muss dieselbe Zahl
tragen (Exzellenz-Konzept §5.2: eine PDF, die eine Zahl anders zeigt als der
Body, ist ein Verstoß).

## arXiv-Versions-Praxis

Eine Korrektur ist eine arXiv-`replace` (Version), nie eine Neuanmeldung; die
Commit-Historie des Papers ist die Versionshistorie. Der Dateiname bleibt stabil
(kebab-Slug), nur der Inhalt wird in einer neuen arXiv-Version nachgezogen.

## Grenzen (benannt, nicht verborgen)

- TeX-ähnliche Fragmente im Prosa-Text (z. B. `e^{iϖ}`) werden als Text escaped,
  nicht als Mathematik geparst — sie kompilieren, aber unschön. Eine
  mathematik-bewusste Inline-Pass ist ein benannter, zukünftiger Lauf.
- Europäische Dezimal-Kommas (urknall-echo `1,2e-1`) bleiben unverändert
  (A = A); die arXiv-Darstellung ist operator-Entscheidung.
- `src/`-Pfade in den „Data and code“-Fußzeilen einiger Paper verweisen auf die
  alte flache Struktur (`src/te.rs`) — deklarierte Doku-Drift, für die
  Reproduzierbarkeit in einer Revision nachzuziehen (siehe survey-2026-08-27-axiom-gate-papers.md).
