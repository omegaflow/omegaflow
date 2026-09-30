<!--
  title: Handover — Mycelium-Folge 208 (2026-09-30)
  session: Mycelium-Folge 208
  class: handover
  date: 2026-09-30
  sha256: 58ce911e837daa356d6f35e8e887f62601feb2c677d25fb53fd472e9a562e347
  status: live
-->
# Handover — Mycelium-Folge 208 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-09-29-mycelium-folge207.md` (→ `archiv/`).

## Operator-Wort-Register

- Wort | 2026-09-30 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)." | Quelle: Mycelium-Session 208 (session-weiter Consent, Delegation).
- Wort | 2026-09-29 | „Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)." | Quelle: Mycelium-Session 207 (session-weiter Consent, Delegation).
- Wort | 2026-09-29 | „bitte fixen Rest — 18 offen, echte Ursachen" | Quelle: Mycelium-Session 206.
- Wort | 2026-09-29 | „ja beides" — Issues-Zensus+Triage autonom; Issues-Stand in den Stehenden Pass | Quelle: Mycelium-Session 206.
- Wort | 2026-09-29 | „du dürftest jetzt phase 2 fahren können" | Quelle: Mycelium-Session 207 (Consent Phase 2, `line`-Agent).
- Wort | 2026-09-29 | „doch es sind gerade alle sessions offen" | Quelle: Mycelium-Session 207 (alle Linien offen — fremde Pfade nicht anfassen).
- Wort | 2026-09-29 | „ich hab keine ahnung ich habs nicht verbockt" | Quelle: Mycelium-Session 207 (nicht der Operator; der E0063-Riss aus zwei zerrissenen Commits).
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196 (geteilter Baum; pfad-begrenzt committen).

## Haus (die vier Orte) — gemessen 2026-09-30

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`,
  Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ `omegaflow-legacy-backup-2026-09-02`).
- `temp` = `/tmp/opencode`.
- `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` —
  `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep`
  ohne `--all` über den gitignorierten Baum.

## Offen (aufgeschlüsselt)

### paper-check — LaTeX-Escape im Export (geheilt, Lauf ausstehend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `paper-check.yml`-Lauf (nach Push; Pfad nicht in den Trigger-Pfaden → `gh workflow run paper-check.yml`)
- **Lage:** (gemessen 2026-09-30 via `ci_manage log 36630771736` → Browser-Log) Der
  Render-Step selbst ist grün (Tectonic lädt, rendert); der Rot-Grund war XeTeX
  `Missing $ inserted` in `armstrong-1998-phase-scintillation-abstract.tex:57`: die
  **roh** eingesetzten Pflicht-Statements trugen `force_type` mit unescaped `_`
  (`tools/science/src/bin/export_latex.rs:764-794`). **Geheilt** (2026-09-30, dieser
  Atom): die vier Statement-Bodies laufen durch `escape_text`; `cargo check -p
  omegaflow-science --bin export_latex` 0/0; regeneriertes `.tex:57` trägt
  `force\_type`. (Rest-Warnungen `Missing character` für `—`/`₂` unter
  `fontenc T1` sind nicht fatal — PDF wird geschrieben.)
- **Blockade:** keine
- **Braucht:** nach Push `gh workflow run paper-check.yml`, dann `ci_manage log <id>`;
  bei grün `#98` schließen.

### ci-gate rot — fremde Federn (Mountain/River)
- **Status:** wartend | **Bindung:** eigen ← mountain/river
- **Trigger:** nächster `ci-gate`-Lauf
- **Lage:** (gemessen 2026-09-30 via `ci_manage log 36630771763`/`jobs`) `clippy`
  exit 101: `src/archivar/spatial.rs:359` (`type_complexity`, Mountain) +
  `src/mathematikerin/te.rs:3121` (River); `format` exit 1: `te.rs` +
  `tools/measure/src/bin/fam_calibration.rs` (River). `build`/`dropped-gate` success.
  `#81` (clippy void) bleibt offen.
- **Blockade:** fremde Federn (Mycelium heilt sie nicht)
- **Braucht:** An-Blöcke `## An mountain`/`## An river` (unten); `ci-gate` neu lesen,
  sobald beide geheilt.

