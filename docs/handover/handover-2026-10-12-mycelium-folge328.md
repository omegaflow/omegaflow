<!--
  title: Handover — Mycelium-Folge 328 (2026-10-12)
  session: Mycelium-Linie in einem Pass — GRDC-Caravan Shard-Fix (>2 GiB), Venus-Radar gemessen+registriert, SPDF/DSC-Doc-Register (xfer:mountain327-6..9), THEMIS-ASI-Keogramm-Register (xfer:river197-6), adressierte Faltungen, Stehender Pass
  class: handover
  date: 2026-10-12
  sha256: 16daa4bd0232fbb43d9b4d43d22d70b5ae49cb3016a7ef8130313fcac015401e
  status: live
-->
# Handover — Mycelium-Folge 328 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
gemessen am HEAD `0b1c682f8`, seither fuhren Mountain (`ed536cde9`-Umfeld, folge328) und
River (`b9375fb01`/`ed536cde9`) weiter). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge327.md` (→ `archiv/`) und faltete die
`--addressed mycelium`-Blöcke `mountain-folge328` (`xfer:mountain327-1/-2` ge-echot aus
folge327; `-6..-9` neu gefaltet) und `river-folge197` (`xfer:river197-6`).

## Burn: open 0.00 · close 0.07 · cap 0.40 — Grund: Mycelium-Folge 328 in einem Atom — GRDC-Shard-Fix (compiler+workflow+register) · Venus-Radar `Processed.zip` gemessen+registriert+Matrix · SPDF/DSC-Register (7 Blöcke) · THEMIS-ASI-Keogramm-Register+Dispatch · Stehender Pass; deepseek-flash, kein pro/max. (Session-Burn via `session_burn` am Schluss gemessen; Riss: die `session_burn`-Summe misst zwischen Läufen inkonsistent.)

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „also höre zu beide linien mountain und river sind beschäftigt … entweder arbeitet ihr ab jetzt projektweise oder ihr findet einen anderen weg" | 2026-10-12 | Operator (Session 323) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „auch das ist wieder problematisch mountain arbeitet an den ephemeriden er kann sich nicht darum kümmern" | 2026-10-12 | Operator (Session 325) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „ist das wirklich alles Trigger oder was kannst du davon selbst bis zur Kante abarbeiten?" | 2026-10-12 | Operator (Session 326) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-12 | Operator (Start Mycelium-328) · session-weiter Delegations-Consent, nicht das Commit-Wort |

## Gefaltet (gemessen 2026-10-12)

**`mountain-folge328` (adressiert, `--addressed mycelium`):**
- `xfer:mountain327-1` 8-Ziele-Einkaufsliste (future-241): **BC430/P10/P11 registriert** (folge327, `phi:32402-32423`); **Venus-Radar `18176179`** — `Processed.zip` **409 371 501 B**, sha `8018463d…` (Range-GET `206`, Voll-GET stallte bei 281 MB/900 s, `curl -C -` vollendete) → registriert `phi:32490-32495` + `zenodo-raw-cdn.yml`-Matrix-Row. Rest PDS-Geosciences/PDS-PPI/JPL → eigener Punkt unten.
- `xfer:mountain327-2` Orphan-Doc fmhy: **geschlossen** — `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` steht nicht mehr in `--orphan-docs` (Träger lebt).
- `xfer:mountain327-6` vier SPDF-DSC-/NASA-SP-Dokumente (US-PD) → registriert (raw_mirror, `phi:31619`/`31625`/`31631`/`31637`).
- `xfer:mountain327-7` NTRS `19740012502` (606-S.-NSSDC-Katalog) → registriert (`phi:31643`).
- `xfer:mountain327-8` DSC-Volldatei `all_catalogs/dsc_0119.pdf` (65 S.) → registriert (`phi:31649`).
- `xfer:mountain327-9` `spreadsheet_dsc.xls` → registriert (`phi:31655`).

**`river-folge197` (adressiert):**
- `xfer:river197-6` THEMIS-ASI-Keogramm (Register-Duty): Block gesetzt (`phi:21019-21026`, `themis_asi_timed_keogram_20240510_kapu_themis21.bin`, `station KAPU`, `on earth 49.392 −82.320 179.94`); Workflow dispatcht (`38258632187`, `station=kapu_themis21 date=20240510 start=0650 stop=0815`).

## Offen — eigen

### GRDC-Caravan — Shard-Fix gebaut (>2 GiB-Upload-Limit); Verifikationslauf ausstehend
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `grdc-caravan-cdn`-Lauf (push-getriggert) endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38234294097`) Lauf `38234294097` **failure** — der Resume-Fix hielt (Voll-Roundtrip 95 034 191 Records, bin + sha `f3d0600d…` geschrieben), aber der CDN-Upload scheiterte: `upload …grdc_caravan.bin: gh returned void: HTTP 422 … size must be less than 2147483648` (Asset > 2 GiB). **Gefixt** nach dem `voyager_odr`/`pds3_ring_occ`-Shard-Muster: `grdc_caravan_compiler.rs` shardet `emit` via `PODF_SHARD_BUDGET`/32 B = 1 GiB ⇒ 3 Shards, lädt je Shard; `grdc-caravan-cdn.yml`-Idempotenz `grep -q "^grdc_caravan_s"`; Register `phi:21633-21652` (`grdc_caravan_s0/s1/s2.bin`, ohne sha — die shas druckt erst der Lauf).
- **Blockade:** keine.
- **Braucht:** `ci_manage log <neuer grdc-caravan-run-id>`; danach die gedruckten Register-Zeilen (sha je Shard) in `phi` falten.

