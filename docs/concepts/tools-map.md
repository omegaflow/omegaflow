<!--
  title: Tools-Map — was jedes Werkzeug kann, was es kostet, wer es darf
  class: concept
  date: 2026-09-17
  sha256: e895153d2d6448ac5bf1d8f352912abf27db3e7e2daf6d7c88e032bcf16a430e
  status: live
  see-also: AGENTS.md
-->
# Tools-Map — was jedes Werkzeug kann, was es kostet, wer es darf

Eine Karte statt dreier sich widersprechender Stellen (gemessen 2026-09-16,
Release-Binaries, Einzellauf je Messung). `osearch` ist gestrichen — es gehört
nicht zum omegaflow-Werkzeugsatz; seine generischen Fähigkeiten (`--count`,
`--case`, `--path`) sind in `archive_search` eingeflossen; `--hexcolor` bleibt
draußen (spezifische Anwendung, kein Such-Werkzeug).

## Die eine Regel (ersetzt alle früheren Inhaltssuche-Vorschriften)

- Inhalt im lebenden Baum: `archive_search <kws> --root <dir>` (auf PATH) oder
  `sgrep [-i] <pattern> [dir]` — `sgrep` ist case-sensitiv ohne `-i`.
- Pfade/Dateinamen: `archive_search --index [<query>] [--path] [--kind] [--sort]`.
- `grep`, `ls`, `cat` **in bash** sind Verstöße (der Zähler gehört ins Handover).
  Das `Grep`-Tool von OpenCode ist erlaubt; bash-`grep` ist es nicht. Die
  Leitbefehle (`grep`/`ls`/`cat` am Kommandoanfang) sind in `opencode.json`
  **strukturell verweigert** (`"grep *": "deny"`); eine Pipe (`cmd | grep`) fällt
  weiter unter die Regel und den Zähler.
- Neu gemessen 2026-09-16 11:00 (opencode.db, Fenster 2026-09-13 09:49 →
  2026-09-16 11:00, gezählt je bash-Kommando): 206 bash-Aufrufe; `grep` 8 (alle 8
  strukturell verweigert — 0 ausgeführt), `ls` 6, `cat` 3; dagegen `curl` 44,
  `sgrep` 37, `archive_search` 20, `sfetch` 10; Playwright-Tool 14, bash-Playwright
  2. Reibung: 18 verweigerte Aufrufe (17 bash, 1 read) — jeder `grep`/`ls`-Griff
  kostete einen Turn, bevor die Sperre griff.
- `curl` 44 > `archive_search` 20 ist kein Verstoß: die 44 tragen REST-APIs
  (TAP/ERDDAP/GitHub), für die `archive_search` keinen Modus hat. Für Quellen und
  JS-Seiten bleibt `archive_search <quelle>` / `--playwright` der erste Zug.
- Das `--help` des Werkzeugs selbst ist die kanonische Schnittstelle — alle
  Aufrufe und Flags stehen dort (`sgrep --help`, `archive_search --help`,
  `git_safety`, `omega_sh help`, `smail`, `register_lookup`). Die
  Interfaces-Sektion unten spiegelt sie; bei Widerspruch gilt das `--help`.

## Gemessen — Inhaltssuche (Query `purpleair`, `src/`)

| Werkzeug | Zeit | Treffer | Urteil |
|---|---|---|---|
| `archive_search --root src/` | 0,051 s | korrekt | kanonisch |
| `sgrep -i` | 0,041 s | korrekt | schlank |
| `sgrep` (default) | 0,075 s | 0 (case-sensitiv) | ohne `-i` fehlt Großschreibung |
| `grep -rni src/` | 0,105 s | korrekt | auf Repo-Wurzel: Hänger (`target/`+`data/`) |

## Gemessen — `sgrep` Modi

`default` 0,075 s · `-i` 0,071 s · `-l` 0,067 s · `-c` 0,067 s · `-g '*.rs'` 0,052 s.

## Gemessen — `archive_search` lokale Modi

| Modus | Zeit | Bemerkung |
|---|---|---|
| `--root` | 0,051 s | Inhalt, Ranking, Caps |
| `--index` | 0,023 s | Pfade, lebender Walk |
| `--git <query>` | 9,98 s | History-Suche — teuer, gezielt nutzen |
| `--leads <kw>` | 0,85 s | Kandidaten-Homes abzüglich Registern |
| `--mft <device>` | 0,003 s | benannter Exit ohne NTFS-Device |
| `--sniff <url>` | 0,16 s | magic bytes + sha256 |
| `--verdict <url>` | 24,2 s | Reichweiten-Leiter mit Fallbacks — nur für Reichweite |
| `--playwright <url>` | 2,97 s | Browser-Render |
| `--pdf-image <file\|url>` | — | hebt eingebettete JPEG/PNG/JP2 aus einem PDF (kein Rasterizer); `--out <dir>` sonst Temp-Verzeichnis |
| `--count` / `--case` / `--path` | — | gebaut 2026-09-16; im Binär nach dem nächsten Build |

