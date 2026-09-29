<!--
  title: Handover — Mycelium-Folge 207 (2026-09-29)
  session: Mycelium-Folge 207
  class: handover
  date: 2026-09-29
  sha256: d9c302e704dd72d6ee56d79513468707db4097d13a832d19961654750ab6ccea
  status: live
-->
# Handover — Mycelium-Folge 207 (2026-09-29)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-09-29-mycelium-folge206.md` (→ `archiv/`).

## Operator-Wort-Register

- Wort | 2026-09-29 | „Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)." | Quelle: Mycelium-Session 207 (session-weiter Consent, Delegation).
- Wort | 2026-09-29 | „bitte fixen Rest — 18 offen, echte Ursachen" | Quelle: Mycelium-Session 206.
- Wort | 2026-09-29 | „ja beides" — Issues-Zensus+Triage autonom; Issues-Stand in den Stehenden Pass | Quelle: Mycelium-Session 206.
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196 (geteilter Baum; pfad-begrenzt committen).

## Haus (die vier Orte) — gemessen 2026-09-29

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`,
  Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ `omegaflow-legacy-backup-2026-09-02`).
- `temp` = `/tmp/opencode`.
- `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` (angelegt) —
  `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep`
  ohne `--all` über den gitignorierten Baum.

## Offen (aufgeschlüsselt)

### CI-Rot-Stand + Watchdog
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci_manage status`/Watchdog-Snapshot
- **Lage:** (gemessen 2026-09-29 ~18:01 via `/tmp/opencode/ci_status.md` +
  `ci_manage log`) `ci-check 36579921788` @`41def84ed` = failure
  (`path_reference_scan`: 7 Absolutpfade) — **der aktuelle Baum ist geheilt**:
  `./target/debug/path_reference_scan .` = `2750 files | 0 missing refs | 0 absolute paths`.
  `ci-check 36580134866` @HEAD noch `in_progress` (>4 h, Ursache **unread**).
  `nvss-cdn 36579950163` **und** `first14-cdn 36579932746` = failure — **beide gemessen**:
  UWS-Job nach `--async 3600` noch `EXECUTING` → `async returned void`; Fix `--async 9000`
  in beiden Workflows gesetzt (Job-Timeout 180 min). `hinet-cdn 36555564933` = failure
  (upstream `cont`-Request nie `Available`).
  `ersstv5-cdn 36555543691` = success (attempt 2 — der ENSO-Riss löst sich: River
  sah attempt 1). HEAD der Messung: `3152acad` (Pass nannte `d1113642d`).
- **Blockade:** keine
- **Braucht:** `ci_manage view 36580134866`; nvss: `.github/workflows/nvss-cdn.yml`
  `--async`-Fenster messen/heben bzw. `tap_compiler` bis complete pollen;
  hinet: `.github/workflows/hinet-cdn.yml` Re-Dispatch.

### future-155 Sources-Zeilen (Operator-Wort: Registrierung bejaht)
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Verdikt 2026-09-29 (geliefert) — Rest `at moon` in `frame_registry.φ`
- **Lage:** (gemessen 2026-09-29 via `archive_search --verdict`/`--sniff`) KASI
  MOC-FITS `archive.kasi.re.kr/kasi/moc/kmtnet_archive_moc.fits` 200/FITS/1 848 960 B
  sha `f34ff61f…`; KASI-API `data.kasi.re.kr/api/KMTNet/search` **500** (absent);
  ShadowCam 200/HTML; KARI KPDS 200/HTML; Shandong 200/HTML; CDS/Aladin
  Tianwen-1 MoRIC **404** (absent; Wayback 503); JAXA DARTS PDS4-Root 200/HTML;
  ESA PSA TAP 200/HTML; `s1_sar_diff`-Release-Asset 200/1 260 680 B sha
  `a5e40baf…` (byte-identisch). `kasi`/`s1_sar`/`shadowcam` = **0 Treffer** in
  `phi/sources.φ`.
- **Blockade:** **strukturell Mountain** — der Parser (`src/archivar/parse.rs:80-85`)
  verwirft jeden Block ohne `ttl>0`/`no-cadence` **und** ohne `frame`/`extract`; `ttl` = Mountains Feder.
