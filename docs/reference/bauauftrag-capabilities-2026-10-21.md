<!--
  title: Bauauftrag — die Fähigkeiten von archive_search (2026-10-21)
  class: ref
  date: 2026-10-21
  status: open
  see-also: docs/reference/offen-ungebaut.md docs/reference/bauauftrag-2026-10-21.md
-->
# Bauauftrag — die Fähigkeiten von `archive_search`

Die Quellen sind gebaut. Dieser Auftrag schließt die **Engine-Fähigkeiten**:
Ausgabe, Cache, `--all`-Pipeline, Filter, Volltext, Beobachtbarkeit, Proxy,
Profile. Jeder Punkt bekommt **einen bauenden Agenten** (A–H).

## Regeln für jeden Agenten

- Bauen, testen, commiten — kein `absent`/`blockiert` als Ersatz für Bau.
- Keine Kommentare im Code (Hausregel), keine erfundenen Defaults, keine
  stillen Nullen.
- Standard-Textausgabe bleibt unverändert, wenn kein neues Flag gesetzt ist.
- Keine Secrets in Ausgabe, Logs oder Commits.
- Nur die eigenen Dateien stagen; fremde Arbeit nicht anfassen.
- Test: `cargo test --release -p omegaflow-utils --bin archive_search`.
- Live-Prüfung mit `target/release/archive_search`, nicht über `bin/`.
- Commit am Ende des Punktes, nicht pushen (der Hauptagent pusht nach Review).

## A — Ausgabe und Exit-Codes

**Ziel:** maschinenlesbare Ausgabe und ein Status, den Skripte verstehen.

**CLI:** `--jsonl`, `--csv`, `--bibtex`, `--ris`, `--allow-empty`.

**JSONL:** eine Zeile pro Treffer:
`{"mode":"openalex","url":"…","fields":{"title":"…","doi":"…"},"via":["4get"]}`
Nur Treffer, keine `pending`/`absent`-Zeilen.

**CSV:** Kopf `mode,url,title,doi,year,authors,venue,pdf,via`; RFC-4180-Quoting.

**BibTeX/RIS:** aus vorhandenen Feldern; Key aus DOI, sonst URL-Hash.
Keine Felder erfinden; fehlende Felder entfallen.

**Exit:** `0` = mindestens ein Treffer; `3` = nur `pending`/`absent`;
`2` = Nutzungsfehler; `--allow-empty` erzwingt `0`.

**Dateien:** neu `tools/utils/src/bin/archive_search/output.rs`;
`archive_search.rs` (Flags, Ausgabe, Exit); Tests in `output.rs`.

**Abnahme:** `--jsonl` ist mit `jq -c .` lesbar; `--csv` round-trip;
`--bibtex`/`--ris` enthalten nur vorhandene Felder; Exit-Codes 0/3/2
getestet; Textausgabe unverändert.

## B — Cache v2 und Circuit Breaker

**Ziel:** blockierte Arme nicht bei jedem Lauf erneut anfragen.

**Verhalten:**
- Erfolg: TTL 3600 s (bestehend).
- `pending` mit HTTP 400/402/403/429/432: **Negative-Cache 300 s**.
- `absent`: 3600 s (eine Abwesenheit ist ein Ergebnis).
- Pro Host: nach 3 Fehlern 600 s Sperre (persistiert im Cache-Verzeichnis).
- Atomar schreiben (`tmp` + `rename`), Lock gegen parallele Prozesse.
- `--cache-clear`, `--cache-stats` (Treffer, Fehlschläge, Bytes, Hosts).

**Dateien:** `archive_search/cache.rs`, `net.rs` (Hook in `run_lines_max`
und Host-Fehlerzähler), `archive_search.rs` (Flags).

**Abnahme:** zweiter `--brave`-Lauf ohne Netz; `--consensus` wird nach dem
ersten 429 negativ gecacht; `--cache-stats` zählt; `--cache-clear` leert;
651+ Tests grün.

## C — `--all`-Pipeline: Dedupe, Parallel, Gruppen

**Ziel:** `--all` liefert eindeutige, schnelle, gruppierte Ergebnisse.

**CLI:** `--dedupe` (Default an für `--all`), `--no-dedupe`, `--jobs N`
(Default 4), `--group astro|literature|web|data|all`.

**Dedupe:** normalisiere DOI (lowercase, `https://doi.org/` entfernen),
arXiv-ID, URL (Trailing-Slash, `http`/`https`, Tracking-Parameter).
Gleicher Schlüssel → eine Zeile, Felder mergen, `also: openalex,crossref`.

**Parallel:** Worker-Pool über Arme, per-Host-Gate bleibt wirksam,
deterministische Reihenfolge nach Rang; `--jobs 1` und `--jobs 4` liefern
dieselbe Reihenfolge.

**Gruppen:** Mapping in `groups.rs`; `--group` filtert die Armenliste.

**Dateien:** neu `archive_search/dedupe.rs`, `archive_search/pool.rs`,
`archive_search/groups.rs`; `net.rs` (`all_lines`), `archive_search.rs`.

**Abnahme:** doppelte DOI erscheint einmal mit `also`; `--jobs` ändert die
Reihenfolge nicht; `--group astro` enthält nur Astro-Arme; Tests.

