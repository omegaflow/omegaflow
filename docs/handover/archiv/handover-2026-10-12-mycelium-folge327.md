<!--
  title: Handover — Mycelium-Folge 327 (2026-10-12)
  session: Mycelium-Linie in einem Pass (INPOP/NOE4-Fix, adressierte Faltungen, Stehender Pass)
  class: handover
  date: 2026-10-12
  sha256: 3bc20455e08a5a7bb44e714c95b3f33fb3199aa1be4888e80bbe34d98f3abb6c
  status: live
-->
# Handover — Mycelium-Folge 327 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-12-mycelium-folge326.md` (→
`archiv/`) und faltete die adressierten Blöcke `future-folge242` und `mountain-folge327`
**mit Token** (die folge326-Faltung war unvollständig: `xfer:mountain327-1`- und
`-2`-Einträge fehlten im Live-Handover).

## Burn: open 0.00 · close 0.34 · cap 0.40 — Grund: Mycelium-Folge 327 in einem Atom (opencode.db Session-Cost 0.3366, Live-Flush): INPOP/NOE4-Fix (grün) · BC430+P10+P11 Register/Mirror · Taucher-Staffel (6 Punkte) · CI-Fold-Fix · Browser-Stimmen-Runde (4 Seats, Operator-Ok) · adressierte Faltungen · Stehender Pass; deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „also höre zu beide linien mountain und river sind beschäftigt … entweder arbeitet ihr ab jetzt projektweise oder ihr findet einen anderen weg" | 2026-10-12 | Operator (Session 323) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „auch das ist wieder problematisch mountain arbeitet an den ephemeriden er kann sich nicht darum kümmern" | 2026-10-12 | Operator (Session 325) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „ist das wirklich alles Trigger oder was kannst du davon selbst bis zur Kante abarbeiten?" | 2026-10-12 | Operator (Session 326) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-12 | Operator (Start Mycelium-326/327) · session-weiter Delegations-Consent, nicht das Commit-Wort |

## Gefaltet (gemessen 2026-10-12)

**`future-folge242` (adressiert, ohne Token — Altblock):** vollständig gefaltet.
- USGS-Kombi-Mosaik `Mercury_M1_M2_M3_M10_mosaic_global_2010.tif`: der `future-242`-Wert
  (`1a90a8ff…`/66 414 088 B) ist **ungültig**; gemessen `324c0e49…`/1 880 562 475 B. Der
  Registerblock `phi:31582-31586` trägt **keine `sha256`-Zeile** → als Ask an Mountain
  (`xfer:mycelium327-1` unten); der M10-Mosaik-Lauf `38237291082` ist **success** (8 Jobs grün).
- `archive_search --all`-Spill: **geschlossen** — `OMEGAFLOW_SPILL_DIR` wird bereits honoriert
  (`tools/utils/src/bin/archive_search/net.rs:3780`).
- NIST NCNR NSE: **geschlossen** (Lauf `38233919420` grün; folge326).
- Fetch-Provenienz Sidecar `<datei>.src`: **geschlossen** (gemessen via Taucher) — beide Sinks schreiben den
  Sidecar bereits (`tools/utils/src/bin/archive_search.rs:1173`, `tools/utils/src/bin/sfetch.rs:87`); die
  Lücke war die rohe `curl -o`-Nutzung der Taucher → Delegation muss `sfetch --out` bzw. die Tool-Sinks nennen.
- `de441-de442-sha`: → Ask an Mountain (`xfer:mycelium327-2` unten).
- `dropped-gate-move-disposition`: **geschlossen** (gemessen via Taucher) — 0 **verlorene** Punkte; alle 9
  `git: none`-Zeilen sind Fortsetzung/Rename oder commit-aufgelöst (8 distinct; die river-Zeile ist doppelt).

**`mountain-folge327` (adressiert):**
- `xfer:mountain327-1` 8-Ziele-Einkaufsliste (future-241) → eigener Punkt unten.
- `xfer:mountain327-2` Orphan-Doc `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` → Trägerpunkt unten.
- M10-Kamera/ISIS-Kette (ASU `url` `phi:31568`, `terms`/`format`, `camera_angles`-Parser-Arm),
  DSC 0119, JADES, BK18, FMI-NUR, NIST, USGS-Mosaik: registriert (folge326) bzw. Mountain-Ask.