**Handoff — PDF→Bild→`vision`:** `archive_search --pdf-image <pdf|url> [--out <dir>]`
hebt die eingebetteten Bilder (`DCTDecode`→`.jpg`, `FlateDecode`→`.png`,
`JPXDecode`→`.jp2`) und druckt die Pfade; die gereichten Dateien gehen an
`vision` (P6, liest nur, kein bash). Kein Bild gefunden → eine `pending`-Zeile,
kein erfundenes Bild.

## Gemessen — `archive_search` Netz-Modi (Einzelaufruf, kleiner Query)

| Modus | Zeit | | Modus | Zeit |
|---|---|---|---|---|
| `--arxiv` | 0,48 s | | `--crates` | 0,56 s |
| `--wiki` | 0,46 s | | `--datacite` | 0,58 s |
| `--librs` | 0,73 s | | `--github` | 0,76 s |
| `--brave` | 1,04 s | | `--heasarc` | 1,4–2,1 s |
| `--ntrs` | 1,38 s | | `--isc` | 1,70 s |
| `--ads` | 2,05 s | | `--wayback` | 2,72 s |
| `--crossref` | 3,48 s | | `--zenodo` | 11,6 s (101 Zeilen) |
| `--openalex` | 10,8 s (101 Zeilen) | | `--supermag` | 3,4 s Daten / 0,54 s Inventory |
| `--all` | Σ der 13 Modi | | — letzte Stufe, nie der erste Zug | |

## Interfaces — exakt (damit niemand rät)

- `sgrep [-i] [-l] [-c] [-g <glob>] <pattern> [dir|file]` — **kein `-n`**; die
  Standardausgabe ist `pfad:zeile:text`; ohne `-i` case-sensitiv; `-l` nur
  Dateipfade, `-c` nur der Zähler. Der Dateisatz kommt aus `git ls-files`.
- `archive_search <kws>... [--root <dir>]... [--lines n] [--files n] [--max-mb n]
  [--skip n] [--binary] [--count] [--case] [--include <glob>]` — default
  case-insensitiv; `--count` druckt `n files, m hits for: …`; `--include '*.rs'`
  filtert auf Dateinamen (der `grep --include`-Ersatz).
- `archive_search --index [<query>] [--path] [--kind any|file|dir] [--sort name|size|mtime]`.
- `archive_search --supermag "station=<code> start=<YYYYMMDDHHMM> end=<YYYYMMDDHHMM>"`
  (Daten) oder `"start=<YYYYMMDDHHMM> extent=<sekunden>"` (Stations-Inventory);
  Logon = `SUPERMAG_USER` aus `.secrets.local` (ohne Konto: Inventory geht, Daten
  nicht). Ein leeres `OK` ist `absent`, kein Fehler.
- `archive_search --heasarc "table=<w3browse-tabelle> rows=<n>"` — echte
  W3Browse-Tabellen, z. B. `table=sao`; `master` existiert nicht (W3Browse sagt
  es wörtlich).

## Gemessen — übrige lokale Werkzeuge

| Werkzeug | Zeit | Bemerkung |
|---|---|---|
| `omega_sh reports` | 0,007 s | 3645 Zeilen (Watchdog-Konkatenat) |
| `omega_sh search` | 0,055 s | `sgrep`-Wrapper |
| `omega_sh fetch` | 0,122 s | `sfetch`-Wrapper |
| `omega_sh jwst` | 0,644 s | CDN-Watch |
| `omega_sh sha <file>` | — | Header-sha256 über den Body ohne `<!-- … -->`-Header (docs-naming) |
| `omega_sh check` | — | `cargo check`-Zusammenfassung (Fehler-/Warnungszahlen) |
| `sfetch` / `curl -s` | 0,113 / 0,073 s | `sfetch` zuerst |
| `smail --dry-run` | 0,288 s | kein Versand |
| `register_lookup --open` | 0,054 s | 659 Zeilen — Planungs-Pass |
| `register_lookup --history` | 1,03 s | 3049 Zeilen |
| `git_safety --snapshot` | 1,20 s | Planungs-Pass |
| `git_safety --list` | 0,017 s | |
| `git_safety --close [<own-path>…]` | — | Commit-Abschluss-Check in einem Aufruf |
| `session_burn` | — | Release-Binär fehlt noch (Baum rot) — bis dahin `cargo run -p omegaflow-register` |
| `./bin/archive_search` (Wrapper) | 0,009 s nach Stempel | baut bei Bedarf, 5-min-Cooldown, Fallback statt Tod |
| OpenCode-Tools (`read`/`grep`/`glob`) | ein Tool-Round-Trip, kein Prozess | Kosten sind Kontext, nicht CPU |

