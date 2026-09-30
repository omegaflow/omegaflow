<!--
  title: Handover — Mycelium-Folge 210 (2026-09-30)
  session: Mycelium-Folge 210
  class: handover
  date: 2026-09-30
  sha256: 460a7e800c156b77ab769c11e6bc0bf577d01417f6c64093c893393537492f34
  status: live
-->
# Handover — Mycelium-Folge 210 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-09-30-mycelium-folge209.md` (→ `archiv/`).

## Operator-Wort-Register

- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209 (`## An mycelium`) — keine Ausnahme für vorbestehende Register-Verstöße in `phi/`; jede `note`-Zeile ≤ 256 Zeichen, keine `#`-Kommentarzeilen in den gated Registern; in diesem Atom gefaltet.
- Wort | 2026-09-30 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)." | Quelle: Mycelium-Session 210.
- Wort | 2026-09-30 | „es DARF nichts machbares in der nächsten session landen das ist ein befehl" | Quelle: Mycelium-Session 210.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.

## Haus (die vier Orte) — gemessen 2026-09-30

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote
  `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02).
- `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` —
  `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep`
  ohne `--all` über den gitignorierten Baum.

## Offen (aufgeschlüsselt)

### CI-Rot-Stand nach Gate-Heilung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-gate`/`ci-check`-Lauf am neuen HEAD
- **Lage:** (gemessen 2026-09-30 via `ci_manage`) `ci-gate 36692099934` rot:
  `clippy::too_many_arguments` `src/mathematikerin/te.rs:3139` — geheilt von river70
  (`8ab227d6e`); `format` rustfmt `src/gate/commit_gate.rs:1870` +
  `tools/gate/src/bin/commit_check.rs:1` — in diesem Atom geheilt; `dropped-gate`
  delta 7 — in diesem Atom geheilt (`register_lookup --dropped` selbst-messend/träger-bewusst).
- **Blockade:** keine
- **Braucht:** neuen `ci-gate`/`ci-check`-Lauf lesen (`ci_manage jobs <id>`); bei grün
  ist der ci-Rot-Stand leer.

### register_lookup --dropped — Träger-Auflösung Rest
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächste Messung des Rest-Gaps (`register_lookup --dropped --count`)
- **Lage:** (gemessen 2026-09-30 via `./target/debug/register_lookup --dropped --count`)
  **1067** in 53,3 s (< Baseline 1157 → `dropped-gate` grün). Rest-Gap ~97 vs. historisch
  1164 aus (a) Message-Korpus case-insensitiv, (b) `-G`- vs. `-S`-Semantik;
  register-gehaltene Träger (`phi/*.φ`, `wartend.φ`) brachten nur −6 bei +26 s → nicht
  eingebunden.
- **Blockade:** kein Index für Register-Träger
- **Braucht:** n-Gramm-/Hash-Index für Register-Träger, damit Register-gehaltene Punkte
  als getragen zählen (Laufzeit < 60 s). Laufzeit ist mit 53 s unter, aber nicht „well
  under" 60 s — ggf. Release-Build statt debug im Gate.

### Neue Compiler-Bins — Arm-Risse
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Arm-Erweiterung (`## An mountain`)
- **Lage:** (gemessen 2026-09-30 via `cargo check` 0/0 + lokaler Lauf) vier Bins gebaut
  (`pds4_fits`/`pds4_binary`/`pds3_binary`/`pds3_img`) + workflows, aber die Arme decken
  die echten Daten nicht: `pds3_binary` decodiert nur int-Breiten 1/2/4/8 (LRS:
  `OBSERVATION_TIME`/`IEEE_REAL`/`WAVEFORM` → 8/9 Spalten gedroppt); `pds3_img` Mini-RF
  `PC_REAL` hat keine Byte-Order-Regel (0 honored, kein Asset), ENVI-Feldmap ungemessen;
  `pds4_fits` nimmt erstes `IMAGE`-HDU (Map in `BINTABLE` unverifiziert); `pds4_binary`
  nur `data_raw`-Route.
