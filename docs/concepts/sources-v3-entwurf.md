<!--
  title: sources.φ v3 — Entwurf v0.3
  class: concept
  date: 2026-10-21
  sha256: b865486b69ba0b50ac7e50b4f67a74ecd2e23dd450ec75deaad74da9cd60b87a
  status: draft
  see-also: rat/2026-10-21-rat-sources-schema.md docs/concepts/model-eignung.md docs/surveys/survey-2026-10-21-sources-schema-archaeologie.md
-->
# sources.φ v3 — Entwurf v0.3

**Status:** Entwurf, nicht gebaut. v0.1 war der Flash-Entwurf; v0.2
behob die sechs Blocker; **v0.3 setzt die sechs Operator-Entscheidungen
vom 2026-10-21 um.** Der Operator schließt.

## 1. Die sechs Entscheidungen (Operator, 2026-10-21)

| # | Entscheidung | v0.3 |
|---|---|---|
| 1 | Anker-Namen | `global`, `coverage`, `orbit` (+ `craft=`) — kein `spacecraft`, kein separates `map`/`swath` |
| 2 | Drei Zustände | **Neben-Datei** `state/source-zustand.tsv`, eine Autorität |
| 3 | Dämpfung/Transport | **`medium`-Zeile** mit Wert + Einheit; Feld nur `0|absent|pending` (0 mit `cite`) |
| 4 | Lizenz-Härte | 3 fehlende Terms-URLs **hard**; 267 Homepages **soft** mit Ratchet |
| 5 | Release-Historie | aktuelle `release`-Zeile im Block; Historie in `state/source-releases.tsv` |
| 6 | Urheber | `cite` **hard** bei CC-BY/BY; `origin` reicht bei PD-USGov |

Die alten Blocker bleiben darin gelöst: `tau_src = measured|derived|pending`,
`source_id` ohne `terms`, Frame-Migration, echte v3-Achsen, Release-Kette.

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
medium absorption=<value|absent|pending> <unit> advection=<value|absent|pending> <unit>
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
| `orbit <craft> frame=<f> …` | craft, frame | Raumfahrzeug/Orbit; `alt_m`/Epoche statt Oberflächenpunkt |
| `coverage <body> frame=<f> bbox=<…>   oder   footprint=<…>` | body, frame, bbox/footprint | Raster/Fläche/Swath; löst `map` und `swath` ab |
| `absent reason=<…>` | reason | belegte Nicht-Existenz; nicht für „unbekannt“ |

**Genau eine** Anker-Direktive je Block. `on earth 0 0 0` ist **kein**
Anker; es wird zu `absent`/`pending` oder zu `global`/`coverage`/`query`/
`track`/`orbit` migriert — je nach belegter Klasse. `craft=` ist ein
Attribut von `orbit`, kein eigener Anker.

## 4. Physik

- `force` ∈ `em|gravity|acoustic|seismic-body|seismic-surface|thermal|diffusion|advective|electric`.
- `kernel` = Kraft-Default, außer `global` → `aggregate`; Override nur
  mit `kernel_cite`.
- `unit` SI/UCUM; `C` verboten (`K`/`Cel`); `photon/cm2/s` nur mit
  `spectral_band`; `alt`/`alt_m` in Metern.
- `tau` nur aus dem Prozess (`measured`) oder einer belegten Regel
  (`derived`); sonst `pending`. **`ttl`/`ttl/10` sind keine τ-Quelle.**
- `absorption`/`advection` im Feld sind nur `0|absent|pending`; `0` nur
  mit `cite`. Numerische Werte stehen in `medium …` **mit Einheit**;
  ohne `medium` gibt es keinen Wert (kein nacktes `0.0`).
- `depth` ist **Adresse/`quantity`**, kein `field`; die Force-Tabelle
  führt es als `seismic-body`, die Drop-Regel als Koordinate — der
  Widerspruch wird zugunsten der Adresse aufgelöst.
- Drop-Regeln bleiben: Indizes, Zählungen, IDs, Zeitstempel, Booleans,
  Strings, Koordinaten als Skalar; Position ohne Messung.

## 5. Identität, Provenienz, Lizenz

- `source_id` = SHA-256 über `canonical_url`, `format` und die sortierten
  `field`-Keys. **Nicht** über `terms`, `compiler` oder Achsen.
- `terms`: SPDX-ID oder `LicenseRef-<token>` + **tiefe** Lizenz-URL.
  **3 fehlende URLs: hard jetzt.** 267 Homepage-URLs: soft mit Ratchet,
  hard nach der Migration.
- `origin`: Producer-URL/Akteur; hard nach Migration.
- `compiler`: unser Compiler (ID/Pfad); hard nach Migration; `none` nur
  ohne Transformation.
- `cite`: DOI/URL für Attribution. **Hard bei CC-BY und anderen
  BY-Lizenzen; `origin` reicht bei PD-USGov; unklare Lizenz → pending.**
- `release`: die **aktuelle** `release`-Zeile steht im Block; die
  Historie (`release_id`, `supersedes`, `archive_uri`, `sha256`,
  `fetched`) in `state/source-releases.tsv` (Key: `source_id` +
  `release_id`). `release_origin_consistency` prüft 404/200 und die Kette.

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

**Severity (entschieden):** hard bleibt `sha_bad`, `dup_field`,
`anchor_body`, `anchor_arity`, `source_id`; **`terms` ohne URL hard
jetzt**; `terms`-Homepage soft mit Ratchet; **`cite` bei BY hard**;
`medium` nur mit Einheit. Soft bis Migration: `origin`, `compiler`,
`tau_src`, `absorption`/`advection`, `alt_m`; hard danach. `elev_zero`
wird nach der Anker-Migration hard; `anchor_missing` sichtbar und hard
nach der Positions-Migration. Release-Historie liegt in
`state/source-releases.tsv`, nicht im Block.

## 8. Entschieden (Operator, 2026-10-21)

1. Anker: `global`, `coverage`, `orbit` (+ `craft=`); kein `spacecraft`,
   kein separates `map`/`swath`.
2. Drei Zustände: Neben-Datei `state/source-zustand.tsv`.
3. Dämpfung/Transport: `medium`-Zeile mit Wert + Einheit; Feld nur
   `0|absent|pending` (0 mit `cite`).
4. Lizenz-Härte: 3 fehlende Terms-URLs hard; 267 Homepages soft mit
   Ratchet.
5. Release-Historie: aktuelle `release`-Zeile im Block; Historie in
   `state/source-releases.tsv`.
6. Urheber: `cite` hard bei CC-BY/BY; `origin` reicht bei PD-USGov.

Offen ist nur noch die Umsetzung: Parser v3 → Spec v3 → Validator → CI.

## 9. Belege

- `rat/2026-10-21-rat-sources-schema.md`: v0.1, drei Kritiken, Flash-Messung.
- `docs/concepts/model-eignung.md`: Autoren-/Kritiker-Rollen und Efforts.
- `docs/surveys/survey-2026-10-21-sources-schema-archaeologie.md`,
  `...-wissenschaft.md`: Baum- und Wissenschaftskontext.
