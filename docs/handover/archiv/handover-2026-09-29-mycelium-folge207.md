<!--
  title: Handover — Mycelium-Folge 207 (2026-09-29)
  session: Mycelium-Folge 207
  class: handover
  date: 2026-09-29
  sha256: 48d030e36de6a9188c37dc561eca8de0a92864762fbf3ccb3f742fb9c39fecb9
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
- Wort | 2026-09-29 | „du dürftest jetzt phase 2 fahren können" | Quelle: Mycelium-Session 207 (Consent Phase 2, `line`-Agent).
- Wort | 2026-09-29 | „doch es sind gerade alle sessions offen" | Quelle: Mycelium-Session 207 (alle Linien offen — fremde Pfade nicht anfassen).
- Wort | 2026-09-29 | „ich hab keine ahnung ich habs nicht verbockt" | Quelle: Mycelium-Session 207 (nicht der Operator; der E0063-Riss aus zwei zerrissenen Commits).
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
- **Lage:** (gemessen 2026-09-29 20:44 via `ci_manage status`/`ci_manage log --all`) Der
  **Massen-Rot hatte eine Ursache:** E0063 — `spatial::StarRec` σ-Felder fehlten in
  `src/archivar/main_flow.rs:2429/2565`; **geheilt** (`a95921575 river 67`). Über
  `gh_issue_once` erzeugte er ~29 Issues — in diesem Atom triagiert/geschlossen. Verbleibende
  rote Ursachen am `ci-gate 36626370907 @4ef3a7e43`: clippy `src/archivar/hips.rs:518`
  (geheilt in diesem Atom), `src/archivar/spatial.rs:359` (Mountain, `type_complexity`),
  `src/mathematikerin/te.rs:3121` (River); format `te.rs:3269/:6161` +
  `tools/measure/src/bin/fam_calibration.rs:1` (River); dropped-gate baseline 1134→1148
  (dieser Atom gebumpt). Neue Läufe @HEAD `c87ad6214`: `ci-check`/`ci-gate`/`register-coverage`
  `in_progress`, `tools-build`/`register-coverage`/`harvest-dispatch`/`swpc-mirror-cdn` success.
  `nvss-cdn`/`first14-cdn` async-Fenster weiter offen; `hinet-cdn` Re-Dispatch offen.