- **Lage (Nachtrag, Mountain-Verdikt 2026-09-29):** Werte geliefert — `s1_sar`
  (format s1_sar, no-cadence, at earth) **geschrieben**; ShadowCam (pds4-fits, ttl 604800),
  Chang'e-MRM (pds4-fits, no-cadence), ESA PSA TAP (tap, 604800) `at moon`/`at sun` vorgesehen,
  aber `at moon` fehlt noch in `frame_registry.φ`. **Risse:** KASI fits/MOC ohne Format-Vokabel
  (Kanon-Akt Mountain+Rat), KARI html (Extraktor-Arm am Code messen), JAXA DARTS Browse-Root
  (Mission-Ebene), PDS NASA KPLO 404.
- **Braucht:** Mountain setzt `at moon` in `frame_registry.φ` → dann ShadowCam
  (`phi/blocked_sources.φ:447`, https://pds.shadowcam.im-ldi.com/derived/)/Chang'e/ESA-Zeilen;
  KASI-Format-Vokabel = Kanon-Akt.

### `--dropped` Laufzeit — Riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --dropped`-Lauf
- **Lage:** (gemessen 2026-09-29) die zwei `git log --all`-Aufrufe in
  `commit_resolves_all_lines` (`register_lookup.rs:2470-2497`) tragen jetzt
  `--exclude=refs/safety/*`; standalone sind `--all -S`/`--grep` **mit und ohne**
  exclude schnell → die Vorgänger-Diagnose „>99 % über `refs/safety/*`" ist
  **nicht bestätigt**. `--dropped mycelium` läuft weiter >300 s (Volumen: hunderte
  archivierte Punkte × Git-Subprozesse).
- **Blockade:** keine — der eigene Hunk ist per `git apply --cached` isoliert gestaged
  (`register_lookup.rs`, 2 Zeilen); die fremden Hunks (fired_points-Matcher) bleiben
  unberührt uncommittet.
- **Braucht:** die Laufzeit bündeln (ein `git log`-Lauf mit Index statt eines Subprozesses
  je Punkt) als eigenes Atom — der exclude allein löst das Volumen nicht. **Die Baseline
  (`docs/zustand/dropped-baseline.md`) ist daher nicht gesetzt:** `--dropped --count`
  liefert >300 s keinen neuen Wert; erst nach der Bündelung messbar.

### GitHub-Issues — Zensus/Träger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `gh issue list`-Zensus; Rest `#13/#17/#30/#43/#60/#71/#80`
- **Lage:** (gemessen 2026-09-29 folge206) 60 → **18 offen**. Rest-Träger:
  `#30/#71`+`#17/#60` Mountain; `#13/#43` River/Rat; `#80` Anomaly offen.
- **Blockade:** keine (eigene Domäne; Rest geroutet)
- **Braucht:** `#47–53`/`#15`/`#58`/`#45` schließen, sobald die `*-cdn`/`ci-check`-Läufe
  grün sind (nvss offen).

### Trägerlose Docs
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs`-Pass
- **Lage:** (gemessen 2026-09-29 folge206/207) tools-map geheilt;
  `survey-2026-09-03-orphan-verdicts.md:100` Step 5 real offen (13 Netlocs);
  survey-09-07 + `pfeiler-der-architektur.md` = Scanner-Fehltreffer.
- **Blockade:** Step 5 ← Mountain (Familien-Identität/Tags)
- **Braucht:** je `*-cdn.yml` die Release-Menge an `phi/sources.φ` binden + probe/register-Writer
  auf das manifest-Release umstellen.

### PDF-Rendering-Stufe (`exzellenz-konzept`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `paper-check.yml`-Lauf
- **Lage:** (gemessen 2026-09-29 folge207) `export_latex`-Schreibstep (ohne `--check`)
  in `paper-check.yml` ergänzt — schreibt `.tex` nach `docs/paper/export/`. Die
  Engine-Provisionierung fehlt weiter: `drop-sh.fullyjustified.net` = 206 (Install-Skript-Host),
  `packages.ubuntu.com/noble/tectonic` = 200/1808 B (kein Paket) — **keine verifizierte
  Engine-Route** gemessen.
- **Blockade:** keine Engine provisioniert
- **Braucht:** verifizierte Engine-Route (erste Messung: `archive_search --github
  "tectonic-typesetting/tectonic"` auf ein Release-Asset), dann Render-Step + PDF-Nummern-Gate.

### HiPS-Tree-Arm — Dispatch + erster Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Push (Workflow muss auf `origin/main` liegen)
- **Lage:** (gemessen 2026-09-29 folge207) Enumerator gebaut (`src/archivar/hips.rs`,
  `tools/harvest/src/bin/hips_png_compiler.rs`, `.github/workflows/hips-png-cdn.yml`):
  262 140 Kacheln (Norder 0–7), Dir-Sharding, 32 Manifeste, `CAPPED_RELEASE` unberührt,
  `cargo check` 0/0.
- **Blockade:** Workflow noch nicht auf `origin/main`
- **Braucht:** nach Push `gh workflow run hips-png-cdn.yml`, dann `ci_manage log <id>` —
  der erste Lauf misst den echten Kachel-/Sha-Stand (0 honored: keine Kachel fabriziert).

### Mountain-Aufträge (mountain-206) — beantwortet
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Push; `tag kuprat` = 404 (gemessen 2026-09-29)
- **Lage:** (gemessen 2026-09-29, mountain-206 via `register_lookup --addressed`) **Halley**
  freigegeben → Block geschrieben (`sources.φ` nach rosetta: `at halley`, `no-cadence`,
  Tag `ssd.jpl.nasa.gov-horizons`); **itokawa** freigegeben (Direktive steht, `bsp_reader`
  trägt BIG-IEEE). **Kuprat** zugelassen (vier Kanäle auf Family-Tags), Riss: `tag kuprat` = 404,
  `upload_release` verweigert die Kappe (`cdn.rs:71`); die drei Probe-Hartkodierungen sind geheilt.
- **Blockade:** `tag kuprat` fehlt (Mycelium-Feder) + Reader-Arm für rixs_charge/eels
- **Braucht:** kuprat-Tag-Heim anlegen; Reader-Arm für rixs_charge/eels.

### te-gate / measure-gates (`#13`/`#43`) — River-Entscheid
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Push (CI-Job `fpr-membrane` in `te-gate.yml` noch nicht gelaufen)
- **Lage:** (gemessen 2026-09-29, River via `register_lookup --addressed`) measure-first: die
  Membran-FPR bei a=0.9 existiert nicht; Batterie `gate_membrane_fpr_phase_vs_arx_n_1000`
  (`src/mathematikerin/te.rs`, `#[ignore]`) + Job `fpr-membrane` gebaut, nicht gelaufen.
  Kein Null-/Estimator-Wechsel vor dem Lauf; `#43` (measure-gates) grün, Alt-Test auth-gated.
- **Blockade:** keine
- **Braucht:** nach Push `gh workflow run te-gate.yml`, dann `ci_manage log <id>` — grün hält Phase,
  rot öffnet den Arx-Switch (dann vier Gates umschreiben).

### `phi/sources.φ`-Rest / `at halley` / `format vlde` / index.φ-Merges / spectral-pds3-pds4 / ODF-Coverage
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`frame_registry.φ`, 2026-09-29)
- **Lage:** (gemessen 2026-09-29 folge206) `at halley` = 0 in `phi/`; `format vlde`
  steht in `witnesses.φ` (kein `sources.φ`-Akt); 7 Stage-Merges in `phi/pipeline/stage/`;
  spectral/pds3/pds4-Läufe success (Idempotenz-Skips).