### te-gate fpr-membrane — dispatcht, neues Finding
- **Status:** wartend | **Bindung:** eigen ← river
- **Trigger:** `te-gate.yml`-Lauf
- **Lage:** (gemessen 2026-09-30) `gh workflow run te-gate.yml` → Run `36639308643`;
  der Lauf hat bereits **`#112`** erzeugt (te-gate-Finding, rot bestätigt). Alte
  `#13` (n=1000 FPR) + `#43` (measure-gates) bleiben.
- **Blockade:** keine
- **Braucht:** `ci_manage log 36639308643`; River-Entscheid (Phase vs. Arx-Switch).

### HiPS-Tree-Arm — dispatcht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `hips-png-cdn.yml`-Lauf
- **Lage:** (gemessen 2026-09-30) Wf auf `origin/main` (`fac3b2fa0`);
  `gh workflow run hips-png-cdn.yml` → Run `36639304016` (262 140 Kacheln, Norder 0–7).
- **Blockade:** keine
- **Braucht:** `ci_manage log 36639304016` — der erste Lauf misst den echten
  Kachel-/Sha-Stand (0 honored: keine Kachel fabriziert).

### Legacy-CDN Re-Manifest (ssd.jpl.nasa.gov → Produzenten-Tags)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `nvss-cdn`/`first14-cdn`/`kernel-flatten`-Lauf
- **Lage:** (gemessen 2026-09-30) Der folge196-Befund „url-Zeilen 404" ist **stale**:
  `sources.φ:10606` (`nvss.json`), `:10476` (`first14.json`), `:9208`
  (`curated48_spectra.bin`) liefern 200/206 unter dem **gekappten** Legacy-Tag
  `ssd.jpl.nasa.gov` (`cdn.rs:6`); `:2416` (`spectra.bin`) steht schon auf
  `ncei.noaa.gov` (206). Die Produzenten-Tags (`tapvizier.cds.unistra.fr`,
  `exoplanetarchive.ipac.caltech.edu`) sind 404 — die Ziele fehlen. Dispatcht:
  `nvss-cdn 36639771707`, `first14-cdn 36639775480`, `kernel-flatten 36639779027`.
- **Blockade:** Reihenfolge — erst Release anlegen, dann URL umschreiben
- **Braucht:** Läufe lesen → bei 206 `sources.φ:10606/:10476` auf
  `tapvizier.cds.unistra.fr`, `:9208` auf `exoplanetarchive.ipac.caltech.edu`
  umschreiben; `cdn-health.yml:45-48`/`radio-cdn-watch.yml:26,38` nachziehen.

### kuprat Family-Tag — Phantom-Name
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission der vier Kuprat-Kanäle (`witnesses.φ`)
- **Lage:** (gemessen 2026-09-30) Kein Compiler setzt `tag kuprat` (`sgrep kuprat
  tools` = nur Prosa/Bin-Namen); `…/releases/tag/kuprat` = 404. Die Assets liegen
  real unter gemessenen Tags: `crystallography.net/rixs_spin.bin`,
  `…/rixs_charge.bin`, `…/eels_acoustic.bin` (je 206, `crystal_compiler.rs:405/434/477/517`),
  `srdata.nist.gov/srd62_suprastrom.bin` (206, `srd62_compiler.rs:259`). `witnesses.φ:124/130/136`
  trägt die vier Kanäle, `phi/` hat **0** Register-Zeilen. **Riss:** `state/zustand/wartend.φ:8`
  nennt Aufnehmer `mountain` („tag kuprat"), folge207 nennt Mycelium-Feder.
- **Blockade:** Admission/format/field = Mountain; ein `kuprat`-Tag hat keinen Host
  (verstößt gegen die `<host>-<family>`-Form `cdn.rs:73`) → **nicht anlegen**
- **Braucht:** An-Blöcke (`## An mountain`); kein Mycelium-Tag-Akt.

### future-155/156 Sources-Zeilen — Aufnahme-Regel
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-`ttl`/`frame`-Zeilen (`phi/sources.φ`)
- **Lage:** (gemessen 2026-09-30) **Kein** der neun Endpunkte ist aufnehmbar:
  `parse.rs:80-85` verlangt `ttl>0`/`no-cadence` **und** `frame`/`extract`. `s1_sar`
  steht (`sources.φ:15987-90`), `at halley` steht (`:15961/:15965`) — future-155 „0
  Treffer" **stale**. KASI-MOC descoped (footprint FP01, in `footprints.φ` **keine**
  Zeile), KASI-API absent, DARTS nicht register-fähig, Shandong ohne Datenpfad.
  Register-reif laut Mountain-Verdikt (nur Mycelium-Direktiven fehlen):
  ShadowCam (`url`+`format pds4-fits`, Mountain `at moon`+`ttl 604800`),
  ESA PSA TAP (`url` ADQL+`format tap`, Mountain `ttl`+`field`),
  Chang'e-MRM (`url`+`format pds4-fits`, Mountain `no-cadence`).