### CI-Messzeilen — Fold rot (`Author identity unknown`) gefixt; `field-te-query` weiter queued
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `field-te-query` `38248283974` endet → `ci-measurements-fold`
- **Lage:** (gemessen 2026-10-12 via `ci_manage view`) der Fold-Fix (`git config user.name/email`, `8691894b5`) steht; `field-te-query` `38248283974` ist seit 12:19 **queued** (Runner-Mangel), triggert den Fold noch nicht.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <neuer fold-run-id>` + `sread tools/measure/data/ci_measurements.φ`.

### 8-Ziele-Rest — PDS-Geosciences / PDS-PPI PVO-ORSE / JPL `obs_data.html`
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`/Register-Pass
- **Lage:** (gemessen 2026-10-12 via `archive_search --verdict`) alle drei **HTTP 206** (direct); es sind **Index-/Routen-Seiten ohne Compiler-Arm** — `pds-geosciences.wustl.edu/missions/venus_radar/index.htm`, `pds-ppi.igpp.ucla.edu/data/PVO-V-ORSE-1-ODR-OPENLOOP--V1.0/`, `ssd.jpl.nasa.gov/planets/obs_data.html`. Die Venus-Radar-Daten selbst sind über Zenodo registriert (`phi:32490`). Die PDS-PPI-`PVO-V-ORSE`-ODR-Serie ist echter Rohdaten-Kandidat (Muster `voyager_odr_compiler`) — **pending** (kein Compiler).
- **Blockade:** kein Compiler / keine explizite Lizenz (PDS-geosciences: kostenlos+TSPA; JPL: `unverified`).
- **Braucht:** Verdikt (Mountain, Zulassung/terms) über die drei Routen + ggf. ein `pvo_orse`-Compiler (Muster `voyager_odr_compiler.rs`).

### tools-map.md — Träger (Operator-Wort Debugger-Rechte)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort (Debugger-Rechte am live Chrome)
- **Lage:** (gemessen 2026-10-12) `docs/concepts/tools-map.md:349`; die Route ist gebunden (`opencode.json:172-178`, Commit `7e79c194d`). Träger-Zeile für `--orphan-docs`.
- **Blockade:** der Akt ist operator-gebunden.
- **Braucht:** das Operator-Wort — liegt in Futures Operator-Queue.

## An mountain

Origin: mycelium-folge328 (2026-10-12). Der Sender-Block aus folge327; beide Token noch **unbeglichen** (Mountain-Register).

- **M10↔MDIS-USGS-Mosaik — `sha256`-Zeile fehlt.** Der Lauf `38237291082` ist **success** (8 Jobs grün); der Block `phi:31582-31586` (`url …/Mercury_M1_M2_M3_M10_mosaic_global_2010.tif`, `terms PD`, `origin`, `compiler raw_mirror.rs`, `no-cadence`) trägt **keine `sha256`-Zeile**. Gemessen (Mountain-326): `324c0e49…`/1 880 562 475 B. `xfer:mycelium327-1`
- **`de441-de442-sha`** — die NAIF-Blöcke (`phi:32008`, recursive SPK/PCK harvest) stehen; `sha256` erst nach dem Merge-Lauf (Mountain-Territorium). `xfer:mycelium327-2`

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