## D — Generische Filter

**Ziel:** `--since`, `--until`, `--type`, `--language` statt API-Syntax.

**Mapping (mindestens):**
- OpenAlex: `from_publication_date`, `to_publication_date`, `type`, `language`
- Crossref: `from-pub-date`, `until-pub-date`, `type`
- Semantic Scholar: `year`, `fieldsOfStudy`
- arXiv: `submittedDate`
- PubMed: `mindate`/`maxdate`
- Europe PMC: `PUB_YEAR`
- ADS: `pubdate`

**Nicht unterstützte Arme:** keine stille Filterung. Mit `--explain` eine
`note: filter unsupported by <mode>`-Zeile; sonst unverändert.

**Dateien:** neu `archive_search/filters.rs`; `net.rs` (Query-Builder-Hooks);
`archive_search.rs` (Flags).

**Abnahme:** `--since 2020 --until 2021 --openalex` liefert nur 2020/2021;
Crossref/S2 ebenso; `--explain` nennt nicht unterstützte Arme; Tests für
alle Query-Builder.

## E — OA-Volltext

**Ziel:** Treffer optional mit offenem Volltext anreichern.

**CLI:** `--fulltext` (Default aus), `--fulltext-limit N` (Default 5).

**Verhalten:** pro Treffer mit DOI/URL: Unpaywall (`OPENALEX_MAILTO` oder
`UNPAYWALL_EMAIL`) und OpenAlex `best_oa_location`; Felder `pdf:` und
`fulltext:` ergänzen; Ergebnisse im Cache; ohne Flag kein Netzzugriff.

**Dateien:** neu `archive_search/fulltext.rs`; `archive_search.rs`
(Ausgabe-Hook); Tests mit Fixture.

**Abnahme:** bekannter OA-DOI bekommt `pdf`; ohne `--fulltext` keine
zusätzlichen Requests; Limit greift; Tests.

## F — Beobachtbarkeit

**Ziel:** pro Arm Status, Dauer, Cache, Retry-After sichtbar machen.

**CLI:** `--report <pfad>` (JSON), `--timings`.

**Report:** pro Arm `{mode,status,ms,results,cached,retry_after,via}`;
keine Secrets; `pending` mit HTTP-Status. `--timings` druckt eine kurze
Tabelle nach `stderr` (stdout bleibt Ergebnis).

**Dateien:** `net.rs` (Instrumentierung), neu `archive_search/report.rs`,
`archive_search.rs` (Flags).

**Abnahme:** Report ist valides JSON; `--brave` zeigt 402 und Cache-Status;
`--timings` schreibt nur nach stderr; Tests.

## G — Proxy und Token-Refresh

**Ziel:** Netzzugang und Token-Laufzeit steuerbar machen.

**CLI:** `--proxy <url>` (http/socks5), Env `OMEGAFLOW_PROXY`;
Priorität vor Auto-Proton. Token-Refresh über
`OMEGAFLOW_<KEY>_CMD` (Befehl, dessen stdout den Token liefert), Ergebnis
im Speicher gecacht; kein Token in Ausgabe/Log.

**Dateien:** `net.rs` (Proxy-Auswahl), neu `archive_search/token_cmd.rs`,
`archive_search.rs` (Flag), Tests.

**Abnahme:** `--proxy` wird an curl übergeben (Unit-Test der Argumente);
`*_CMD` liefert Token; ohne Proxy unverändert; Tests.

## H — Profile und Doku

**Ziel:** wiederkehrende Suchen als Profil, Gruppen dokumentiert.

**CLI:** `--profile <datei>` (JSON/φ: Name, Gruppe, Arme, Filter, max);
`--list-profiles`; Profile unter `docs/reference/profiles/`.

**Dateien:** neu `archive_search/profile.rs`, Profile, Doku;
`archive_search.rs` (Flags).

**Abnahme:** `--profile astro-solar` setzt Arme/Filter; `--list-profiles`
zeigt sie; unbekanntes Profil = Exit 2; Doku mit Beispiel.

## Staffel und Reihenfolge

Die Agenten laufen **nacheinander auf demselben Baum**, weil fast jeder
Punkt `net.rs` oder `archive_search.rs` berührt; parallel würden sie sich
überschreiben.

| Punkt | Agent | Dateien (Eigentum) | Status |
|---|---|---|---|
| A | Agent A | `output.rs`, `archive_search.rs` | offen |
| B | Agent B | `cache.rs`, `net.rs` | offen |
| C | Agent C | `dedupe.rs`, `pool.rs`, `groups.rs`, `net.rs` | offen |
| D | Agent D | `filters.rs`, `net.rs` | offen |
| E | Agent E | `fulltext.rs`, `archive_search.rs` | offen |
| F | Agent F | `report.rs`, `net.rs` | offen |
| G | Agent G | `token_cmd.rs`, `net.rs` | offen |
| H | Agent H | `profile.rs`, Profile, Doku | offen |

Nach jedem Punkt: Tests, Review, Commit, Push. Kein Punkt gilt als fertig,
solange nicht `cargo test` grün ist und die Abnahme live gezeigt wurde.
