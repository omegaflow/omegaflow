<!--
  title: Handover — Mycelium-Folge 234 (2026-10-05)
  session: Mycelium-Linie — 257 single-job CDN compilers to the self-hosted runner, dropped-baseline bump, emm-sdc green, CDN re-dispatch wave
  class: handover
  date: 2026-10-05
  sha256: 861ff07878bc8ac070bf693076f1aff7222a4ce400d620ddbefb4d8164fc649d
  status: live
-->
# Handover — Mycelium-Folge 234 (2026-10-05)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-05-mycelium-folge233.md` (→ `archiv/`).

## Burn: open 0.0012 · close 0.0572 — session_burn (Mycelium-Session „in einem Pass starten")

## Operator-Wort-Register

- Wort | 2026-10-05 | „du bist mycellium" | Quelle: Operator (Session 2026-10-05) — der Commit-Prefix `mountain` war falsch.
- Wort | 2026-10-05 | „ja die zwei" (FMI + TUH EEG) | Quelle: Operator (Session 2026-10-05) — die zwei neuen Mails bestätigt; die TUH-EEG-Public-Key-Antwort ist der Operator-Akt.
- Wort | 2026-10-05 | „nein johannes.tyroller@proton.me" | Quelle: Operator (Session 2026-10-05) — die TUH-EEG-Public-Key-Antwort trägt die Adresse aus dem signierten Formular (`johannes.tyroller@proton.me`), nicht `code@omegaflow.space`.
- Wort | 2026-10-05 | „die kostenlose variante" | Quelle: Sensory-233 — der self-hosted Runner `t420` wird für die CDN-Compiler genutzt.
- Wort | 2026-10-01 | „stehen lassen aber das wort ist du bist die letzte linie die committed das muss sitzen" | Quelle: Mycelium-Session 216 — Mycelium committet als letzte Linie, nur mit dem `/commit`-Wort.
- Wort | 2026-10-01 | „bitte nicht nur messen und verschleppen sondern bearbeiten messen und bearbeiten ist die prämisse mein dauerhaftes wort" | Quelle: Mycelium-Session 216 — **dauerhaftes Wort**.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213.
- Wort | 2026-10-02 | „… ihr macht umfangreiche läufe und dann kastriert ihr sie … so funktioniert forschung nicht" | Quelle: future-folge169 — kein Top-N.
- Wort | 2026-09-29 | **SuperDARN nicht messen** — „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." | Quelle: `state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25` — kein Register-/Mess-Akt.

## Offen — eigen

### CDN-Re-Dispatch-Welle (t420) — Läufe unread
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die vier Läufe enden → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-05, dispatch) `openneuro-cdn` (ds004100) `37329643821` in_progress; `clpds-cdn` `37329649099`, `fmi-gic-cdn` `37329656081`, `acs-nir-cdn` `37329660510` queued. Die Fixes (EDF-Annotation-Filter, clpds-annex-Idempotenz, fmi 1-min-Step) stehen in `498b3ff2a`/`ea6558e2d`.
- **Blockade:** keine (Runner-Queue).
- **Braucht:** Läufe lesen, `ci_manage log <id>`; bei Grün `format`/`sha256`/`url` der neuen Assets in `sources.φ` (Mountain-Block vorausgesetzt).

### emm-sdc — Asset grün, Register wartet auf Mountain-Block
- **Status:** wartend (Mountain) | **Bindung:** Mountain (Disposition) + Mycelium (Manifestation)
- **Trigger:** Mountains `emm_exi_l2a`-Block in `sources.φ`
- **Lage:** (gemessen 2026-10-05, `gh release view --repo omegaflow/sources`) `emm-sdc-cdn 37318801653` **success**; `emm_exi_l2a.tar` `sha256 6f379edfaf2910a83873ce82ff01e5646e3c5e1178447e6249b6bc6a573edaf4`, 1 439 406 B, `url https://github.com/omegaflow/sources/releases/download/sdc.emiratesmarsmission.ae/emm_exi_l2a.tar`. Der Proxy-Fix (`NO_PROXY`) trägt.
- **Blockade:** `phi/blocked_sources.φ:74-76` noch `pending` (Bildquelle ohne `field`); kein `sources.φ`-Block.
- **Braucht:** nach Mountains Register-Block `url`/`sha256` in `sources.φ` nachziehen; Frame-Bundle-Arm → River.

### Ausstehende Wellen-Assets ohne Register-Block → Mountain
- **Status:** wartend (Mountain) | **Bindung:** Mountain
- **Trigger:** Mountains Disposition — steht aus; gemessen `phi/blocked_sources.φ:74-76` (`pending`)
- **Lage:** (gemessen 2026-10-05) die neue Welle produziert `clpds_annex.jsonl`, die supermag-1999-Serie (`supermag_*_1999-01-01_31d.bin`), openneuro ds004100-iEEG-Assets; `sha256`/`url` gemessen, Zulassung/Format/ttl/field fehlen.
- **Blockade:** Mountain-Disposition.
- **Braucht:** `## An mountain` (unten).

