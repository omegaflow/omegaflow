<!--
  title: Handover — Ernte-Folge 126 (Stand 2026-09-21)
  session: Ernte-Folge 126
  class: handover
  date: 2026-09-21
  sha256: f07d1745881d8144b55d4af1fed6ce212c7e25425cee31d6a83bccce5c912262
  status: live
-->
# Handover — Ernte-Folge 126 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt. Das Handover wird **vor allem anderen gegen den Baum gehalten**
— das Register ist die Frage, der Baum die Messung.

## Stehender Pass (gemessen 2026-09-21, Folge 126)

- **HEAD** `a70b1ee2` beim Start (geteilter Baum, fremde Commits laufen
  gleichzeitig); eigener Commit folgt. Arbeitsbaum: eigener Pfad-Satz (unten);
  fremde uncommittete Arbeit (AGENTS.md, docs/specs, src/gate, external-state.md,
  post.md, fremde Handover-Moves) nicht angefasst.
- **Postfach** — 1 neuer Mail-Ledger-Eingang seit `1789930255`: `1789970277`
  (bietet 1× Ox64 SBC, Shipping-Info/Telefon nötig). Nicht ernte-eigen.
- **CI** — `ci-check` `35567290094` pending / `35566258372` in_progress;
  `tools-build` `35567260047` success; `ps1-cdn` `35563001793` in_progress;
  `demeter-cdn` `35567568429` (dieser Session-Dispatch). Der repo-weite fmt-Rot
  (Lauf `35537130867` @`5894b345`) ist geheilt: fremd `a70b1ee2`
  (bia/free_model_*/hyperscanning/register_lookup) + eigener ps1-Fix.
- **Zustand** — `docs/zustand/external-state.md` ist fällig (GitHub PII,
  `dr3_stars.bin`, `voyager_odr` Shards, Release-Binär), aber gerade
  fremd-uncommittet → nicht angefasst; nächster Pass misst gegen den dann
  stehenden HEAD.

## CDN-Rotation rpw_efield_lira (härtester undatierter Punkt)

- **`rpw_efield_lira.bin`** (199152 Records, Roundtrip parst) ist unmanifestiert:
  Upload HTTP 422, Release `ssd.jpl.nasa.gov` (id 367063539) am 1000-Asset-Limit
  (`Link rel=last page=1000`, gemessen 2026-09-21). Schritt: family-tag-Rotation —
  neuer Upload über `upload_release(family_tag, path)` (`src/archivar/cdn.rs:44`),
  family-tag const in `bia_efield_compiler` (Präzedenz `--release-tag` in
  tap_compiler), Download-Tag `.github/workflows/rpw-cdn.yml:36`, URL-Tag
  `phi/sources.φ:1139`; Verifikation `gh workflow run rpw-cdn.yml`; der Tag, der
  geschrieben wird, ist der Tag, der gelesen wird.

## DEMETER (wartend auf Re-Dispatch)

- **Route-Gegenmessung 2026-09-21** (`research-max`): `regards.cnes.fr` DNS+Root
  HTTP 200; `rs-order` GET 403 = Auth-Gate (Jetty-Backend erreicht), **kein**
  F5-ASM-Text; Login 200 `access_token present`; `rs-catalog search` HTTP 500.
  Die Lauf-Blocker (F5 ASM + DNS) reproduzieren nicht → Eintrag von
  `blocked ip-blocked` auf `pending` umgetragen (`phi/blocked_sources.φ`).
  Re-Dispatch `demeter-cdn.yml` `35567568429`. Bei success: 77 `url`+`sha256`-Zeilen
  registrieren. Der `rs-catalog`-500 ist ein eigener Register-Befund (frischer
  URN-Scan), kein Re-Dispatch-Blocker solange der Workflow-Cache (`demeter_urns.txt`)
  trägt.

## Ernte-Dropped-Audit (Rest)

- **EPA AQS map-key-Gap widerlegt** (`daily_88101`, `sources.φ`) — Route 200
  (8 647 326 B); „Arithmetic Mean" = Spalte 16 (deckt `field 16`); kein
  `aqs_parser` (csv_zip/rows-Pfad), header-key gebaut+getestet (`tests.rs:9748`).
  Verdikt „gebaut" im `note`. Beobachtung (offen): `Sample Duration`=„1 HOUR" —
  der Feldname `pm25_daily_ug_m3` ist ggf. ein Misnomer.