## Werkzeug → kann → darf

| Werkzeug | Kann | Scope | Profile |
|---|---|---|---|
| `archive_search` (PATH, Symlink auf `target/release`) | Inhalt, Pfade, NTFS, 16 Netz-Modi, `--playwright`, `--all`, `--leads`, `--serve`, `--count/--case/--path` | lokal + Netz | P1–P5 |
| `bin/archive_search` (Wrapper) | baut bei Bedarf, sonst `exec` | lokal | P1–P5 |
| `sgrep` | Zeilensuche über `git ls-files` | lokal | P1–P5 |
| `sfetch` / `omega_sh` | fetch / reports-status-search-fetch-jwst-sha-check | Netz / lokal | P3 (fetch), P1 (alle) |
| `smail` | Mail senden (Resend), `--dry-run` | Netz | P1 |
| `register_lookup` | `--live`/`--history` — Register mit OPEN-Zeilen, zustand-/post-Scan | lokal | P4 |
| `git_safety` | `--snapshot/--restore/--list/--watch/--close` | lokal | P1/P4 |
| `session_burn` | Burn je Session (opencode.db) | lokal | P1 |
| OpenCode-Tools | kein Prozess, ein Round-Trip | — | nach Profil |
| `curl` | nur wo `sfetch`/`archive_search` nichts trägt | Netz | P3 |

## Profile (aus AGENTS.md) — gemessene Kosten ihres erlaubten Satzes

| Profil | Agenten | Darf | Messung |
|---|---|---|---|
| P1 | build | edit + full bash | alle oben; teure Builds sind CI |
| P2 | explore, council | git-read + `sgrep` | `sgrep` 0,04–0,08 s |
| P3 | general, research-max | `archive_search`/`sfetch`/`curl`/`proton-wg` + git-read + `sgrep`/`omega_sh` | Netz-Modi 0,5–24 s, lokal 0,02–0,11 s |
| P4 | plan | `register_lookup` + `git_safety` + git-read + `sgrep` | 0,05 + 1,2 + 0,07 s |
| P5 | grind-* | edit + full bash | wie P1 |
| P6 | vision | nichts (kein edit, kein bash) | — |

## Agenten-Benchmark (2026-09-16, identischer Task: dieselben vier Befehle, alle 8 Profile)

Lauf 3, nach der Werkzeug-Grenze in der System Directive und den Struktur-Sperren.
Werkzeug-Zeiten, identisch über alle Profile: `--root --count` 0,026–0,035 s;
`sgrep -i` 0,038–0,069 s; `--index` 0,020–0,037 s; `--crossref` 0,54–3,14 s
(Netz; bei 8 parallelen Aufrufen 2× HTTP 429 — Rate-Limit, als `pending` gemeldet).

| Agent | Modell | Cost | cache_read | Dauer |
|---|---|---|---|---|
| general | flash | $0.0008 | 51k | 6 s |
| explore | flash | $0.0015 | 25k | 6 s |
| plan | flash | $0.0017 | 47k | 5 s |
| grind-flash | flash | $0.0031 | 35k | 7 s |
| research-max | pro | $0.0115 | 53k | 27 s |
| grind-max | pro | $0.0116 | 28k | 19 s |
| council | pro | $0.0117 | 28k | 27 s |
| grind-pro | pro | $0.0119 | 28k | 26 s |

Pro/max kostet 4–15× flash bei identischem Ergebnis — flash-first bestätigt,
Sieger flash (`general` $0.0008). **Reibung gemessen:** Lauf 2 (vor der Grenze)
hatte 1–2 verweigerte Werkzeug-Versuche je Profil (plan/general/research-max);
Lauf 3 (nach der Grenze im System-Directive-Text + den Sperren) hatte **0
Versuche** in allen 8 Profilen — kein Agent probiert mehr ein verbotenes Werkzeug.
`vision` (P6, kein bash) ist mit Kommando-Tools nicht benchmarkbar.

## Benchmark (wiederholbar)

Der komplette Lauf ist diese Datei selbst: jede Zeile der Tabellen ist ein
Befehl — `time <befehl>` je Modus, Einzellauf, `src/`-Query `purpleair` bzw.
die genannten kleinen Queries. Das Skript braucht kein `grep` — nur `time` und
`wc -l`.

## Offen

- `session_burn` auf PATH nach dem nächsten erfolgreichen Build des
  `omegaflow-register`-Release-Binärs.