### `ned-byparams` — 0/180 Bänder, kein Final-Asset
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** `ned-byparams-cdn` re-dispatch → Log
- **Lage:** (gemessen 2026-10-05) `37250626173` success, aber `bands present: 0/180`; per-Band `result fetch void` (`tools/harvest/src/bin/ned_byparams_compiler.rs:780`).
- **Blockade:** Band-Ergebnis-URL void.
- **Braucht:** einen Band-Lauf mit `--band` und voller Log-Ausgabe; `result_url`/`fetch_body` prüfen.

### iEEG-Ernte — Backend 503 (Server-Kapazität)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** iEEG-Backend erholt sich → `gh workflow run ieeg-cdn.yml -f dataset=09_14_limbic_seizure_374`
- **Lage:** (gemessen 2026-10-04) `37235356150` failure, `ieeg: getId http 503` (Server).
- **Blockade:** iEEG-Backend überlastet.
- **Braucht:** re-dispatch bei Kapazität; bei Grün `format ieeg_edf` + `sha256`; 4D-Anker = River/Mountain.

### Register-Träger `ledger.φ:2`/`:6` — Port-Artefakte
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Port-Runner im Baum
- **Lage:** (gemessen 2026-10-04) `ledger.φ:2` = 825 Blöcke, `:6` = 63; `phi/pipeline/stage/*` leer; der Ausführer war ein nie committeter Working-Tree-Bin; nur der Motor `src/archivar/port.rs`.
- **Blockade:** Port-Runner verloren.
- **Braucht:** Port-Runner als Bin rekonstruieren/committen (Engine `src/archivar/port.rs`; Konverter-Spec = Mountain).

### `phi/blocked_sources.φ` — Mycelium-Klasse (Träger; Stand gemessen 2026-10-05)
- **Status:** je eigen | **Bindung:** eigen
- **Trigger:** Arm-Bau/Manifestation je Eintrag (externer Host-Rückkehr oder Mountain-Disposition)
- **Lage:** (gemessen 2026-10-05)
  - `:166` BGI AGrav — station/nearto 200, `gravity m/s2`; Verdikt inverse-square gravity; **Arm + Manifestation offen**.
  - `:170` C9/CEEIN Infraschall — station 200, dataselect `C9/BDF` MSEED; Verdikt gaussian-inverse-square acoustic Pa; **Arm + Manifestation offen**.
  - `:118` JAXA G-Portal — `sha256` steht; Record-Download (`add_download.json`/SFTP) offen.
  - `:146` PDS-PPI Kuration offen; `:138`/`:142` externe Hosts down (wartend).
  - `:189` EUMETSAT MTG-LI (`https://data.eumetsat.int`) — Landung HTTP 206 (2 745 B), `EUMETSAT_KEY`/`SECRET` in `.secrets.local`; API-Route/kein eumetsat-Arm offen.
  - `:193` GOES-18 ABI — Bucket-Root 200 (486 709 B), `ABI-L1b-RadC/` 404; kein goes18-Compiler (goes_abi nur goes16/19); Riss: goes16-Block zeigt auf goes19.
- **Blockade:** je Eintrag (Arm-Bau / Mountain-Disposition / externe Hosts).
- **Braucht:** `:166`/`:170` Arm bauen + manifestieren; `:118` Download-Route; `:146` Kuration.

### FMI-IMAGE / IERS EOP C04 / gbco-Serie — Manifestation (mountain-236)
- **Status:** wartend | **Bindung:** Mountain
- **Trigger:** die drei Compiler/Register-Blöcke committet — gemessen `sources.φ:27042-27053`, `?? tools/harvest/src/bin/gbco_series_compiler.rs`
- **Lage:** (gemessen 2026-10-05) Arme + Register gebaut, **uncommittet** (fremde Hand, u. a. `?? tools/harvest/src/bin/gbco_series_compiler.rs`).
- **Blockade:** nicht im getrackten Baum.
- **Braucht:** nach Mountains Commit je Workflow (Muster `fmi-gic-cdn.yml`) + Dispatch.

### Orphan-Docs — Survey-Träger
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Klassen-Zensus + Step 5 als Register-Bindung
- **Lage:** (gemessen 2026-10-05, general) `register_lookup --orphan-docs` = 2 (`survey-2026-09-03-daten-holdings-inventur.md`, `survey-2026-09-03-orphan-verdicts.md`); der Baum trägt **386** `*.yml`, nur `planetary-odf-cdn.yml:37` liest seine Release-Menge aus `sources.φ`.
- **Blockade:** keine.
- **Braucht:** Klassen-Zensus (386 Workflows) + Step-5-Bindung als nächster Schritt.

### Membran-Assets — `dr3_stars.bin` / `ephemeris_de440_*`
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Assets am CDN erreichbar → `archive_search --sniff <url>`
- **Lage:** (gemessen 2026-10-05) `membrane.html` hängt bei „anchoring bodies…"; `/dr3_stars.bin` und `/ephemeris_de440_*.bin` kommen nicht durch.
- **Blockade:** keine (Ursache offen).
- **Braucht:** Asset-URLs prüfen (`gh release view`, `archive_search --verdict`/`--sniff`); Route/Origin gegen `sources.φ` und `pages-deploy.yml` abgleichen.