- **GHRC GLM/TRMM widerlegt** (`blocked_sources.φ:33`) — Compiler gebaut
  (`glm_l1b_compiler.rs`, `trmm_lis_compiler.rs`), CDN sha256-verifiziert
  (`glm_l1b.bin` 1 050 368 B, `trmm_lis.bin` 30 548 B). GHRC bleibt 401 anonym.
  Offen: öffentliche NOAA-AWS-Route (`noaa-goes16/18` S3 200; `ghrc.s3` 403) als
  token-freie Alternative in `sources.φ` registrieren.
- **Quaoar sha256** (`zenodo.21185812`, 572 467 032 B) — Zenodo 504, sha256
  `pending` (`wartend` auf Zenodo-Rückkehr).

## Aus dem Postfach / Register gefaltet (offen)

- **EPA RadNet ERM_RESULT** — Route 200 (`data.epa.gov/efservice`), `result_in_si`
  = Bq/L (em); **Position fehlt** (ERM_LOCATION nur city/state, keine Koordinaten).
  Als `parser-def` in `blocked_sources.φ`. Schritt: lat/lon per externer
  RadNet-Stationsliste joinen, dann Port nach `sources.φ` + CDN-Manifestation.
- **Rosetta Idempotenz-Gate** (`wartend`) — letzte Nennung
  `handover-2026-09-17-ernte-folge75.md`, kein zentraler Trigger in
  `external-state.md`. Schritt: Auslöser benennen.

## Katalog-Inventar stale (Prozess-Gap)

- Der Digest zählt die 192 disponierten Kandidaten weiter als `candidate → ernte`
  (gitignored `phi/pipeline/catalog/*.φ` tragen die Zeilen unverändert). Schritt:
  Digest gegen die Register deduplizieren oder die Katalogzeile bei Disposition
  mitführen.

## PS1

- **PS1-Ernte-Rate** — `wartend` auf `ps1-cdn`; Verdikt offen. Schritt:
  `ci_manage list` nach `ps1-cdn`, einmalig `ci_manage log <id>`.
- **PS1-Fraktional Order-10-Final** — Größe aus dem Combine-Log → `footprints.φ`.

## Wartend (kein Auswahlpunkt)

- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** (`ledger.φ`) — Trigger sync-QUERY/tables 500→200.
- **SSDC Limadou** — operator-gebunden (PI-Freigabe).
- **Lasair-LSST API** — Backend 502; Trigger Erholung.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno/Cassini) `blocked_sources.φ`.
- **BepiColombo bc_mpo_more** — Freigabe ~April.
- **NRS02-10,12,13 SHAPE** `nrs_stations.φ:17` — Trigger neuer Stations-Prefix.
- **Babamul / IA2 TAP** `blocked_sources.φ` — kein gebauter Konsument → `pending`.
- **Split-Routing-Verifikation** — operator-gebunden (`An entscheid`).

## Termin

- **EMODnet HFRADAR NADR** — nächste Re-Messung **2026-10-19**.

## Ausgelagerte Fremd-Owner-Punkte (nicht im eigenen Handover)

- post_body-Migration → bau (`post.md`).
- units.rs `bq/l` → bau (`post.md`).
- Split-Routing → entscheid (`post.md`).
- force-gate-B-Flush-Lücke (`port_mode`) → bau (`post.md`).
- PINE64 Ox64-Angebot / Framework-Ablehnung → entscheid (operator-gebunden;
  `post.md` ist gerade fremd-uncommittet, daher hier getragen statt als Post).
- fmt-Rot `bia_efield_compiler` + measure/register → geheilt durch fremd `a70b1ee2`.

## Benchmark

- **Ernte-Folge 126** — `research-max` (DEMETER-Route-Gegenmessung: DNS/Auth-Gate/
  Token/Katalog-500), `grind-pro` (EPA AQS-Verdikt, GHRC-Widerspruch, RadNet
  parser-def), eigener fmt-Fix ps1. Kein flash/pro-Vergleich; Burn nicht gemessen.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `tools/harvest/src/bin/ps1_coverage_compiler.rs` (rustfmt
  Z.605,651), `phi/sources.φ` (AQS-`note`), `phi/blocked_sources.φ` (DEMETER
  `pending` + GHRC-`note` + RadNet `parser-def`), neues Handover
  `handover-2026-09-21-ernte-folge126.md`, Move
  `handover-2026-09-20-ernte-folge125.md` → `archiv/`.
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