- **Korrektur:** folge326 trug „Sonke/Robinson-Tabellen 1–8" als Mycelium-`blockiert` mit
  `Braucht: archive_search`. Gemessen (mountain-327:65-71): die ASU-`url`-Zeile `ser.im-ldi.com/M10/M10_ISIS/`
  ist **registriert** (`phi:31568`); die Tabellen 1–8 sind **paper-only** (voller cnet-Satz) —
  kein messbarer URL-Satz, letzte Meile Operator-Hand → `## An future`.

## Offen — eigen

### GRDC-Caravan — Resume-Fix gebaut; Verifikationslauf läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `grdc-caravan-cdn`-Lauf `38234294097` endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage view`) Lauf `38234294097` (head `934c131f3`) **in_progress**;
  Resume-Fix (`src/archivar/fetch.rs`) steht.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 38234294097`.

### CI-Messzeilen — Fold rot (`Author identity unknown`); Workflow gefixt, neu dispatcht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `ci-measurements-fold`-Lauf endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38242415976`) Lauf `38242415976` **failure**:
  `block_commit --path tools/measure/data/ci_measurements.φ --key 'run 38238626309' …` →
  `block_commit: Author identity unknown` (exit 2) — dem CI-Runner fehlte die git-Identität; der Fold hatte
  13 Artefakt-Blöcke gesammelt. **Gefixt:** `.github/workflows/ci-measurements-fold.yml` setzt jetzt
  `git config user.name/user.email` (kanonisch `omegaflow-bot`/`bot@omegaflow.space`, wie `cdn-reconcile.yml`).
- **Blockade:** keine.
- **Braucht:** `ci_manage log <neuer fold-run-id>` (durch einen frischen `field-te-query`-Lauf getriggert)
  + `sread tools/measure/data/ci_measurements.φ`.

### 8-Ziele-Einkaufsliste — BC430 registriert + Mirror dispatcht; P10/P11/Venus offen `xfer:mountain327-1`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der Zenodo-GET für Venus-`Processed.zip` liefert wieder einen Körper statt `504 Gateway Time-out`
- **Lage:** (gemessen 2026-10-12) **BC430** Zenodo `3558850`: sha `0b2da752…`/59 541 013 B → **registriert**
  (`phi:32389-32394`) + `zenodo-raw-cdn.yml`, Lauf `38244619269` **success** (gemirrort). **P10** `13309156`
  (`SC_23.asc`, sha `330126869…`/678 784 528 B) und **P11** `13309040` (`SC_24.asc`, sha `271888047…`/
  721 591 221 B) **gemessen** (`curl --http2 --retry --retry-all-errors`) → **registriert** (`phi:32411-32422`,
  nach `llr`/vor `rixs`, Sort-Dry-Run 724 = Baseline) + Matrix-Zeilen in `zenodo-raw-cdn.yml`. **Venus-Radar**
  `18176179`: `Processed.zip` (409 371 501 B) weiter **`504 Gateway Time-out`** (92-B-Body, auch mit `--http2`/
  Retry); `.tif` 80 909 342 816 B (nicht machbar). **Riss/gemessen:** HTTP/1.1 → 504; `--http2 --retry` löst die
  großen GETs (BC430/P10/P11 kamen durch), **Venus nicht** (Gateway-Timeout auf jenem Record); Rate-Limit
  `x-ratelimit-limit: 133`, `retry-after: 25`.
- **Blockade:** Venus an Zenodo (Record-spezifischer Gateway-Timeout). Register-Ordnung: `register_sort --write`
  würde 175 fremde (inpop-)Zeilen sweepen → Blöcke manuell an kanonischer `(ttl,url)`-Position gesetzt.
- **Braucht:** `ci_manage log <zenodo-raw-cdn-run-id>` (P10/P11-Mirror, push-getriggert); Venus
  `curl -sSL --http2 --retry 3 --retry-all-errors --max-time 900 -o /tmp/opencode/venus.zip '<Processed.zip-content-url>' && sha256sum …`
  sobald der 504 weicht. Nicht-cc-by (PDS/JPL) `terms` → Mountain.

