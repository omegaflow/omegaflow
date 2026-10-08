<!--
  title: Handover — Mycelium-Folge 200 (2026-09-28)
  session: Mycelium-Folge 200
  class: handover
  date: 2026-09-28
  sha256: ae2e3b8185dd8145f235aa290a0f2cb160023f6bfa3777aacef60932a8cc59bb
  status: live
-->
# Handover — Mycelium-Folge 200 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-28-mycelium-folge199.md` (→ `archiv/`).

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert).

## Operator-Wort-Register

- Wort | 2026-09-28 | „nein ich möchte dass erst ein echtes survey gemacht wird mit flash tauchern mit harten bandagen einen pro rätseln um zu prüfen was wir haben und was fehlt eine art tabelle" | Quelle: Mycelium-Session 200.
- Wort | 2026-09-28 | „schau mal auf den desktop und die mails da ist was für dich ankekommen" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „bitte mach erstmal eine archeologie wrum ich die daten überhaupt angefragt habe nach cdn veröffentlicht wird es natürlich nicht solange ich kein einverständnis habe" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „aber ich dachte das sei die 5te teiöcheneigenschaft doe wir für das rätsel gebraucht haben" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „aber wenn wir sie als sources registrieren dann ist ja der ursprung klar oder nicht was wäre denn sinnvoll und redlich?" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „ansonsten haben wir keine daten die uns ohne einverständnis gegeben wurden veröffentlicht , oder?" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „ja bitte" (Register-Arm + Audit) | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196 (weiterhin bindend — geteilter Baum).

## Offen (aufgeschlüsselt)

### Rätsel-Bestand (Survey) — Träger + Verdikt-Trägerschaft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die nächste Rätsel-Messung (`docs/surveys/survey-raetsel-bestand.md`).
- **Lage:** (gemessen 2026-09-28 folge200, 15 flash-Taucher + Hauptlauf-Gegenmessung) das stehende Survey `docs/surveys/survey-raetsel-bestand.md` steht (12 Nadeln + 2 Blätter + Kuprat, je Artefakt/Kanäle/fehlend/Holding/Verdikt/Schritt); der Vorläufer `survey-2026-09-28-raetsel-zensus.md` ist `consumed`. **Kopf-Befund:** das Verdikt von 10 der 15 Rätsel lebt nur als Paper-/Archiv-Prosa; `sgrep -ci dark_matter|dark_flow|corona_conditional|signal_cone|frb_blatt|kuprat|rixs|srd62|kugelblitz phi/` → je 0.
- **Blockade:** keine.
- **Braucht:** je Rätsel den im Survey genannten `Nächster Schritt` einzeln dispatchen; Ⅹ descope-Befund (registerlos) belegen oder auf `pending` setzen; Feder der Verdikt-Zeilen = Mountain. Der Split: Fehlkanal-Findung/Verdikt/Parser → Mountain (`## An mountain`); Rechen-/Probe-Code → River (`## An river`); Operator-Wort → Future (`## An future`).

### Rätsel-Fehlkanäle — CDN-Materialisation (Mycelium)
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountains Admission der Fehlkanäle (`## An mountain` im selben Handover).
- **Lage:** (gemessen 2026-09-28 folge200) die Rätsel-Kanäle sind registriert, aber wo ein neuer Producer/Verdikt von Mountain kommt, fehlt der Compiler → CDN-Weg.
- **Blockade:** Mountain-Admission.
- **Braucht:** je neuem Kanal Compiler + CDN-Workflow (Muster `nvss-cdn.yml`), dann `gh workflow run <kanal>-cdn.yml`.

### Sonden-Ephemeriden — 5 Läufe rot am historischen DAF-Idword, Fix im Atom
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `origin/main` trägt den Fix (`09ece9878`).
- **Lage:** (gemessen 2026-09-28 via `ci_manage status`/`log 36454286664`) die 5 `{galileo,cassini,rosetta,messenger,near}-ephemeris-cdn`-Läufe = `failure`; Fix `daf.rs:168` akzeptiert `NAIF/DAF` (`cargo check` 0/0).
- **Blockade:** keine.
- **Braucht:** `gh workflow run galileo-ephemeris-cdn.yml` (+ cassini/rosetta/messenger/near); 9 weitere Horizons-Flybys (`juno`/`juice`/`europa_clipper`/`voyager*`) registrieren; 10 übrige Sonden routen.

### spectral-cdn.yml — angelegt, Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `origin/main` trägt den Workflow (`09ece9878`).
- **Lage:** (gemessen 2026-09-28) `.github/workflows/spectral-cdn.yml` angelegt (Muster `nvss-cdn.yml`); `spectra.bin` unter `ncei.noaa.gov` = 404.
- **Blockade:** keine.
- **Braucht:** `gh workflow run spectral-cdn.yml`; Re-Manifest-Läufe `36451187881`/`36451192094`/`36451197406` prüfen; `twomass_psc`/`jwst_spectra` → Mountain-Admission.

