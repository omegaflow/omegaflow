# Source-Port: Der eine Pfad

Selbsttragend. Interpretierbar von einer Session mit null Vorkontext. Dieses
Dokument ist der einzige Einstiegspunkt für alle Source-Arbeit (Grind, Port,
Kuration). Eine neue Session liest genau dieses eine Dokument.

## 1. Was die Arbeit ist

Die gold/pre-cdn-Korpora (alte Grammatik, teilweise gitignored-historie) werden
mechanisch nach v6 überführt (Konvertierung), gegen die lebenden APIs
verifiziert (Probe + Extract) und in eines der zwei Register disponiert.
Daneben: neue Kandidaten aus dem Recherche-Bestand grinden. Der Reichtum sind
nicht nur die URLs — Katalog-Inventare, Domänen-Maps und Kandidaten-Listen
liegen im Bestand und sind im Ledger registriert. Kein offener Posten ohne
registrierten Ort.

## 2. Pfadkarte

| Pfad | Was es IST |
|------|------------|
| `phi/port/queue/` | Portierbare Korpora, alte Grammatik. Eine Datei = ein Korpus. Getrackt, sichtbar. |
| `phi/port/park/` | Geparkte Kandidaten mit Parser-Gap (jeder mit Gap-Verweis). |
| `phi/port/stage/` | Konvertierungs-Ausgänge `<korpus>_converted.φ` + Sweep-Ergebnisse `staging_verified.φ` / `staging_empty.txt`. |
| `phi/port/ledger.φ` | DAS Zustands-Register. Jeder offene Posten mit Zustand. |
| `phi/port/index.φ` | Der Index der zu portierenden Dateien (eine Zeile je Queue-Datei: Zustand + Blockzahl). Regenerierbar. |
| `phi/port/prompt.φ` | Port-Vorlage für Agenten (Korpus → Disposition). |
| `phi/research/` | Der lebende Bestand: Katalog-Inventare, Kandidaten-Listen, abgeschlossene Grind-Wellen (read-only, Historie bleibt). |
| `phi/sources.φ` | Das kanonische Register (Annahme-Ziel). |
| `phi/dead_sources.φ` | Das Dispositions-Register (dead/parser-def/decline/key-needed). |
| `/home/johannes/projects/archive/` | Externes Archiv: die pre-cdn-Historie (ehemals `archeology/`), inkl. CI-Vorlagen. |
| `docs/concepts/SOURCES_V2_SPEC.md` | Die Kontroll-Spec (Grammatik, τ-Gate, Force-Unit-Registry, File-Regeln). |

## 3. Die zwei Register

`phi/sources.φ`: kanonisches Format nach SOURCES_V2_SPEC §1.0 — Blöcke
sortiert nach `ttl` (aufsteigend), dann nach `url`; keine Kommentare; eine
Leerzeile zwischen Blöcken; Direktivenreihenfolge: `url`, Blockzustand
(`ttl`, `format`, `header`, `post_body`, `at`/`on`), Fanout-Familie, dann
Extracts in Abhängigkeitsreihenfolge.

`phi/dead_sources.φ`: Einträge sortiert nach `url`; eine Dispositionszeile,
eine `url`-Zeile, eine `note`-Zeile; Leerzeile zwischen Einträgen; keine
Duplikate pro URL.

## 4. Zustandsmaschine + ledger.φ

Zustände: `ausstehend | verifiziert | void | geparkt | disponiert`.

- `ausstehend` — unberührt, offene Arbeit
- `verifiziert` — Sweep lief, Samples extrahiert, Merge in die Register steht aus
- `void` — Sweep lief, alle URLs void (diagnostiziert) — Korpus ist erschöpft
- `geparkt` — Kandidat wartet auf einen Parser-Gap (Gap im Eintrag benannt)
- `disponiert` — in `sources.φ` / `dead_sources.φ` eingegangen; Eintrag wird entfernt (Git trägt ihn)

Ledger-Format (wie dead_sources.φ, φ-Textformat):

```
<zustand>
<art> <wert>
note <was IST — Herkunft, Umfang, Gap-Verweis>
```

Arten: `queue` (Korpus-Datei), `kandidat` (URL), `bestand` (Recherche-Asset),
`nachlauf` (verlorenes Session-Artefakt), `parser-gap` (P-Liste-Eintrag).
Beispiel:

```
ausstehend
queue phi/port/queue/sources_astro_untested_30-astro.φ
note 30 Blöcke, alte Grammatik — kein URL im kanonischen Register; --gold + Sweep, dann Disposition
```

## 5. Workflow-Prozedur (kein neuer Parser-Modus)

Die Kette existiert bereits; das Protokoll kettet sie nur. Pro Korpus:

1. `cargo run -- --gold phi/port/queue/<korpus>.φ phi/port/stage/<korpus>_converted.φ`
   — mechanische Old→New-Konvertierung (Lautsignal: der Konverter meldet
   Blöcke, die nicht parse-fähig sind; `source`-Köpfe, `method`, `pos` und
   unbekannte Direktiven fallen — siehe §9).
2. `cargo test -- test_backlog_batches_verify -- --nocapture` — der Sweep
   liest `phi/port/stage/*_converted.φ`, substituiert Templates mit der
   Fixture, fetcht (Secrets via `render_headers`), extrahiert mit
   `extract` + `diagnose_no_samples` und schreibt `staging_verified.φ`
   (Samples da) bzw. `staging_empty.txt` (`void <url> <diagnose>`).
3. Disposition: verified-Blöcke nach §1.0-Regeln in `phi/sources.φ` einbauen;
   void/decline in `phi/dead_sources.φ`; `ledger.φ`-Eintrag aktualisieren oder
   entfernen (disponiert). Erschöpfte Queue-Datei aus `queue/` entfernen.
4. `cargo check` 0/0; ein Commit, der TODO.md im selben Schritt aktualisiert.

Per-Block-Kuration (neue Kandidaten, nicht mechanisch):
URL-Templates füllen → `curl`-Erreichbarkeit → Struktur prüfen (200er-HTML ist
kein Daten-JSON) → Force-Gate → Klassifikation → Disposition.

Die kanonischen Register werden nie gemirrort — nur `--verify phi` spiegelt
(CI). `phi/port/` und `phi/research/` sind fetch-only (kein Mirror, Quota).

## 6. Grammatik-Kurzfassung

Bindend: SOURCES_V2_SPEC §1. Die lebenden `field`-Formen: 5 Token
(`field <key> <force> <unit> <tau>`), 6 Token (mit Kernel), 9 Token
(`field <key> <name> <kernel> <force> <unit> <tau> <absorption> <advection>`).
Der 3-Token-`field` und die `force`-Direktive werden laut abgelehnt (P01).
τ = 0 schließt das Gate (kein Oszillator, 0 honored). `on <body> <lat> <lon>
[alt]` (Meter), `at <body>`, `body <body>` + `lat`/`lon`-Keys (datengetragen).
Extracts: `map/cmap/rows/flatten`, `first/last/lastrow/lastline/lastobj/
objlast/path/deep/regex/count`, `geojson/cmrpolygon/celestialpolygon/
keplermap/hapi/ephemeris/vectors`; cmap: `ra/dec/plx/pmra/pmdec/radvel/
dist/dist_scale/z`.

## 7. Referenz-Karte

| Dokument | Rolle | Status |
|----------|-------|--------|
| `docs/concepts/SOURCES_V2_SPEC.md` | Kanonische Grammatik, τ-Gate, Force-Unit-Registry, File-Regeln | bindend |
| `docs/reference/FORCE_SYSTEM.md` | 9 Kraftkanäle, IDs, Ausbreitungsgeschwindigkeiten | bindend |
| `docs/reference/URL_TEMPLATES.md` | Template-Variablen (Spatial/Temporal) | bindend |
| `docs/reference/CONSTANTS.md` | Φ, c, Konstanten | bindend |
| `docs/reference/TIME.md` | Zeit-/Epoch-Handling | nachschlagen |
| `docs/concepts/SI_UNITS.md` | Force-Unit-Matrix (SUPERSEDED als Kontrolle, physische Referenz) | nachschlagen |
| `docs/reference/NIST_SP330_tables.md`, `NIST_SP811_units.md`, `ucum-essence.xml` | Unit-Identität (BIPM/UCUM) | nachschlagen |
| `docs/concepts/PARSER_MAGIC.md` | Parser-Intelligenz + offene Lücken (P05/P07/P08) | nachschlagen |
| `docs/reference/naif_body_ids.tsv` | Bodynamen für Frames | nachschlagen |
| `docs/concepts/DOMAIN_COVERAGE.md` | Host-Manifest der Grind-Wellen | nachschlagen |
| `docs/reference/EXTRACT_TYPES.md` | Extract-Enum (veraltet) | SUPERSEDED |
| `docs/concepts/PARSER_EVALUATION_MATRIX.md` | 4-Token-Behauptung (widerspricht P01) | SUPERSEDED |

## 8. Force-Gate + τ + Klassifikation

Force-Gate: `force` deklariert die physikalische Ausbreitung DER MESSUNG
selbst, nicht das Transportmedium der API. Litmus: könnte ein
nicht-menschlicher Organismus ein Sinnesorgan für diese Messung evolvieren?
Automatisch decline: nackte Zählwerte ohne Ereignis-Records, Stationslisten/
Register/Kataloge ohne Messwert, Modell-Forecasts/Reanalysen, Referenz-
Konstanten, aggregierte Indizes (skalar, positionslos), Text-Warnungen,
abgeleitete Satellitenprodukte, geographische Infrastruktur, position-only.

