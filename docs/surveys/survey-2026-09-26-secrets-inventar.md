<!--
  title: Survey — Secrets-Inventar und Konsumenten-Kreuzung (Stand 2026-09-26)
  class: survey
  date: 2026-09-26
  sha256: 9bbaa6a9676f5c283f7348a83aa817ca8dfa9b7cc8c3db62420e572f73ca1a66
  status: live
-->
# Survey — Secrets-Inventar und Konsumenten-Kreuzung (2026-09-26)

Zweck: welche Secrets aus `.secrets.local` werden gelesen, welche nur registriert,
welche nirgends im Baum referenziert. Gemessen 2026-09-26 über den getrackten Baum
(`git grep`), den `{NAME}`-Template-Resolver aus `phi/*.φ`, das TSV
`tools/measure/free_models.tsv` (`env_var`-Spalte) und den privaten Baum `state/`.
**Werte wurden nie gelesen** — nur die Namen.

## Bilanz

- **122** Secrets in `.secrets.local`.
- **38** im Code gelesen (`src`/`tools`: `secret(...)`, `env.get(...)`,
  `env::var(...)`, Namenslisten via `token(&[…])`).
- **9** nur in `phi/*.φ` als `{NAME}` registriert (kein Code-Leser):
  `AIRQO_KEY`, `FROST_BASIC_AUTH`, `HERMES_TOKEN`, `NASA_API_KEY`,
  `OPENAQ_API_KEY`, `SPACETRACK_USER`, `SSDC_PASS`, `SSDC_USER`, `TRANSIT511_KEY`.
- **43** in tsv/docs/bin referenziert (`free_models.tsv`, `docs/specs/ref-auth-apis.md`,
  `bin/*.sh`, Root) — teils gebaut, teils dokumentierte Absicht.
- **32** nirgends im öffentlichen getrackten Baum; davon **5** im privaten `state/`
  referenziert (`FLY_API_TOKEN`, `GEMINI_API_KEY`, `OPENALEX_API_KEY`, `ZAI_COOKIE`,
  `ZAI_TOKEN`); **27** in beiden Bäumen unreferenziert.

## 27 in beiden Bäumen unreferenziert

```
BABAMUL_KAFKA_PASSWORD  BABAMUL_KAFKA_USERNAME  BABAMUL_PASSWORD  BOREALIS_API_KEY
CEDA_TOKEN  CLOUDFLARE_EMAIL  DAHITI_PASS  EUMETSAT_KEY  EUMETSAT_SECRET
FROST_CLIENT_SECRET  GFW_PASS  GOSAT_GW_MAIL  GOSAT_GW_PASS  IGETS2_PASS
IGETS2_USER  METEOFRANCE_RADAR_TOKEN  METEOFRANCE_TOKEN  MOVEBANK_PASS
MOVEBANK_TOKEN  NCBI_PASSWORD  RUBIN_PASS  tedp_gtfs_rt  tedp_ojp20
tedp_siri_et  tedp_siri_pt  UNOROUTER_PASS  UNOROUTER_USER
```

## Herkunft der 27 — an der Homepage gemessen (2026-09-26)

**Datenquellen-Credentials ohne Disposition** (brauchen einen `phi`-Eintrag — Quelle oder `declined`):
- `GFW_PASS` → Global Forest Watch / Global Nature Watch (WRI), Daten-API `data-api.globalforestwatch.org` (200).
- `GOSAT_GW_MAIL`/`GOSAT_GW_PASS` → **GOSAT-GW** (NIES/JAXA Nachfolgemission); Homepage derzeit `pending` (nur Wayback 2022-09-06), nicht tot.
- `IGETS2_PASS`/`IGETS2_USER` → IGETS (International Geodynamics and Earth Tide Service; GFZ ISDC / EOST) — kein zweiter Dienst „IGETS2".
- `RUBIN_PASS` → Rubin Observatory / LSST.
- `BABAMUL_*` → Babamul Alert-Broker (Caltech / Univ. Minnesota; ZTF+LSST, Kafka) — Datenquelle, keine generic-Infra.
- `MOVEBANK_*` → Movebank (Max-Planck, Animal-Tracking-Datenbank) — Datenquelle, keine Bank-API.

**Keine Datenquelle (Infrastruktur / API-Dienst):** `FLY_API_TOKEN` → Fly.io (PaaS);
`UNOROUTER_*` → UnoRouter (LLM-Modell-Router); `ZAI_TOKEN`/`ZAI_COOKIE`/`GEMINI_API_KEY`/
`CLOUDFLARE_*` → LLM/Infra.

**Korrektur der ersten Fassung:** Die Aussage „überwiegend declined/dead" war falsch.
`GFW`, `GOSAT-GW`, `IGETS2`, `BABAMUL`, `MOVEBANK` sind lebende Datenquellen ohne
Disposition; `RUBIN` gehört in den Bestand (termin 2026-12-02, `folge129`). Die
Quellen-Präfixe sind nur ein unscharfes Signal — die Identität wurde an der Homepage
gemessen, nicht aus dem Namen geraten. Die **Secret-Namen selbst** stehen in keinem Register.

## Vorbehalte

- `OPENALEX_API_KEY`: zum Messzeitpunkt ohne Code-Leser; am 2026-09-26 wurde der
  `api_key`-Arm in `tools/utils/src/bin/archive_search/openalex.rs` verdrahtet
  (`&api_key=` bei vorhandenem Secret) — damit 39 Namen im Code.
- Die 43 tsv/docs-Treffer sind nicht alle gebaute Konsumenten; ein Teil ist
  dokumentierte Absicht.
- `ZAI_TOKEN`/`ZAI_COOKIE`/`GEMINI_API_KEY` werden von den **privaten** Skripten
  `state/stimmen/stimme.sh` gelesen (nicht getrackt).
- Der Abgleich ist eine statische Messung; dynamisch gebildete Namen sind als
  Sonderfall benannt, nicht geraten.

## Geschlossen (2026-09-27)

Der Verdict über die Namen ist gefällt (gemessen 2026-09-27 via `sgrep` + GH-API;
die „6" zählten die Lebendquellen, nicht die Namen). Die Quellen sind disponiert
(GOSAT-GW `phi/sources.φ` `gosat_tanso3`, IGETS `phi/sources.φ` `igets`, Babamul
`phi/sources.φ` `babamul` — registriert; GFW `phi/declined_sources.φ`, Movebank
`phi/declined_sources.φ` — declined; Rubin/LSST `phi/blocked_sources.φ` — descoped).

- **behalten — in Benutzung:** `GOSAT_GW_MAIL`/`GOSAT_GW_PASS`
  (`.github/workflows/gosat-cdn.yml:29-30`; `gosat_tanso3_compiler.rs`).
- **behalten als Vorrat — Quelle registriert, kein Code-Leser:** `IGETS2_USER`/
  `IGETS2_PASS` (`phi/sources.φ` `igets`, anonymer Bezug).
- **entfernen — Quelle declined/descoped oder überholt:** `GFW_PASS`, `MOVEBANK_PASS`,
  `MOVEBANK_TOKEN`, `RUBIN_PASS`, sowie `BABAMUL_KAFKA_PASSWORD`/
  `BABAMUL_KAFKA_USERNAME`/`BABAMUL_PASSWORD` (überholt durch `BABAMUL_API_TOKEN`,
  `babamul_compiler.rs:65`). Vollzug (`gh secret delete` im Repo-Setting) ist der
  Konto-Akt, nicht diese Messung.