### Kuprat-Kanäle — Register-Arm fehlt
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountains Admission der vier Kanäle / Operator-Wort zur Zeugenart.
- **Lage:** (gemessen 2026-09-28 folge200) RIXS spin (Zenodo 7286412), RIXS charge (15179114), EELS, SRD62 auf der CDN (206), **0 Register-Zeilen** (`sgrep kuprat|rixs|srd62 phi/` → 0). Tag-Drift bestätigt: `rixs_cuprate_probe.rs:5` etc. hardkodieren `ssd.jpl.nasa.gov`, Produzent Zenodo/NIST.
- **Blockade:** `format`/`field`/`witness kuprat` = Mountain-Feder (Mountain 199 `0f9bfdab3` hat die Substance-Zeugenart gebaut, aber `sgrep -i kuprat phi/witnesses.φ` → 0); `url`/`origin`/`compiler`/Tag = Mycelium — die fremde RIXS/witness-Arbeit ist mit `0f9bfdab3` committed, die Tag-Drift ist frei; Operator-Wort zur Zeugenart ausstehend.
- **Braucht:** Mycelium: `url`/`origin`/`compiler` + Tag-Drift heilen (jetzt möglich); Mountain: `witness kuprat` + `format`/`field`.

### Release-Namespace-Migration — `ssd.jpl.nasa.gov`-Legacy-Tag
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort / nächster CDN-Migrations-Atom.
- **Lage:** (gemessen 2026-09-28 via `archive_search --verdict` + `sgrep`) die Compiler laden bereits unter den Produzenten-Tag (`cosmicflows_compiler.rs:204` → `tapvizier.cds.unistra.fr`; `goes_xrs_compiler.rs:430` → `ncei.noaa.gov`), die Register-URLs sind korrekt — aber physisch liegt das Asset unter dem Legacy-Tag `ssd.jpl.nasa.gov` (`ssd.jpl` 206 vs. `ncei`/`tapvizier` 404); Probes/Workflows hardkodieren den Legacy-Tag (`dark_flow_probe.rs:64`, `bigbang_echo_probe.rs:65`, `solar_causal_graph_probe.rs:9-15`, `wso_cycle_probe.rs:8-10`, `goes-xrs-cdn.yml:26`); corpus-weit (u. a. `jwst_spectra`/`spectra.bin`/`dr3_stars`/`dastcom`; `cdn_reconcile.rs` kennt `ssd.jpl.nasa.gov-*`).
- **Blockade:** systemische Migration (CI-Re-Materialisation unter Produzenten-Tag + Probe-/Workflow-Tag-Flip), kein Spot-Fix.
- **Braucht:** pro Asset `gh workflow run <asset>-cdn.yml` unter dem Produzenten-Tag, danach Probe-Fetch + Workflow-Download flippen; Inventar zuerst: `sgrep -i ssd.jpl.nasa.gov tools/ .github/workflows/`.

### Sonden-Konten CNSA/NSSDC/ISRO/EMM — pending → sources
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Konten registriert (Operator-Hand 2026-09-28); Download end-to-end ungemessen.
- **Lage:** (gemessen 2026-09-28) `phi/blocked_sources.φ:373/377/381/385` = `pending`.
- **Blockade:** keine.
- **Braucht:** je Portal `archive_search --verdict <url>` end-to-end, dann `pending` → `phi/sources.φ`.

### ODF-Coverage der Flyby-Fenster
- **Status:** wartend | **Bindung:** eigen (mit sensory)
- **Trigger:** die `ephemeris_<sonde>`-Blöcke auf `origin/main`.
- **Lage:** (gemessen 2026-09-28) `galileo_odf`:9764 · `messenger_odf`:9810 · `cassini_odf`:8977 · `rosetta_odf`:8947; Record-Coverage gegen Flyby-Datum ungemessen.
- **Blockade:** keine.
- **Braucht:** je ODF-Asset Record-Coverage gegen das Flyby-Datum messen.

### Register-Kandidaten `index.φ` (8)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Port-Pass (`docs/SOURCE_PORT.md`).
- **Lage:** (gemessen 2026-09-28 via `sread phi/pipeline/index.φ`) 8 `verifiziert` (owner mycelium).
- **Blockade:** keine.
- **Braucht:** die 8 Inventare über `docs/SOURCE_PORT.md` portieren.