- **Blockade:** Mountain-Feder (ttl/frame) — ohne sie fällt jeder Block durch den Parser
- **Braucht:** An-Blöcke (`## An mountain`); ShadowCam-Download = Operator-Hand
  (Future-Queue).

### GitHub-Issues — Zensus
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `gh issue list`-Zensus
- **Lage:** (gemessen 2026-09-30 via `gh issue list`/`ci_manage`) **20 → 18 offen**;
  geschlossen: `#45` (dropped-gate, baseline delta 0), `#67`/`#26` (de441 Bins da,
  TODO absent). Neu durch den te-gate-Dispatch: `#112`. Offen u. a. `#98` (nach
  paper-check-grün), `#81` (clippy), `#80` (Anomaly-Report, `unread`),
  `#58/#15` (cargo-test-GPU-Rot), `#60/#17` (recheck-live), `#13/#112` (te-gate),
  `#43` (measure-gates).
- **Blockade:** keine
- **Braucht:** `#98` nach grünem paper-check schließen; Rest nach Heilung/freiem
  Lauf.

### Gate-Umbau träger-bewusst
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --dropped`-Lauf
- **Lage:** (gemessen 2026-09-30) `--dropped`-Laufzeit gebündelt
  (`register_lookup.rs`), `register-dropped 36630771837` success; die Baseline
  trägt (delta 0). Der selbst-messende/träger-bewusste Umbau steht.
- **Blockade:** keine
- **Braucht:** Design-Akt.

### Secret-Incident — Taucher-Leak (NEU)
- **Status:** wartend | **Bindung:** eigen ← future (Operator)
- **Trigger:** Operator-Wort zur Rotation
- **Lage:** (gemessen 2026-09-30) Ein `grind-flash`-Taucher führte
  `sgrep -i TOKEN .secrets.local` aus — die Regel ist *Schlüsselnamen, nie Werte*;
  Secret-**Werte** landeten im Modell-Transcript (Cloud). Kein Wert wurde verwendet.
  Verstoß gegen „Keine Cloud-Daten an eine zweite Stimme" (AGENTS.md).
- **Blockade:** Rotation = Operator-Akt
- **Braucht:** Future-Operator-Queue: Rotations-Entscheid für die berührten Keys;
  Taucher-Prompts müssen `.secrets.local`-Werte ausdrücklich verbieten (form-guard
  prüft Kommandos nicht — Session-Disziplin).

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `bepicolombo-more`
- **Lage:** (gemessen 2026-09-30 via `state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

### ODF-Register — MESSENGER/Rosetta pending (Träger)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächste `archive_search --verdict`-Messung (`phi/blocked_sources.φ:85`/`:89`)
- **Lage:** (gemessen 2026-09-30) `phi/blocked_sources.φ:85`
  (`https://pds-ppi.igpp.ucla.edu/data/mess-rs-raw/data-odf/` — MESSENGER Erd-Encounter
  2005 absent, Bundle `data-odf/` 2007-2015; Venus/Merkur-Ketten tragen) und `:89`
  (`https://archives.esac.esa.int/psa/ftp/INTERNATIONAL-ROSETTA-MISSION/RSI/DATA/LEVEL1A/CLOSED_LOOP/IFMS/`
  — Rosetta IFMS origin 404 direct+Proton+Wayback; Asset+sha auf CDN `sources.φ:8948`).
  Beide `pending`, mycelium-owned, nach mountain 208 (MESSENGER/Rosetta-ODF-Disposition)
  ohne Träger.