τ: temporal decay des Prozesses in Sekunden; schnell wechselnd → ttl/10,
stabil (Kataloge, Geologie) → ttl; explizites Prozesswissen schlägt beides.
Kernel-Vorgabe pro Force (Konverter): em/gravity → inverse-square, acoustic/
seismic-body/diffusion → gaussian-inverse-square, seismic-surface → erfc,
thermal → exponential-decay, advective → patch-levy.

Klassifikation: (accepted) → `sources.φ`; `dead` (Endpoint weg) →
`dead_sources.φ`; `parser-def` (Format unkonsumierbar) → `dead_sources.φ` mit
Gap-Verweis (oder `park/` bei Block-Draft); `decline` (Force-Gate) →
`dead_sources.φ`; `key-needed` (kostenfreie Registrierung) → `dead_sources.φ`.

## 9. Fix-Rezepte + API-Fakten (hart erworben)

- `map` braucht datengetragene Position: `lat`/`lon`-Keys oder GeoJSON
  `geometry.coordinates.N`. Positionslose Feeds → skalar (`last <dot.path>`,
  `lastrow`, `lastobj`) an einem Frame.
- `map`-Extract mit nicht-surface Frame (`at sun`) ist eine Fehlkennzeichnung
  — Migration hatte `wgs84`-Blöcke falsch gelabelt; surface-Frame heißt
  `on <body>`.
- ArcGIS: HTTP 200 mit `{"error":{"code":400}}` bei unbekannten outFields →
  `outFields=*` zuerst; Schema-Epochen wechseln (`_1_` vs `_0_`-Felder).
- Open-Meteo: Split-Hosts (`archive-api.`/`flood-api.`/`marine-api.`/
  `air-quality-api.`/`ensemble-api.`), nie `api.open-meteo.com/v1/<x>/v1/<x>`.
- TAP: `FORMAT=csv/json` nicht überall; VOTable-only-Kataloge = parser-def;
  `SELECT TOP 1 *` vor jedem Query; TAPVizieR kennt kein OFFSET/LIMIT.
- HAPI: BGS braucht `format=json` + `Z`-Suffix an Timestamps; VirES kennt
  `id=/start=/stop=` UND `dataset=/time.min=`; Fenster innerhalb der
  Abdeckung wählen (`{yesterday}`-Fenster + ttl 86400 bei Lag).
- ERDDAP: Constraint-Syntax des jeweiligen Servers prüfen (`[(last)]`-Formen
  400en heute oft).
- Header-Werte sind whitespace-gesplittet (keine Spaces); Secrets als
  Einzel-Marker (`header user-agent {TNS_UA}`) — Braces nicht mischen.
- Refused-Befunde des Ladens sind Source-Befunde, nie Parser-Fixes:
  `pos without body directive` → Block trägt `body <body>`; `no reference
  frame` → Block braucht `on`/`at`.
- Der `--gold`-Konverter übernimmt: `url/format/header/target/catalog/
  flux_from_mag/abs_mag_from/catalog_epoch` direkt; `ttl`; `on/at`;
  numerisches `lat/lon/alt` → synthetisches `on earth`; `map/cmap/rows`;
  `lat_key/lon_key/alt_key/epoch_key` → `lat/lon/alt/epoch`; `field/field_in/
  first/last/count/path/deep` → 9-Token-`field`; `last_row`→`lastrow`,
  `last_line`→`lastline`, `last_obj`→`lastobj`, `regex` (Join), `geojson`
  (tau = ttl/10). **Fällt**: `source`, `method`, `body` (POST), `pos`,
  unbekannte Direktiven — solche Blöcke sind `park/`-Kandidaten mit
  Gap-Verweis (post_body-Migration ist offene Arbeit).

## 10. Secrets

`.secrets.local` (Repo-Root, gitignored, 46 Keys, GitHub-Parität) — geladen
via `resolve_asset` (CWD-relativ). Probe-Läufe außerhalb des Repo-Roots
brauchen einen Symlink. `{MARKER}` in URLs wird case-insensitiv gegen die
UPPERCASE-Env-Vars aufgelöst; absent → void + stderr.

## 11. Regeln

- Name = Implementation. Keine Kommentare in φ-Dateien; Wissen lebt in
  Protokoll, Ledger und TODO.
- Ein Commit = ein Häkchen: TODO.md im selben Commit; Erledigtes entfernt.
- `cargo check` 0 Errors, 0 Warnings. Der Sweep-Test ist der Verifikator;
  `cargo check` prüft nur Syntax.
- Die Session ist das Atom: ein Korpus pro Session komplett durch die
  Prozedur — kein halber Port.
- 0 honored: eine void-Diagnose ist eine vollständige Disposition, kein
  Fehler. Absent ist erst nach Recherche absent (`ausstehend` bis dahin).