- **Blockade:** Parser-Arm-Lücken
- **Braucht:** `## An mountain` (Arm-Erweiterung + ttl/frame).

### Legacy-CDN Re-Manifest — tapvizier jetzt erreichbar
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Dispatch `nvss-cdn`/`first14-cdn`/`kernel-flatten`
- **Lage:** (gemessen 2026-09-30 via `archive_search --verdict`) `tapvizier.cds.unistra.fr`
  root **206**, `/TAPVizieR/tap/capabilities` **200** (5919 B) — nicht void. Der
  `spk_split`-Ordnungsfehler (`kernel-flatten`) bleibt ein Kern-Bug.
- **Blockade:** voriger async-void möglicherweise überholt
- **Braucht:** `gh workflow run nvss-cdn.yml` + `gh workflow run first14-cdn.yml` erneut;
  `spk_split` → mountain.

### gosat-cdn / hips-png-cdn — laufende Läufe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende
- **Lage:** (gemessen 2026-09-30 via `ci_manage`) `gosat-cdn` neu dispatcht
  `36693666593` (in_progress); `gosat-cdn 36283215548` rot: `returned void` (Server-Resultat
  leer, alle Splits) → Mountain-Quelle. `hips-png-cdn 36675582975` in_progress, Shards
  (4,0)/(5,0) rot: `CDN upload returned void` (Kacheln 3072/3072 + 10000/10000 entschieden,
  0 absent), Norder 6 Shards laufen; `hips-png-cdn` neu dispatcht `36693670610` für MoRIC.
- **Blockade:** CDN-Release-Erstellung
- **Braucht:** Lauf-Ende lesen; CDN-Release-Upload-Ursache (`gh release`-Cap/Rechte).

### CDSE-CCM — neue Freigabe, STAC gemessen
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`## An mountain`)
- **Lage:** (gemessen 2026-09-30 via `archive_search --verdict`) Freigabe (mail 198);
  maschinenlesbarer Endpunkt **`https://catalogue.dataspace.copernicus.eu/stac/collections`**
  200 (53309 B); OData `/odata/v1/` 404.
- **Blockade:** Admission/Feldmap fehlt
- **Braucht:** `## An mountain` (Admission + Compiler-Arm STAC).

### NOAA-NCDC-CDO-Token
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Token-Eingang (mail 199)
- **Lage:** (gemessen 2026-09-30 via `archive_search --verdict`)
  `www.ncei.noaa.gov/cdo-web/api/v2/` 400 am Root (Token-Probe nicht ausgeführt —
  `.secrets.local` ist deny-gelistet; Key-Namen-Prüfung `NOAA_CDO_TOKEN` steht aus).
- **Blockade:** Token
- **Braucht:** `awk -F= '{print $1}' .secrets.local` auf Vorhandensein von `NOAA_CDO_TOKEN`
  prüfen (nur Name), dann CDO-Endpunkt mit Token messen.

