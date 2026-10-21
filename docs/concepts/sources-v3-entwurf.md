<!--
  title: sources.φ v3 — Entwurf v0.2
  class: concept
  date: 2026-10-21
  sha256: b33cff8ab51796e81d10df2ce55960b37c53d6060a6e4c8fe80bcaba1fefc79e
  status: draft
  see-also: rat/2026-10-21-rat-sources-schema.md docs/concepts/model-eignung.md docs/surveys/survey-2026-10-21-sources-schema-archaeologie.md
-->
# sources.φ v3 — Entwurf v0.2

**Status:** Entwurf, nicht entschieden. v0.1 war der Flash-Entwurf; v0.2
behebt die sechs Blocker aus der Kritik (Sonnet/Kimi/DeepSeek) und dem
Baumtest. Der Operator schließt.

## 1. Was v0.2 ändert

| v0.1-Blocker | v0.2 |
|---|---|
| `tau_src: ttl|cadence|explicit|none` | **`tau_src = measured|derived|pending`**; `ttl`/`cadence` sind keine τ-Quelle; `derived` braucht `derive`+`cite` |
| Anker `global/query/track` fehlen | **fünf neue Anker-Direktiven** (Tabelle unten); `absent` bleibt nur für belegte Nicht-Existenz |
| `source_id = sha256(url|terms.id)` | **`source_id = sha256(canonical_url|format|sortierte field-keys)`**; Lizenz/Compiler ändern die Identität nicht |
| Frame-Härtung vor Migration | **Frame fehlt → `pending`/`absent`** in der Migration; hard erst nach Parser-Fix |
| `axes_v3` ist eine neue Liste | **echte v3-Achsen** (Messzustand, Lebenszyklus, Zugang) im Sidecar |
| Release-Kette fehlt | **`release`-Direktive** mit `release_id`/`supersedes`/`archive`/`sha256`/`fetched`; CHUR `alt` pending |

## 2. Kanonischer Block (v3)

```text
url <endpoint>
terms <SPDX|LicenseRef> <deep-url>
origin <producer-url>
cite <doi|url>
compiler <id|none>
release <release-id> [supersedes <id>] [archive <uri>] [sha256 <hex>] [fetched <ISO>]

<genau eine Anker-Direktive>

ttl <s> | no-cadence

[map|rows|cmap …]
[lat|lon|alt|epoch … | ra|dec|plx|dist|catalog_epoch …]

field <key> <name> <kernel> <force> <unit> <tau> <tau_src> <absorption> <advection>
```

- `field` ist **10-Token**: `tau` (Zahl oder `pending`) + `tau_src`.
  Die 9-Token-Form bleibt nur als Migrations-Eingang; der Parser setzt
  `tau_src = pending` und markiert `legacy`.
- Kein `Frame::Manifest`-Default. Fehlt der Frame, ist der Block
  `pending`/`absent`, nicht still baryzentrisch.
- `compiler` wird gelesen; `compiler none` nur ohne Transformation
  (keine Einheitenumrechnung, kein `average`, kein `where`).
- `release` ist first-class; `source_id` bleibt stabil, auch wenn ein
  Release 404 ist und `origin` 200 liefert.

## 3. Anker-Arten

| Direktive | Pflicht | Bedeutung |
|---|---|---|
| `at <body> frame=<f>` | body, frame | Körper-/Emitter-Frame; **kein Messort**; nur Ephemeride/Modellgröße |
| `on <body> frame=<f> lat=<deg> lon=<deg> alt_m=<m|absent> datum=<id> epoch=<ISO-TDB>` | body, frame, lat, lon, datum, epoch | fester Oberflächenpunkt; `alt_m` absent/pending, nie synthetisch 0 |
| `global <body> frame=<f> method=<id>` | body, frame, method | Körpermittel ohne Ort; `kernel=aggregate` |
| `query <body> frame=<f> lat_key=… lon_key=… alt_key=… alt_unit=… time_key=…` | body, frame, vier Keys | Position pro Aufruf/Zeile |
| `track <body> frame=<f> lat_key=… lon_key=… alt_key=… time_key=…` | body, frame, Keys | bewegte Plattform (Drifter, Argo, Flugzeug) |
| `sky frame=icrs ra_key=… dec_key=… plx_key=<…> dist_key=<…> epoch=…` | frame, ra, dec, epoch | ICRS-Katalog pro Zeile |
| `orbit <body|craft> frame=<f> …` | body/craft, frame | Raumfahrzeug/Orbit; `alt_m`/Epoche statt Oberflächenpunkt |
| `map <body> frame=<f> bbox=<…> res=<…> epoch=…` | body, frame, bbox | Raster/Fläche |
| `swath <craft> footprint_key=…` | craft, footprint | Satelliten-Swath |
| `absent reason=<…>` | reason | belegte Nicht-Existenz; nicht für „unbekannt“ |

**Genau eine** Anker-Direktive je Block. `on earth 0 0 0` ist **kein**
Anker; es wird zu `absent`/`pending` oder zu `global`/`query`/`track`
migriert — je nach belegter Klasse.

## 4. Physik

