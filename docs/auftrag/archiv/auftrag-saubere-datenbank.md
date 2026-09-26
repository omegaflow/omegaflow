<!--
  title: Auftrag — saubere Datenbank (sources.φ / CI-Jobs / CDN-Assets vereinheitlichen)
  class: auftrag
  date: 2026-09-03
  status: archived
  version: 1
  sha256: f483b03f95fd8156745bd668c416d287e99c4a995ebf82ab59e9012d98053f9a
  see-also: docs/surveys/survey-2026-09-03-daten-holdings-inventur.md,
            docs/auftrag/auftrag-verify-references-regelrunde.md, AGENTS.md
-->

# Auftrag: saubere Datenbank

Eine einzige, konsistente Basis über drei Ebenen: **sources.φ (die eine
Registry), die CI-Manifestation, die CDN-Assets**. Ziel ist eine Datenbank,
in der jede Quelle genau eine Release hat, jede Release genau eine Quelle,
jeder Asset-Name kanonisch ist und keine Workflow-Klasse dupliziert.
Die Verlegung folgt der heute (2026-09-03) etablierten CDN-Manifest-Duty
(AGENTS „CDN-Manifestation — eine Session-Duty"; Granit-Grundsatz 7).

## Ausgangsmessung (2026-09-03)

Gemessenes Chaos als Startbasis (nicht zu wiederholen, aber zu verifizieren):

- **CDN `omegaflow/sources`:** 200 Release-Tags (= Netlocs), 2904 Assets, ~196
  Netlocs. Anomalien: 4 leere (0-Asset-)Releases; 3 doppelte Netloc-Tags
  (`opendap.nccs.nasa.gov`, `ned.ipac.caltech.edu`, `isc.ac.uk`); 70 Assets
  mit Doppel-Endung (`*.json.json`), ~20 literal `json.json`; Pfad-/Paginiere-
  reste als Namen (`1.json`, `100.json`, `1234.json`); Prozent-/MIME-kodierte
  Namen (`SELECT_20TOP_…`, `application_2Fsolr_2Bjson.json`,
  `-180_-60_180_60.json`); GitHub-Repo-Quellen als Tag
  (`github.com`, `raw.githubusercontent.com`, `github.com-…GeoNuclearData`).
- **CI:** ~17 Workflows unter `.github/workflows/`, 4–24 Jobs je Datei;
  Ausreißer `health-check.yml` (20) und `kernel-flatten.yml` (24). Die
  Kategorie-Namenskonvention (`X-cdn` / `X-cdn-watch`) existiert, ist aber
  nicht durchgängig/duplikationsfrei.
- **Registry:** `phi/sources.φ` ist die vorgesehene eine Wahrheit; der
  Registry↔CDN-Abgleich (welche Release hat keine Quelle, welche Quelle keine
  Release, welcher Asset-Name weicht vom kanonischen `source_name_from_url`
  ab) ist **nicht** durchgeführt.

## Auftrag (was die neue Session tut)

Der Auftrag wird in Schritten ausgeführt; jeder Schritt steht allein und wird
committet. Die Reihenfolge ist verbindlich.

1. **Registry↔CDN-Abgleich (nicht-destruktiv, Maschinenbasis).** Eine
   Abgleich-Engine/Tabelle erzeugen: je Release (Netloc) ↔ Quelle in
   `phi/sources.φ`; markiere Orphan-Releases (keine Quelle) und Quellen ohne
   Release; je Asset den kanonischen Namen (`source_name_from_url`) gegen den
   tatsächlichen abgleichen. Ausgabe: maschinenlesbare Tabelle.
2. **Ziel-Schema festlegen (eine kurze Spezifikation).** Die verbindlichen
   Konventionen: Name = flacher Pfad ohne Roh-URL-Reste (`json.json`,
   `1.json`, `_2F`, `.php.json` sind verboten); jede Quelle genau eine Release;
   Netloc = Domain (kein GitHub-Repo als Tag); CI je Quelle genau eine
   `X-cdn`-Klasse, keine Duplikate.
3. **Registry zuerst aufräumen.** `phi/sources.φ` zur einzigen, abgeglichenen
   Wahrheit machen; Duplikate/Fehlbenennungen dort beheben (Registry ändern,
   nicht Assets raten).
4. **CI deduplizieren/schlanken.** `health-check.yml` (2026-09-03: 4 Jobs) und
   `kernel-flatten.yml` (2026-09-03: 18 Jobs) zerlegen oder verschlanken;
   Job-Kategorien vereinheitlichen; jede manifestierende Workflow-Klasse
   speist genau die Releases aus der Registry. Konkretisierung: „Schritt 4 —
   CI-Dedupe (konkretisiert 2026-09-26)" unten.
5. **CDN-Assets kanonisch machen — NUR mit Nachbau-/Sicherungsquelle.** Ein
   umzubenennendes/gelöschtes Asset wird erst angefasst, wenn die Quelle es
   nachbaut (Compiler/`--ci-mode`) oder der Inhalt anderweitig gesichert ist.
   Müll-Zwillinge (`x.json.json`) und Orphans nur dann entfernen. Kein
   Blindwurf über die 2904 Assets.

## Schritt 4 — CI-Dedupe (konkretisiert 2026-09-26)

Gemessen (2026-09-26): `.github/workflows/` trägt 315 `*.yml`-Klassen. Die
zwei manifestierenden Hauptklassen sind gegenüber der Ausgangsmessung
(2026-09-03: health-check 4 Jobs, kernel-flatten 18 Jobs) verschlankt:

- `health-check.yml` — **3 Jobs**: `verify` (16-Shard-Matrix), `reverify`,
  `pages-verify`. Baseline 4; `probe-full` ist descoped (2026-09-25, Grund im
  Datei-Kommentar).
- `kernel-flatten.yml` — **5 Jobs**: `index`, `bodies`, `jwst-spectra`, `eve`,
  `aia`. Baseline 18; `catalogs`, `twomass_bulk`, `chunk_catalogs`,
  `ned_chunks`, `sun`, `solar_xrs`, `goes_r_xrs`, `euvs`, `cmb`,
  `long_window_probe`, `solar_causal_graph`, `wso_hmi_consistency`,
  `pioneer_telemetry` liegen in eigenen `*-cdn.yml`.

Der erste Teil (die zwei Hauptklassen verschlanken) ist damit erledigt. Offen
ist der zweite Teil: über alle Klassen zu messen, dass keine Klasse eine
andere dupliziert und jede manifestierende Klasse ihre Release-Menge aus der
Registry liest (CDN_ZIEL_SCHEMA §3).

**Auftrag:**

1. **CI-Klassen-Zensus** (Maschinentabelle, nicht Prosa): je
   `.github/workflows/*.yml` — Datei | Klasse
   (`manifest`/`probe`/`build`/`register`) | Job-Namen | erzeugte
   Release-Menge (Netlocs, aus dem Compiler/`--ci-mode`-Aufruf) |
   Registry-Zeile. Nur `manifest`-Klassen unterliegen §3
   (`X-cdn`/`X-cdn-watch`).
2. **Duplikat-Messung:** zwei `manifest`-Klassen mit demselben Netloc =
   Duplikat (§1: ein Netloc = eine Release) → in eine `X-cdn`-Klasse führen.
3. **Registry-Bindung:** je `manifest`-Klasse prüfen, dass die erzeugten
   Releases als `url`-Zeile in `phi/sources.φ` stehen (registry-first) und
   die Klasse den Netloc aus der Registry liest, nie einen eigenen Tag-Satz.
4. **Abnahme:** 0 Netloc von ≥2 Klassen erzeugt; jede `manifest`-Klasse liest
   aus der Registry; Benennung `X-cdn`/`X-cdn-watch`.

## Regeln (verbindlich)

- **Nie die letzte Kopie zerstören.** Vor jedem CDN-Entfernen: Nachbau-Quelle
  oder Sicherung nachweisen. `0 honored` — ein nicht nachbaubares Asset bleibt
  `pending`, wird nicht gelöscht.
- **Manifestiert, nicht nur lokal** (Granit 7): jede neue/veränderte Quelle
  wird in `phi/sources.φ` registriert, damit CI sie ins CDN bringt.
- **Registry zuerst:** Was im CDN falsch heißt, wird an der Registry korrigiert
  und von CI manifestiert — nie direkt am CDN geraten.
- `cargo check --all-targets`: 0 Fehler, 0 Warnungen nach jeder Code-Änderung.

## Abnahme

- Abgleich-Tabelle (Schritt 1) liegt committet vor.
- Ziel-Schema (Schritt 2) ist eine versionierte Spezifikation.
- `sources.φ` ↔ CDN sind abgeglichen: keine Orphan-Releases, keine Quelle ohne
  Release, keine Roh-URL-Reste in Asset-Namen (außer mit dokumentierter
  Nachbau-Quelle als `pending`).
- CI: keine duplizierten Job-Klassen; jede manifestierende Klasse liest aus
  der Registry.