### `format vlde`-Quellzeile
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `vlies-density-cdn.yml`-Lauf / nächster Register-Pass.
- **Lage:** (gemessen 2026-09-28 folge200 via `sgrep -i vlde phi/sources.φ` = 0) kein `format vlde`-Block; Reader/Compiler/Asset stehen; Manifestations-Direktive = Mycelium (sensory-Zuweisung = Riss).
- **Blockade:** keine.
- **Braucht:** Block setzen (Owner-Wort liegt: mycelium).

### DAS2 Iowa + Occultation-DB UTFPR — Workflow nach Admission
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountains Admission (`phi/blocked_sources.φ:363-368`).
- **Lage:** (gemessen 2026-09-28) beide Arme gebaut (`b4106e69a`), kein `sources.φ`-Block, kein Workflow.
- **Blockade:** Admission-Verdikt.
- **Braucht:** nach Admission `format`/`tag`/`pattern` + Workflow.

### [redacted] Warte-Zeile schließen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** gefeuert — Mail-Eingang `state/mail/mail_ledger.φ:181`, Daten `data/lab_a.data/SAMPLE_NSE_[RETRACTED-SAMPLE]/`.
- **Lage:** (gemessen 2026-09-28) `state/zustand/wartend.φ:8` (Aufnehmer mycelium) wartete auf Mail-Eingang; `mail_ledger.φ:181` (SAMPLE_CONTACT/LAB_A), Daten `data/lab_a.data/SAMPLE_NSE_[RETRACTED-SAMPLE]/`.
- **Blockade:** keine.
- **Braucht:** die Warte-Zeile in `state/zustand/wartend.φ` schließen.

### Journal-Emitter — drei Rest-Haken (a)–(d)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** future-folge150-Block (`## An mycelium`) / `state/zustand/ereignisse.φ`-Arm.
- **Lage:** (gemessen 2026-09-28, future-folge150) `.opencode/plugin/act_recorder.ts` schreibt nach `state/zustand/ereignisse.φ`; offen: (a) `register_lookup --open` lernt den `ereignisse.φ`-Arm; (b) die Planungs-Pass-Regel als AGENTS.md-Zeile; (c) der Stehende Pass liest `ereignisse.φ`; (d) der Mess-Test.
- **Blockade:** keine.
- **Braucht:** (a)–(d) umsetzen; (b)/(-c) berühren AGENTS.md.

### Sources-Zeilen (PDS Chang'e-MRM, Zenodo/CDS/Shandong, PDA-Alternativen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** future-folge150-Block (`## An mycelium`) / nächster `docs/SOURCE_PORT.md`-Pass.
- **Lage:** (gemessen 2026-09-28, future-folge150) für PDS Chang'e-MRM-Bundle (200, kein Konto), Zenodo `records/15812343`, `alasky.cds.unistra.fr`, `pds.wh.sdu.edu.cn`, KASI `data.kasi.re.kr`, ShadowCam, KARI KPDS, NASA PDS KPLO, JAXA DARTS PDS4-Root (206), ESA PSA TAP (200) fehlen `sources.φ`-Zeilen.
- **Blockade:** keine.
- **Braucht:** je Endpunkt `archive_search --verdict <url>` + `sources.φ`-Zeile + Manifestation.

### DEMETER — CDPP-REGARDS server-seitig defekt (Order 18400)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CDPP-Reparatur / neuer Order-Versuch — `phi/blocked_sources.φ:89-91`.
- **Lage:** (gemessen 2026-09-28, sensory-201 + mountain-198) kein ip-Block; Gate = CDPP-Konto/`orderToken`; Generierung `availableFilesCount 0` (`18400` 1000 Fehler; `18387` 96 978/97 078), Grenze 12.–18.09.2026; Eintrag `phi/blocked_sources.φ:89-91` = `pending`.
- **Blockade:** quellenseitige Reparatur.
- **Braucht:** Dispositions-Klasse schärfen auf „CDPP-Generierung defekt — wartend"; Zeilen-Riss `:86`/`:88` klären.

### ci-check / ci-gate — Bestätigungslauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des Laufs auf dem HEAD.
- **Lage:** (gemessen 2026-09-28 folge199) `36454281654` pending; `register-coverage 36454281665` success.
- **Blockade:** keine.
- **Braucht:** `ci_manage status` beim nächsten Pass.

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster periodischer `hinet-cdn`-Lauf.
- **Lage:** (gemessen 2026-09-28 folge198) Job `hinet`: `cont status never read` (8×), exit 1.
- **Blockade:** quellenseitige Readiness (Hinet).
- **Braucht:** erneuter Lauf; kein Code-Schritt.

### ci_watchdog — Matcher
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster transienter Rot-Lauf.
- **Lage:** (gemessen 2026-09-28) Trigger nicht gefeuert.
- **Blockade:** keine.
- **Braucht:** der nächste Shutdown-Rot trägt „rerun … measured transient cause".