- `force` ∈ `em|gravity|acoustic|seismic-body|seismic-surface|thermal|diffusion|advective|electric`.
- `kernel` = Kraft-Default, außer `global` → `aggregate`; Override nur
  mit `kernel_cite`.
- `unit` SI/UCUM; `C` verboten (`K`/`Cel`); `photon/cm2/s` nur mit
  `spectral_band`; `alt`/`alt_m` in Metern.
- `tau` nur aus dem Prozess (`measured`) oder einer belegten Regel
  (`derived`); sonst `pending`. **`ttl`/`ttl/10` sind keine τ-Quelle.**
- `absorption`/`advection` ∈ `0|absent|pending`; `0` nur mit `cite`;
  numerische Werte brauchen eine `medium`-Direktive (offen, §8).
- `depth` ist **Adresse/`quantity`**, kein `field`; die Force-Tabelle
  führt es als `seismic-body`, die Drop-Regel als Koordinate — der
  Widerspruch wird zugunsten der Adresse aufgelöst.
- Drop-Regeln bleiben: Indizes, Zählungen, IDs, Zeitstempel, Booleans,
  Strings, Koordinaten als Skalar; Position ohne Messung.

## 5. Identität, Provenienz, Lizenz

- `source_id` = SHA-256 über `canonical_url`, `format` und die sortierten
  `field`-Keys. **Nicht** über `terms`, `compiler` oder Achsen.
- `terms`: SPDX-ID oder `LicenseRef-<token>` + **tiefe** Lizenz-URL;
  Homepage-URLs sind soft, fehlende URL pending; hard nach Migration.
- `origin`: Producer-URL/Akteur; hard nach Migration.
- `compiler`: unser Compiler (ID/Pfad); hard nach Migration; `none` nur
  ohne Transformation.
- `cite`: DOI/URL für Attribution (CC-BY); hard, wo die Lizenz BY verlangt.
- `release`: `release_id`, `supersedes`, `archive_uri`, `sha256`,
  `fetched`; `release_origin_consistency` prüft 404/200 und die Kette.

## 6. v3-Achsen — Sidecar, eine Autorität

`state/source-zustand.tsv`:

```text
source_id	zustand	lebenszyklus	zugang	owner	trigger
```

- `zustand`: `pending|gemessen` (mit `obergrenze|null-echt|absent|riss`).
- `lebenszyklus`: `offen|wartend|declined`.
- `zugang`: `frei|LOCK(privat)|LOCK(rechtlich)`.
- Der Block trägt **nur** `source_id`; die Achsen wechseln, die Quelle
  bleibt stabil. Der Validator joint über `source_id`; keine Doppelzelle.
- `wartend` bleibt in `state/wartend.φ`; die Lizenzmap bleibt die
  Begründung für `terms`/`LOCK`.

## 7. Migration und Severity

**Reihenfolge:** Parser → Spec → Validator → CI.

1. **Parser v3 lossless:** v3-AST + `legacy`-Marker; kein Default-Frame;
   `on` ohne `alt` → `absent`; `compiler` lesen; 9-Token-`field` →
   `tau_src=pending`.
2. **Spec v3:** Pflichtfelder/Anker/Physik/Achsen fixieren.
3. **Validator:** hard/soft/pending; Baseline 291 soft; Ratchet nach unten.
4. **CI:** `source_audit check`, `licenses`, `observe`; alle Checks.

**Baseline (gemessen):** 290 `on earth 0 0 0`; 760 ohne Position;
1.683 τ=ttl/ttl/10; 4.505 `0.0 0.0`; 267 Homepage-terms; 692 ohne
`origin`; 1.328 ohne `compiler`; 291 soft (290 `elev_zero` + 1
`anchor_missing`).

**Severity:** hard bleibt `sha_bad`, `dup_field`, `anchor_body`,
`anchor_arity`, `source_id`; soft bis Migration `terms`-URL, `origin`,
`compiler`, `tau_src`, `absorption`/`advection`, `alt_m`; hard danach.
`elev_zero` wird nach der Anker-Migration hard; `anchor_missing` sichtbar
und hard nach der Positions-Migration.

## 8. Offene Entscheidungen (Operator/Rat)

1. Anker-Namen: `global` vs. `coverage`; `orbit` vs. `spacecraft`.
2. Achsen-Sidecar vs. Block — v0.2 schlägt Sidecar vor.
3. Numerische `absorption`/`advection`: eigene `medium`-Direktive?
4. `terms`-Härte: soft bis alle 267 Homepages migriert sind?
5. Release-Kette: reicht `release` im Block, oder eigener `release`-Block?
6. `cite` verpflichtend für CC-BY, oder nur `terms`+`origin`?

## 9. Belege

- `rat/2026-10-21-rat-sources-schema.md`: v0.1, drei Kritiken, Flash-Messung.
- `docs/concepts/model-eignung.md`: Autoren-/Kritiker-Rollen und Efforts.
- `docs/surveys/survey-2026-10-21-sources-schema-archaeologie.md`,
  `...-wissenschaft.md`: Baum- und Wissenschaftskontext.