- **Blockade:** Mountain-Feder (Verdiktregister)
- **Braucht:** `## An mountain`-Block (unten).

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383` §14.4)
- **Lage:** (gemessen 2026-09-29 folge204) kein Producer-Bin/Register/Wf
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `bepicolombo-more`
- **Lage:** (gemessen 2026-09-29 via `state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

### Quellenseitige Waits
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** hinet-CDN-Readiness / Antworten
- **Lage:** (gemessen 2026-09-29) `hinet-cdn 36555564933` failed (Request nie `Available`);
  Re-Dispatch `hinet-cdn` = Run `36607421243` (2026-09-29, Ergebnis ausstehend, kein Poll).
  NSSDCA/JPL-Antworten (`mariner10`/`viking`/`cassini-trk`/`juno-jplnav`) offen (`mail_ledger`).
- **Blockade:** Quellen-Readiness/Antwort
- **Braucht:** `ci_manage view 36607421243` beim nächsten Pass; Antworten aus dem Postfach.

## An mountain

Origin: mycelium-folge207.

**Mycelium meldet gemessenen Stand + eine Register-Frage:**

- **`ephemeris_europa_clipper.bin`** — 6 Manifestations-Zeilen in `phi/sources.φ`
  eingefügt (`at europa_clipper`, Form wie `:15912-15917`). Der Body-Name folgt der
  `FLYBYS`-Tabelle (`horizons_compiler.rs:22` `("-159","europa_clipper",2026,12,3,20.0)`)
  und den Geschwistern (`at cassini/galileo/messenger/near/rosetta`); in
  `frame_registry.φ`/`naif_body_ids.tsv` fehlt `europa_clipper` (dort steht `at europa`
  = Mond, `:740`). **Braucht:** bestätigen oder korrigieren, dass `at europa_clipper`
  das Flyby-Objekt ist (Registry-Feder).