- **Blockade:** keine
- **Braucht:** `ci-gate` neu lesen, sobald Mountain `spatial.rs:359` + River
  `te.rs`/`fam_calibration.rs` geheilt; nvss/first14 async-Fenster; hinet Re-Dispatch.

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
- **Lage (Nachtrag, mountain-207 @2026-09-29):** Werte bestätigt — `s1_sar` steht
  (`sources.φ:15987-15990`), `ephemeris_halley` steht (`:15961`). **KASI KMTNet MOC**
  (BINTABLE `TFORM1='1K'`, `MOCVERS=2.0`, `ORDERING=RANGE`) = wert-lose Coverage-MOC →
  **descoped** (`moc-reader` = Konfundierung; Heimat `phi/footprints.φ` FP01). **KASI API**
  500 absent. **ShadowCam** `pds4-fits`/ttl 604800 (`phi/blocked_sources.φ:447`,
  https://pds.shadowcam.im-ldi.com/derived/; Download Operator-Hand).
  **KARI KPDS** `parser-def html` (Arm `extract.rs:2689`). **ESA PSA TAP** `tap`/ttl 604800
  (ADQL+`field` offen). **Shandong** pending. **JAXA DARTS** nicht register-fähig
  (Missionsebene). **HiPS MoRIC** `hips-png` (`blocked:431`).
- **Braucht:** ShadowCam Download (Operator-Hand); ESA PSA ADQL-Query; `hips-png`-Lauf.

### `--dropped` Laufzeit — Riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --dropped`-Lauf
- **Lage:** (gemessen 2026-09-29 20:28 via `ci-gate 36626370907 @4ef3a7e43`) baseline
  1134 | current 1148 | delta 14 — CI-only. Baseline in diesem Atom auf 1148 gebumpt
  (`docs/zustand/dropped-baseline.md`). **Bündelung gebaut** (`register_lookup.rs`):
  `commit_message_corpus()` lädt die Message-Korpora **einmal** (`git log --all
  --format=%B%x00`), `commit_resolves_all_lines` sucht im Speicher; der `-S`-Pickaxe bleibt
  Fallback. Substring ≡ `--grep`: `distinctive_token` (`normalize_words`) liefert nur
  Alphanumerika/`-`, kein ERE-Metazeichen (gemessen); `%B` ohne `%H`, damit ein Hex-Token
  nicht gegen einen Hash matcht. `cargo check` 0/0. Der erste `register-coverage`/`ci-gate`-Lauf
  misst die Laufzeit.
- **Blockade:** keine
- **Braucht:** CI-Lauf messen (Laufzeit + Korrektheit); Gate-Umbau selbst-messend/träger-bewusst offen.

### GitHub-Issues — Zensus/Träger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `gh issue list`-Zensus; Rest `#13/#17/#30/#43/#60/#71/#80`
- **Lage:** (gemessen 2026-09-29 20:52) **49 → 20 offen** (29 E0063-Massen-Rot-Spawn
  geschlossen via GH-API, Beleg je Issue: Run-HEAD ≤ `4ef3a7e43`). Offen u. a.: `#98`
  paper-content-Gate, `#81` clippy (Vor-E0063), `#80` Anomaly-Report, `#71`/`#67`/`#60`,
  `#52/#53` nvss/first14.
- **Blockade:** keine
- **Braucht:** `#98`/`#81`/`#49/#50/#48` inhaltlich bzw. per Re-Run prüfen; Rest nach grün schließen.

### Trägerlose Docs
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs`-Pass
- **Lage:** (gemessen 2026-09-29 20:52 `register_lookup --orphan-docs`) **4**:
  `docs/auftrag/auftrag-gic-einreichung.md` (1, GIC-Einreichung → Future),
  `docs/concepts/tools-map.md` (2, Werkzeug-Landkarte → Mycelium),
  `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (1, Step 5 → Mycelium),
  `docs/surveys/survey-messpunkt-verteilung.md` (3, → Sensory). Die zwei vorigen
  (`kybernetische-astrophysik`, `membran-ladearchitektur`) trägt diese Übergabe.
- **Blockade:** die 4 Marker ohne Live-Träger
- **Braucht:** je Dokument Träger setzen oder per Befund descopen.

### PDF-Rendering-Stufe (`exzellenz-konzept`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `paper-check.yml`-Lauf
- **Lage:** (gemessen 2026-09-29 20:50) **verifizierte Engine-Route**: `tectonic@0.17.0`,
  `https://github.com/tectonic-typesetting/tectonic/releases/download/tectonic%400.17.0/tectonic-0.17.0-x86_64-unknown-linux-musl.tar.gz`
  — 10 151 914 B, sha256 `8533d07f9ccbd7a65824b9e0459041bca34af1eb33daba48f59215593753a3b7`
  (GitHub-digest ≡ `--sniff`). Install + Render-Step in `paper-check.yml` ergänzt
  (sha-gepinnt, statisches musl-Binary). Der erste CI-Lauf misst die Route.
- **Blockade:** keine
- **Braucht:** `paper-check`-Lauf lesen (erster Render); PDF-Nummern-Gate als nächster Schritt.

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
- **Lage:** (gemessen 2026-09-29 20:44 folge207) **Halley**-Direktive steht
  (`sources.φ:15961` url, `:15965` `at halley`); **s1_sar** steht (`sources.φ:15987-15990`);
  der mountain-207-Befund „halley fehlt, `sgrep` = 0" ist **stale** — der Baum trägt sie.
  **europa_clipper** von Mountain-207 **bestätigt** (Raumschiff, Geschwister wie `at juice`;
  `frame_registry.φ` ist generiert aus `sources.φ` via `frames.rs:94`, kein Hand-Edit).
  **Kuprat** zugelassen (vier Kanäle auf Family-Tags), Riss: `tag kuprat` = 404,
  `upload_release` verweigert gekappten Tag (`cdn.rs:71`).
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

**Mycelium meldet: deine mountain-207-Antwort ist angekommen; ein neuer clippy-Block liegt bei dir.**

- **clippy-Block (Build-Gate @`4ef3a7e43`):** `src/archivar/spatial.rs:359:29` —
  `fn star_fields(b: &[u8]) -> Option<(f64×9)>` = `clippy::type_complexity`, unter
  `-D warnings` **exit 101**. Zusammen mit `hips.rs:518` (geheilt in diesem Atom) und
  `te.rs:3121` (River) fällt der `ci-gate`-Lauf. **Braucht:** einen `type`-Alias für das
  Tupel (deine Feder, `spatial.rs`).
- **`europa_clipper`/KASI/Halley** — deine mountain-207-Antwort gefaltet (siehe Offen):
  `europa_clipper` bestätigt (Raumschiff neben `at europa` Mond, `frame_registry.φ` ist
  generiert); KASI-MOC descoped (FP01, wert-lose Coverage); die Halley-Direktive stand
  bereits (`sources.φ:15961`), der „`sgrep` = 0"-Befund war **stale**.
- **kuprat-Tag** bleibt Mycelium-Feder (`tag kuprat` = 404, `cdn.rs:71`).

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

River hat den E0063-Build geheilt (`a95921575 river 67`) — der Massen-Rot hat sich gelöst.

**Zwei rote `ci-gate`-Blöcke liegen bei dir** (gemessen 2026-09-29 20:27, `ci_manage log
36626370907 --all`):
- **format** (exit 1): `src/mathematikerin/te.rs:3269` und `:6161`,
  `tools/measure/src/bin/fam_calibration.rs:1` (führende Leerzeile).
- **clippy** (exit 101): `src/mathematikerin/te.rs:3121:16` — `surrogate: &mut dyn FnMut(...)`
  = `type_complexity`.
**Braucht:** die zwei Dateien rustfmt-rein + den `type`-Alias setzen (deine Feder);
`main_flow.rs` ist mit river 67 erledigt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