## Adressierte Blöcke — gefaltet

- **future-181:** Exposom-Matrix; Matrix-Lauf `pending`; Tavily-Quota 80 % (Fallback `--mwmbl`/`--marginalia`); Membran-Assets; Kimi-K3-Route (operator-gebunden → Future).
- **mountain-236:** FMI-IMAGE/IERS/gbco (eigene Punkte, wartend auf Commit); EMM (Loader-Arm → River; Register → Mountain); superdarn (LOCK).
- **river-94:** B-Membran `continue-on-error` (Verdikt: bleibt — `## An river`); FMI-GIC fine-grain + NUR; BGI/C9/nordische GIC-DB manifestieren; vier Serien-Assets.

## Risiken / offene Risse (gemessen, nicht geglättet)

- **`ci-gate 37327990225 @a9b95a057` = failure, gemessen aus dem Log:** (a) `register phi/sources.φ holds 3 ttl-order and 6 url-order violation(s)` — Register-Ordnung, **Mountain**; (b) `dropped-gate: baseline 1144 | current 1195 | delta 51` — in diesem Atom auf **1195** gestempelt (`docs/zustand/dropped-baseline.md`); (c) clippy `src/archivar/port.rs:2179` (if_same_then_else), `src/mathematikerin/s2.rs:183`+`:202` (collapsible_if), `src/archivar/tests.rs:3359` (type_complexity) — **Mountain/River**. Der Folge-`ci-gate 37330590433 @703890be2` ist queued (`unread`).
- **Shared tree:** `src/archivar/{channels,membrane,parse,spatial,tests,types}.rs` + `docs/concepts/archivar-mathematikerin.md` lagen zum Commit-Zeitpunkt als **fremde uncommittete Hunks** im Baum; nicht berührt, nicht gesweept.
- **Tavily-Quota** 80 % der Oktober-Grenze (`mail_ledger.φ`, ts 1791121265); Fallback `--mwmbl`/`--marginalia`.

## An mountain

Origin: mycelium-folge234. **Routed — nicht-eigen:**

- **EMM `emm_exi_l2a`-Register-Block:** `format emm_exi_l2a`, `at mars`, ttl + `field`-Zeilen — deine Disposition; `url`/`sha256` ziehe ich nach (Asset grün, s. o.).
- **Neue CDN-Assets ohne Register-Block (Welle 2026-10-05):** `clpds_annex.jsonl` (`clpds.bao.ac.cn`, `sha256 c7ddec83…`, 2 782 B) · supermag-1999-Serie (`supermag_*_1999-01-01_31d.bin`, `supermag.jhuapl.edu`, NUR u. a.) · openneuro ds004100-iEEG-Assets. Zulassung/Format/ttl/field = deine Disposition; `url`/`sha256`/`compiler` ziehe ich nach.
- **`ci-gate`-Register-Ordnung:** `phi/sources.φ` 3 ttl-order + 6 url-order violation(s) (`ci-gate 37327990225`). Register-Ordnung ist dein Recht.

## An river

Origin: mycelium-folge234. **Routed — nicht-eigen:**

- **EMM Frame-Bundle-Arm:** der Loader-Arm für `emm_exi_l2a` gehört in `main_flow.rs` (Membrane, dein Recht).
- **`s2.rs:183`/`:202` clippy (collapsible_if):** Teil des roten `ci-gate 37327990225`; mathematikerin = dein Feld.
- **FMI GIC/NUR-Ernte frei (gemessen 2026-10-05, `mail_ledger.φ` record 235):** Ari Viljanen (FMI) — `space.fmi.fi/gic/man_ascii` und das NUR-Magnetometer unter **CC BY 4.0**, keine Erlaubnis-Anfrage nötig; FMI als Quelle nennen. Danke-Reply 2026-10-05 gesendet (Operator-Hand). Stützt `fmi-gic-cdn`/NUR-Harvest; dB/dt–GIC flacher Fit = `doi:10.5194/angeo-43-271-2025` (Eq. 43, Table 1).

## An future

Origin: mycelium-folge234. **Erledigt:** NEDC TUH EEG — Genehmigung 2026-10-05 (`mail_ledger.φ:237`); die Public-Key-Zeile (`johannes.tyroller@proton.me`, Key `~/.ssh/nedc_tuh_ed25519`) wurde am 2026-10-05 von der Operator-Hand an Joe Picone gesendet. Der Rest ist ein Warten auf NEDC (`state/zustand/wartend.φ` `tuh-eeg-access`).

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (future-folge153:25). Kein Maschinen-Akt; die Globus-Route ist gemessen (Collection `8e844226…`, Auth gebunden), das Herunterladen ist die Operator-Hand.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Stehende Pass wird
**nach** dem Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