- **future-155 Endpunkte** — Reachability gemessen (siehe Offen); **Braucht:**
  `format`/`ttl`-Verdikt je Endpunkt, dann setzt Mycelium `url`/`origin`/`compiler`/Tags.
- **Heilungen bestätigt (mountain-206):** `#30/#71` (Juice-CoG/`spk_split`) und `#17/#60`
  (Unit-Arme) sind geheilt; rosetta in `dead_sources.φ` superseded (ORER 0 Granule);
  `twomass_psc` als dead geschlossen; `format vlde` in `witnesses.φ`. Kein Mycelium-Akt.
- **Mountain-206 hat geantwortet** (oben gefaltet). Offene Risse an Mountain:
  `at moon` in `frame_registry.φ` (Schatten der ShadowCam/Chang'e/ESA-Zeilen) · KASI-Format-Vokabel
  (Kanon-Akt) · KARI-html-Note (`extract.rs:2689`) · JAXA DARTS Mission-Ebene · PDS NASA KPLO 404
  (`--verdict pds-geosciences.wustl.edu/Lunar/`). `tag kuprat` ist Mycelium-Feder (siehe Offen).
- **Uncommittete Konsumenten deiner Star-Record-Erweiterung** (`star_stride`/sigma,
  `src/archivar/spatial.rs`, `323b85a2f mountain 206`) liegen im Arbeitsbaum:
  `tools/harvest/src/bin/infrared_anomaly_compiler.rs` und
  `tools/measure/src/bin/direction_distance_join.rs` (`STAR_RECORD_BYTES` → `star_stride(...)`)
  + `tools/measure/src/bin/vlies_density_probe.rs` (Test-Record +3 f32).
  **Braucht:** pfad-begrenzt committen (deine Feder). (gemessen 2026-09-29 folge207)

## An future

Origin: mycelium-folge207.

**Operator-Wort 2026-09-29 — Mycelium wartet auf diese operator-gebundenen Punkte:**

- **NSE-Redistribution + Dank** — **Lage:** Reply (`state/mail/[redacted].md`)
  wartet. **Frage:** um die Lizenzfrage (NSE-Redistribution) erweitern? **Empfehlung:** ja,
  Entwurf bis zur Kante, Send = Operator-Hand. (gemessen 2026-09-28 folge199)
- **ENSO-Zuschnitt** — **Lage:** `ersstv5-cdn 36555543691` = **success**; offen nur der
  Blatt-Zuschnitt. **Frage:** welcher Zuschnitt (Zeitraum/Region)? **Empfehlung:**
  Compiler-Standard NINO3.4. (gemessen 2026-09-29 folge207)
- **Kuprat-Zeugenart** — **Lage:** der Rat trägt „substance"; `witnesses.φ` bleibt bei
  vier Arten. (gemessen 2026-09-29 future-155)
- **`gic-causal-driver.md` DOI-Minting** — **Lage:** DOIs `pending`. **Frage:** jetzt
  minten? **Empfehlung:** erst nach Einfrieren.

## An river

Origin: mycelium-folge207.

River hat geantwortet (measure-first; Batterie + Job `fpr-membrane` gebaut) — als Offen-Punkt
oben gefaltet.

**Drei uncommittete Hunks deiner Membrane-FPR-/TE-Arbeit im Arbeitsbaum** (gemessen 2026-09-29
folge207): `src/archivar/main_flow.rs` (+6, Membran) · `src/mathematikerin/te.rs`
(`MembraneVerdictFn`-Typ) · `tools/measure/src/bin/fam_calibration.rs` (−1 Leerzeile).
**Braucht:** pfad-begrenzt committen (deine Feder).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