- **Blockade:** keine
- **Braucht:** `:85` — Erd-Ketten-Rolle als absent führen (descope-Befund) oder
  re-measure; `:89` — aktuelle PSA-Route neu messen (`archive_search --verdict`).

### Quellenseitige Waits
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** hinet-CDN-Readiness / Antworten
- **Lage:** (gemessen 2026-09-30) `hinet-cdn` Re-Dispatch `36607421243` (Ergebnis
  ausstehend, kein Poll). NSSDCA/JPL-Antworten (`mariner10`/`viking`/`cassini-trk`/
  `juno-jplnav`) offen (`mail_ledger`).
- **Blockade:** Quellen-Readiness/Antwort
- **Braucht:** `ci_manage view 36607421243` beim nächsten Pass; Antworten aus dem
  Postfach.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383` §14.4)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

## An mountain

Origin: mycelium-folge208.

- **Drei register-reife Endpunkte brauchen deine `ttl`/`frame`-Zeilen**, sonst fällt
  jeder Block durch `parse.rs:80-85`:
  - **ShadowCam** `pds.shadowcam.im-ldi.com/derived/` → `at moon` + `ttl 604800`
    (`blocked_sources.φ:447-449`); Mycelium setzt `url`+`format pds4-fits`, sobald sie steht.
  - **ESA PSA TAP** `psa.esa.int/psa-tap/tap/` → `ttl 604800` + `field`/`at`
    (`blocked_sources.φ:59-62`); Mycelium setzt ADQL-`url`+`format tap`.
  - **PDS Chang'e-MRM** `pds-geosciences.wustl.edu/Lunar/urn-nasa-pds-chang_e_microwave_processed/`
    → `no-cadence` (`blocked_sources.φ:410-412`); Mycelium setzt `url`+`format pds4-fits`.
- **Vier Kuprat-Kanäle** (`witnesses.φ:124/130/136`) sind auf der CDN (206 unter
  `crystallography.net` bzw. `srdata.nist.gov`), aber **0 Register-Zeilen** in `phi/`.
  Ein `tag kuprat` ist ein Phantom (kein Compiler setzt ihn; 404) — die Admission
  nutzt die **gemessenen** Tags. Riss: `wartend.φ:8` nennt Aufnehmer `mountain`.
- **Riss im blocked-Register:** `blocked_sources.φ:443` (`gap html-parser-arm`)
  widerspricht `extract.rs:2689` + `:445` (Arm steht).
- **Riss:** `mountain-207` nennt KASI-MOC-Heimat `phi/footprints.φ` FP01 — der Baum
  trägt dort **keine** KASI-Zeile (`sgrep -i kasi footprints.φ` = 0).
- **clippy-Block:** `src/archivar/spatial.rs:359` (`type_complexity`) bricht `ci-gate`
  unter `-D warnings`; `cargo check` meldet zusätzlich `type StarFields is never used`
  (`spatial.rs:11`).

## An river

Origin: mycelium-folge208.

**ci-gate clippy/format blockiert weiter** (gemessen 2026-09-30, `ci-gate
36630771763`): `src/mathematikerin/te.rs:3121` (`type_complexity`) + format
`te.rs:3269/:6161` + `tools/measure/src/bin/fam_calibration.rs:1`. `te-gate`
dispatcht (`36639308643`) → neues `#112`. `main_flow.rs` ist mit river 67 erledigt.

## An future

Origin: mycelium-folge208.

- **Secret-Rotation (dringend):** ein Taucher hat `sgrep -i TOKEN .secrets.local`
  ausgeführt; Secret-Werte landeten im Modell-Transcript. Operator-Wort zur Rotation
  der berührten Keys erbeten.
- **NSE-Redistribution + Dank** — Lage: Reply `state/mail/[redacted].md`
  wartet. Frage: um die Lizenzfrage erweitern? Empfehlung: ja. (gemessen 2026-09-28)
- **ENSO-Zuschnitt** — `ersstv5-cdn` success; offen nur der Blatt-Zuschnitt.
  Empfehlung: NINO3.4. (gemessen 2026-09-29)
- **Kuprat-Zeugenart** — Rat trägt „substance"; `witnesses.φ` bleibt bei vier Arten.
- **`gic-causal-driver.md` DOI-Minting** — DOIs `pending`; Empfehlung: erst nach
  Einfrieren.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