### Orphan-Doc-Träger — fmhy-Survey; Schritte 1–3 gemessen `xfer:mountain327-2`
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-12) Träger dieser Zeile ist
  `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` (2 offene Marker). **Schritt 1 (Jina)
  bestätigt** (`--jina` im Baum `archive_search.rs:312` + Manifest `5d0f9652d` ⊃ `c32af6d0d`, e2e getestet).
  **Schritt 2 (SearXNG) gebaut** (`$OMEGAFLOW_SEARXNG`, Commit `78a4a8057`). **Schritt 3 (AI2 Playground)
  gemessen** (Browser-Runde, Operator-Ok): keyless, aber Terms-Gate; Olmo 3.1 32B = **3/4** (0-Kanon-Szene
  verfehlt) → unter der 4/4-Gate, kein Roster; `docs/concepts/ui-seats.md` fortgeschrieben. **ui-seats**
  damit ebenfalls gemessen (Sakana ~40 s/4/4 · SciSpace Login-Wall · Gemini Notebook kein freier Composer)
  → Punkt zu.
- **Blockade:** keine.
- **Braucht:** die restlichen Survey-Schritte 4–7 (Elicit/Consensus, Sci-Bot → Rat-Verdikt vor Berührung,
  NVIDIA NIM) gegen den Baum messen/entscheiden; der Träger bleibt bis dahin.

### tools-map.md — Träger (Operator-Wort Debugger-Rechte)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort (Debugger-Rechte am live Chrome)
- **Lage:** (gemessen 2026-10-12) `docs/concepts/tools-map.md:349`: „offen bleibt allein das
  Operator-Wort (Debugger-Rechte am live Chrome)"; die Route ist gebunden (`opencode.json:172-178`,
  Commit `7e79c194d`). Träger-Zeile für `--orphan-docs` (die folge326-Zeile ist nach `archiv/` gewandert).
- **Blockade:** der Akt ist operator-gebunden.
- **Braucht:** das Operator-Wort — liegt in Futures Operator-Queue (`## An future`).

## An mountain

Origin: mycelium-folge327 (2026-10-12). Der `future-242`-Block war bei folge326 nur als Kopfzeile
„gefaltet" (Token-Pflicht verletzt); hier einzeln gefaltet.

- **M10↔MDIS-USGS-Mosaik — `sha256`-Zeile fehlt.** Der Lauf `38237291082` ist **success** (8 Jobs grün);
  der Block `phi:31582-31586` (`url …/Mercury_M1_M2_M3_M10_mosaic_global_2010.tif`, `terms PD`, `origin`,
  `compiler raw_mirror.rs`, `no-cadence`) trägt **keine `sha256`-Zeile**. Gemessen (Mountain-326):
  `324c0e49…` über 1 880 562 475 B. `xfer:mycelium327-1`
- **`de441-de442-sha`** — die NAIF-Blöcke (`phi:32008`, recursive SPK/PCK harvest) stehen; `sha256`
  erst nach dem Merge-Lauf (Mountain-Territorium). `xfer:mycelium327-2`

## An future

Origin: mycelium-folge327 (2026-10-12). Operator-gebundener Rest (No-referral → Futures Operator-Queue).

- **Sonke/Robinson (M10) Tabellen 1–8 — paper-only, letzte Meile Operator-Hand.** Gemessen (mountain-327):
  Sonke & Robinson 2026, PSJ 7:35, `10.3847/PSJ/ae32f5` — **OA gold, CC-BY** (`--unpaywall`, host publisher);
  der volle cnet-Satz ist nicht maschinell erreichbar (`ser.im-ldi.com/M10/M10_ISIS/` = 403, kein Listing;
  die ASU-`camera_angles.txt`-`url` ist registriert `phi:31568`). Wird gebraucht für das M10-MDIS-Pose-Wiring
  (Mountain/River). Möglichkeit: die Autoren anschreiben (Operator-Hand, kein Maschinen-Send) **oder**
  mit ISIS nachrechnen. `xfer:mycelium327-3`
- **Debugger-Rechte am live Chrome** — Operator-Wort nötig (`docs/concepts/tools-map.md:349`); ohne es
  bleibt die DevTools-MCP-Bindung (`opencode.json:172-178`, Commit `7e79c194d`) ungenutzt. `xfer:mycelium327-4`

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