### hinet-cdn Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Route trägt (`hinet-cdn`)
- **Lage:** (gemessen 2026-09-30 via `archive_search --verdict`)
  `https://hinetwww11.bosai.go.jp` direct **403**; Werkzeug schlägt Proton-Exit vor
  (bisheriger Fehler: „cont status never read Available").
- **Blockade:** 403 direkt; Wayback nur 2013
- **Braucht:** Operator-Wort für Proton-Exit, oder Re-Dispatch `hinet-cdn` bei Readiness.

### KARI KPDS (Danuri/KPLO) — kein Maschinen-Endpunkt
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Verdikt (descope oder neuer Endpunkt) (`## An mountain`)
- **Lage:** (gemessen 2026-09-30 via `archive_search --playwright`)
  `https://www.kari.re.kr/kpds/` 200, gerendert nur Login/Search/`mailto:` — **kein**
  maschinenlesbarer Endpunkt (0 honored); `extract.rs:2689` HTML-Arm steht.
- **Blockade:** kein Daten-Endpoint
- **Braucht:** `## An mountain` (descopen mit Befund oder auf Portal-Antwort warten).

### te-gate #112/#13/#43 — River
- **Status:** wartend | **Bindung:** eigen ← river
- **Trigger:** `te-gate`-Lauf
- **Lage:** (gemessen 2026-09-30) `#112` rot bestätigt; river70 trägt die Membran-FPR
  auf 128 Trials (Rat). `#13` (n=1000 FPR) + `#43` (measure-gates) bleiben.
- **Blockade:** keine
- **Braucht:** `## An river`.

### Quellenseitige Waits (`state/zustand/wartend.φ`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort/Readiness
- **Lage:** (gemessen 2026-09-30 via `state/zustand/wartend.φ`) NSSDCA/JPL-Antworten
  (`mariner10`/`viking`/`cassini-trk`/`juno-jplnav`) offen; Asmar (JPL) verweist auf die
  Paper-Autoren (Antwortmail 1790697586); drei ODF-Anfragen (ESOC/KinetX/Turyshev) raus.
- **Blockade:** Antwort
- **Braucht:** Postfach (`mail_ledger.φ`) lesen; bei Antwort Ernte/Route prüfen.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2026-12-03 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `europa-clipper` · `bepicolombo-more`
- **Lage:** (gemessen 2026-09-30 via `state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium.
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

### GitHub-Issues — Zensus
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Issues-Zensus (`ci_manage list`)
- **Lage:** (gemessen 2026-09-30) offen u. a. `#81` (clippy), `#80` (Anomaly, unread),
  `#58/#15` (cargo-test-GPU), `#60/#17` (recheck-live), `#13/#112` (te-gate), `#43`.
- **Blockade:** keine
- **Braucht:** `ci_manage list` + `gh issue list`; nach Heilung schließen.

### Trägerlose Dokumente
- **Status:** wartend | **Bindung:** eigen ← future (ein Dokument)
- **Trigger:** nächster `register_lookup --orphan-docs`-Pass
- **Lage:** (gemessen 2026-09-30 via `register_lookup --orphan-docs`) **4**:
  `docs/concepts/exzellenz-konzept.md` (2), `docs/concepts/kybernetische-astrophysik.md` (1),
  `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (1) → Mycelium;
  `docs/surveys/survey-messpunkt-verteilung.md` (3) → **River** (river-folge47–65, nicht
  Sensory). `docs/auftrag/auftrag-gic-einreichung.md` → Future (dessen Übergabe privat).
  Die frühere 0/Meldung war transienter Carrier-Verlust (archivierte Übergabe).
- **Blockade:** Prosa-Marker ohne echten offenen Akt
- **Braucht:** die drei Mycelium-Dokumente mit einem Träger benennen oder descope-annotieren;
  `survey-2026-09-03-orphan-verdicts.md` Step 5 umsetzen/descopen; River + Future je eigenes.

### future-155/156 Sources-Zeilen — Aufnahme-Regel
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-`ttl`/`frame`-Zeilen (`phi/sources.φ`)
- **Lage:** (gemessen 2026-09-30) `parse.rs:80-85` verlangt `ttl>0`/`no-cadence` **und**
  `frame`/`extract`; ShadowCam/ESA-PSA-TAP/Chang'e-MRM register-reif bis auf Mountain-Feder.
- **Blockade:** Mountain-`ttl`/`frame`
- **Braucht:** `## An mountain`.

### kuprat Family-Tag — Phantom
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`witnesses.φ`)
- **Lage:** (gemessen 2026-09-30) kein Compiler setzt `tag kuprat`; die vier Kanäle liegen
  unter gemessenen Tags (`crystallography.net`/`srdata.nist.gov`).
- **Blockade:** kein `kuprat`-Host
- **Braucht:** `## An mountain`.

### ersstv5 403 — Diagnose
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** `ersstv5-cdn`-Lauf mit Diagnose-Step
- **Lage:** (gemessen 2026-09-30, mountain-208) 403 runner-spezifisch; lokal 200
  (14 999 659 B); `fetch.rs:149` ohne UA widerlegt.
- **Blockade:** Diagnose-Step fehlt
- **Braucht:** `curl -g -D - -o /tmp/body '<URL>'` im `ersstv5-cdn`-Workflow-Log.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383` §14.4)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

## An mountain

Origin: mycelium-folge210.

- **Parser-Arm-Lücken (blockieren vier neue Compiler-Bins).** `src/archivar/pds3_binary.rs`
  decodiert nur int-Breiten 1/2/4/8 — LRS braucht `IEEE_REAL`/`CHARACTER`/Vektor-`ITEMS`
  (8/9 Spalten fallen). `src/archivar/pds3_img.rs::byte_order_of` kennt kein `PC_REAL`
  (Mini-RF → kein Asset, 0 honored). `pds4_fits.rs` nimmt das erste `IMAGE`-HDU — prüfen,
  ob die Chang'e-Maps in einer `BINTABLE` liegen. Bins: `tools/harvest/src/bin/pds3_binary_compiler.rs`,
`tools/harvest/src/bin/pds3_img_compiler.rs`,
`tools/harvest/src/bin/pds4_fits_compiler.rs`,
`tools/harvest/src/bin/pds4_binary_compiler.rs`.
- **ttl/frame für die drei register-reifen Endpunkte** (ShadowCam `at moon`+`ttl 604800`,
  ESA PSA TAP `ttl 604800`+`field`/`at`, Chang'e MRM `no-cadence`) — dann setzt Mycelium
  `url`+`format`.
- **CDSE-CCM Admission:** `https://catalogue.dataspace.copernicus.eu/stac/collections`
  (200, 53309 B) — neuer STAC-Arm + ttl/frame.
- **`gosat-cdn` „returned void":** Server liefert leeres Suchergebnis über alle Splits
  (`ci_manage log 36283215548`) — Source-Seite.
- **KARI KPDS:** kein maschinenlesbarer Endpunkt (0 honored) — descopen mit Befund oder
  auf Portal-Antwort.
- **DAS2 Iowa (`blocked_sources.φ:374`):** `src/archivar/port.rs:1369`
  `hapi_draft_fields_csv` ist ein **Register-Draft-Textgenerator**, kein Datenleser
  (verwirft die Zeilen, einziger Sink `fields`); kein HAPI-CSV→Asset-Packer. Ein
  Compiler braucht einen echten Reader (Vorbild `omni_hro_compiler.rs`).
- **`kernel-flatten` `spk_split`-Ordnungsfehler** (Kern): „segment data begins before
  address … summary arrived after its data" → `curl: (23)` (`ci_manage log 36639779027`).
- **Riss:** `blocked_sources.φ:443` (`gap html-parser-arm`) widerspricht `extract.rs:2689`.

## An river

Origin: mycelium-folge210.

- **`te-gate #112`** rot bestätigt; `#13`/`#43` bleiben. river70 trägt die Membran-FPR
  auf 128 Trials — bitte an `#112`/`#13` weiter. Braucht River-Entscheid Phase vs. Arx-Switch.
- **`ci-check` cargo-test GPU:** `36688544545` @`7fa58643d` 4 Ausfälle
  (`te_gpu_crosscheck`, `te_wgsl_validates_offline`, `gate_wgsl_horizon_and_ksg_parity`,
  `volume_probe_parity_masked_corner_and_plain`) — WGSL `find_cross_mi_lag`-Arität;
  river70 hat `shaders.rs`/`te.rs` geheilt, neuer Lauf bestätigt.

## An future

Origin: mycelium-folge210.

- **Secret-Rotation** — GitHub-PATs `omegaflow-read`/`omegaflow-ci-write` wurden
  regeneriert (mail 200/201) → `http_401`-Residuum beobachten; Taucher-Prompts verbieten
  `.secrets.local`-Werte.
- **NSE-Redistribution + Dank** — Reply `state/mail/[redacted].md` wartet.
- **ENSO-Zuschnitt** — `ersstv5-cdn` success; offen nur der Blatt-Zuschnitt (NINO3.4).
- **`gic-causal-driver.md` DOI-Minting** — DOIs `pending`; erst nach Einfrieren.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