### D5-Orphan-Residuum — Röhren-Asset-CDN-Weg
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:383` §14.4 Punkt 4).
- **Lage:** (gemessen 2026-09-28) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02 / 2027-04-01
- **Trigger:** 2026-10-02 (übrige Routen, CSES) / 2026-12-02 (NOIRLab/Gaia-DR4) / 2027-04-01 (BepiColombo).
- **Lage:** (gemessen 2026-09-28) `pithia` backend-tot; `lasair` direct absent/proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

## An mountain

- **Rätsel-Fehlkanäle — Quellen finden + Admission + Parser** | (gemessen 2026-09-28 folge200) je fehlender Ader braucht es Producer + Verdikt + Parser/Compiler: Ⅳ **Swarm-TEC** (nur CHAMP registriert; Kandidat `vires.services` HAPI `SW_OPER_TEC…`), Ⅷ **tiefste z≳10-Samples** (keine Zeile), Ⅻ **B-Moden** (keine Quelle), Ⅺ **Placebo HRV/Blutmarker** (fehlt), ENSO **Becken-Windfeld/Scatterometer** (nur TAO-Zonal), GIC **co-lokales Mäntsälä-dB/dt** (FMI = Stunden-Peaks). Feder: `sources.φ`-Verdikt + Parser-Konstruktion. Karte: `docs/surveys/survey-raetsel-bestand.md`.
  Origin: mycelium-folge200
- **Rätsel-Verdikt-Zeilen (10)** | (gemessen 2026-09-28 folge200) das Verdikt von 10 der 15 Rätsel lebt nur als Paper-/Archiv-Prosa; `sgrep -ci dark_matter|dark_flow|corona_conditional|signal_cone|frb_blatt|kuprat|rixs|srd62|kugelblitz phi/` → je 0. Feder: Verdikt-Zeilen in `phi/`. Siehe `docs/surveys/survey-raetsel-bestand.md`.
  Origin: mycelium-folge200
- **Ⅹ Kugelblitz — descope registerlos** | (gemessen 2026-09-28 folge200) `descoped` steht nur als Prosa `docs/concepts/kybernetische-astrophysik.md:294-300` ohne Befund-Token; `sgrep -i kugelblitz phi/` → 0. Feder: Verdikt/Befund. Braucht: gemessenen Befund oder `pending`.
  Origin: mycelium-folge200
- **Kuprat — witness + format/field** | (gemessen 2026-09-28 folge200) die vier Kanäle (RIXS spin/charge, EELS, SRD62) gehören als `witness`-Zeilen nach `phi/witnesses.φ` (kein `ttl`/`field`); eine Kuprat-Zeugen-Art fehlt (Operator-Wort ausstehend). Mycelium heilt unabhängig die Tag-Drift.
  Origin: mycelium-folge199/200

## An river

- **Rätsel-Rechen-/Probe-Lücken (Code)** | (gemessen 2026-09-28 folge200) je Rätsel fehlt die Rechnung, nicht das Datum: Ⅰ **per-Voxel-Jeans-Engine** (`sgrep -i jeans tools/` → 0, kein Bin); Ⅶ **`max(0,|Δt|−d/c)`-Fold** nicht in `src/` (Probe `tools/measure/src/bin/signal_cone_audit_probe.rs` steht); Ⅸ **FRB-Paar** (`write_blatt_pair` steht `frb_blatt_probe.rs:295`, kein Blatt); ENSO **TE-Probe** (kein Bin; Muster `tools/measure/src/bin/nobel_probe_corona.rs`, Paar `src/mathematikerin/omega.rs:458 te_probe`). Nature: TE-Maschine/Probe-Code. Karte: `docs/surveys/survey-raetsel-bestand.md`.
  Origin: mycelium-folge200

## An future

- **NSE-Redistribution + Dank** | (gemessen 2026-09-28 folge199) Reply an SAMPLE_CONTACT (`state/mail/[redacted].md`) um die Lizenz-/Redistributionsfrage erweitern; der Akt ist Operator-Hand. Bis zum Einverständnis kein CDN.
  Origin: mycelium-folge199
- **ENSO-Zuschnitt** | (gemessen 2026-09-28 folge200) `state/zustand/wartend.φ:23` `blatt-zuschnitt | Operator`; der Operator setzt Blatt-Reihenfolge/-Zuschnitt. Braucht: Operator-Wort.
  Origin: mycelium-folge200
- **Kuprat-Zeugenart** | (gemessen 2026-09-28 folge200) eine Kuprat-Zeugen-Art fehlt (`phi/witnesses.φ` trägt nur `s2-direction`, `gestalt`). Braucht: Operator-Wort zur Zeugenart.
  Origin: mycelium-folge200
