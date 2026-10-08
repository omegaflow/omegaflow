<!--
  title: Handover — Mycelium-Folge 199 (2026-09-28)
  session: Mycelium-Folge 199
  class: handover
  date: 2026-09-28
  sha256: 1541d9eb4f8cf357246ea8cbb4c293ea97e9b6d4061fcaa905d97eb4bf1c2fef
  status: live
-->
# Handover — Mycelium-Folge 199 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-28-mycelium-folge198.md` (→ `archiv/`).

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert).

## Operator-Wort-Register

- Wort | 2026-09-28 | „schau mal auf den desktop und die mails da ist was für dich ankekommen" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „bitte mach erstmal eine archeologie wrum ich die daten überhaupt angefragt habe nach cdn veröffentlicht wird es natürlich nicht solange ich kein einverständnis habe" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „aber ich dachte das sei die 5te teiöcheneigenschaft doe wir für das rätsel gebraucht haben" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „aber wenn wir sie als sources registrieren dann ist ja der ursprung klar oder nicht was wäre denn sinnvoll und redlich?" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „ansonsten haben wir keine daten die uns ohne einverständnis gegeben wurden veröffentlicht , oder?" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „ja bitte" (Register-Arm + Audit) | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196 (weiterhin bindend — geteilter Baum).

## Offen (aufgeschlüsselt)

### Rätsel-Zensus (Survey) — Träger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die nächste Rätsel-Messung (`docs/surveys/survey-2026-09-28-raetsel-zensus.md`).
- **Lage:** (gemessen 2026-09-28) `docs/surveys/survey-2026-09-28-raetsel-zensus.md` angelegt (12 Nadeln + 3 Blätter + Kuprat, je Artefakt/vorhanden/fehlend/Verdikt).
- **Blockade:** keine.
- **Braucht:** je Rätsel die fehlende Ader einzeln nachmessen.

### Sonden-Ephemeriden — 5 Läufe rot am historischen DAF-Idword, Fix im Atom
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** dieser Commit + Push auf `origin/main`.
- **Lage:** (gemessen 2026-09-28 folge199 via `ci_manage status`/`log 36454286664`) die 5 `{galileo,cassini,rosetta,messenger,near}-ephemeris-cdn`-Läufe = `failure` (2 Runden, 10 Läufe). Grund: `spacecraft_ephemeris_compiler` → `unrecognized DAF identifier: [78,65,73,70,47,68,65,70]`; Sniff `s970311a.bsp` = idword `NAIF/DAF` (historisch), `de440s.bsp` = `DAF/SPK`. **Fix gebaut:** `src/archivar/bsp_reader/daf.rs:168` akzeptiert jetzt `NAIF/DAF` (+ Test `naif_daf_idword_is_accepted`, `cargo check` 0/0). `spectral-cdn.yml` analog.
- **Blockade:** keine.
- **Braucht:** nach Push: `gh workflow run galileo-ephemeris-cdn.yml` (+ cassini/rosetta/messenger/near); 9 weitere Horizons-Flybys (`juno`/`juice`/`europa_clipper`/`voyager*`) registrieren; 10 übrige Sonden routen.

### spectral-cdn.yml — angelegt, Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** dieser Commit + Push auf `origin/main`.
- **Lage:** (gemessen 2026-09-28 folge199) `spectral_compiler` → `spectra.bin` (Tag `ncei.noaa.gov`) hatte keinen Workflow; `.github/workflows/spectral-cdn.yml` angelegt (Muster `nvss-cdn.yml`). `spectra.bin` lag unter `ncei.noaa.gov` = 404 (ssd… 206).
- **Blockade:** keine.
- **Braucht:** nach Push `gh workflow run spectral-cdn.yml`; Re-Manifest-Läufe `36451187881`/`36451192094`/`36451197406` prüfen; `twomass_psc`/`jwst_spectra` → Mountain-Admission.

### Kuprat-Kanäle — Register-Arm fehlt (4 offene + NSE privat)
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountains Admission der vier Kanäle (Koordination `phi/sources.φ` / `phi/blocked_sources.φ`).
- **Lage:** (gemessen 2026-09-28 folge199) RIXS spin (Zenodo 7286412 → `rixs_spin.bin`), RIXS charge/plasmon (Zenodo 15179114 → `rixs_charge.bin`), EELS-Phononen (`crystal_compiler --eels` → `eels_acoustic.bin`), Eindringtiefe (`srd62_compiler` → `srd62_suprastrom.bin`) sind **auf der CDN, aber 0 Register-Zeilen** in `phi/` (`sgrep` 0). Kanal-Taxonomie `rixs_cuprate_probe.rs:7-11` trägt nur 3 Medien. Tag `ssd.jpl.nasa.gov` (Probes hartkodiert) = Drift zum Produzenten Zenodo/NIST.
- **Blockade:** `format`/`field` = Mountain-Feder; `url`/`origin`/`compiler`/Tag = Mycelium.
- **Braucht:** Mountain: `format`/`field`-Zeilen (4 Kanäle, `tag kuprat`); Mycelium: `url`/`origin`/`compiler` + Tag-Drift-Klärung. Siehe `## An mountain`.

### Sonden-Konten CNSA/NSSDC/ISRO/EMM — pending → sources
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Konten registriert/eingeloggt (Operator-Hand 2026-09-28, future-folge149); Download end-to-end ungemessen.
- **Lage:** (gemessen 2026-09-28) `phi/blocked_sources.φ:373/377/381/385` = `pending` (owner mycelium nach mountain `df0617c4c`): `https://moon.bao.ac.cn/` (Chang'e 1–6 GRAS) · `https://www.nssdc.ac.cn/` (Tianwen-1/Zhurong) · `https://pradan.issdc.gov.in/ch2` (Chandrayaan/MOM/Aditya-L1) · `https://sdc.emiratesmarsmission.ae/` (MBRSC/EMM).
- **Blockade:** keine.
- **Braucht:** je Portal den Download end-to-end messen (`archive_search --verdict <url>`), dann `pending` → `phi/sources.φ`.

### NSE I(q,t) — Redistribution-Einverständnis offen
- **Status:** wartend | **Bindung:** operator (SAMPLE_CONTACT-Akt)
- **Trigger:** SAMPLE_CONTACTs Antwort / Operator-Sende-Wort.
- **Lage:** (gemessen 2026-09-28 folge199) SAMPLE_CONTACT/LAB_A lieferte 13 NSE-I(q,t)-Läufe (q=(0.5,0.5,1.0), T 3.47–32.52 K), privat `data/lab_a.data/SAMPLE_NSE_[RETRACTED-SAMPLE]/` (sha256 `SOURCE_SHA…e6d864`, `PROVENANCE.txt`: NOT manifested). Reader `src/archivar/lab_reader.rs` + `lab_reader_compiler` gebaut. Reply-Entwurf `state/mail/[redacted].md`. Operator-Wort: kein CDN ohne Einverständnis. **Audit 2026-09-28:** keine weitere Dritt-Gabe im Register veröffentlicht (`phi/sources.φ` ohne private `origin`; `declined_sources.φ` ~24× `decline redistribution`).
- **Blockade:** Einverständnis zur Redistribution (Mensch, Operator-Hand).
- **Braucht:** Reply um die Lizenz-/Redistributionsfrage erweitern, dann Operator sendet; bis dahin privates Holding.

### ODF-Coverage der Flyby-Fenster
- **Status:** wartend | **Bindung:** eigen (mit sensory)
- **Trigger:** die `ephemeris_<sonde>`-Blöcke auf `origin/main` (dieser Commit).
- **Lage:** (gemessen 2026-09-28) `galileo_odf` `:9764`, `messenger_odf` `:9810`, `cassini_odf` `:8977`, `rosetta_odf` `:8947` existieren; Record-Coverage gegen Flyby-Datum ungemessen (Galileo Ⅰ 1990-12-08 / Ⅱ 1992-12-08 · Cassini 1999-08-18 · Rosetta 2005-03-04 · MESSENGER 2005-08-02).
- **Blockade:** keine.
- **Braucht:** je ODF-Asset Record-Coverage gegen das Flyby-Datum messen.

### Register-Kandidaten `index.φ` (8)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Port-Pass (`docs/SOURCE_PORT.md`).
- **Lage:** (gemessen 2026-09-28 via `sread phi/pipeline/index.φ`) 8 `verifiziert` (owner mycelium).
- **Blockade:** keine.
- **Braucht:** die 8 Inventare über `docs/SOURCE_PORT.md` portieren.

### LLNL_G3D_JPS/S40RTS `volume.bin` — Befund nicht baum-bestätigt
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** neue Messung an `phi/sources.φ:13813`.
- **Lage:** (gemessen 2026-09-28) die Zeilen tragen bereits die korrekte `origin procedure`; mountain-Befund nicht baum-bestätigt (Riss).
- **Blockade:** Mountain-Wort.
- **Braucht:** Wort, ob der Befund fällt.

### `format vlde`-Quellzeile
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `vlies-density-cdn.yml`-Lauf / nächster Register-Pass.
- **Lage:** (gemessen 2026-09-28 folge199 via `sgrep -i vlde phi/sources.φ` = 0) kein `format vlde`-Block; Reader/Compiler/Asset stehen. Riss Schreiber-Rolle: mountain-folge198 weist die Direktive mycelium zu.
- **Blockade:** Riss (Schreiber-Rolle).
- **Braucht:** Block setzen (Owner-Wort liegt: mycelium).

### DAS2 Iowa + Occultation-DB UTFPR — Workflow nach Admission
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountains Admission (`phi/blocked_sources.φ:363-368`).
- **Lage:** (gemessen 2026-09-28) beide Arme gebaut (`b4106e69a`), kein `sources.φ`-Block, kein Workflow.
- **Blockade:** Admission-Verdikt.
- **Braucht:** nach Admission `format`/`tag`/`pattern` + Workflow.

### ci-check — Bestätigungslauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des `ci-check`-Laufs auf dem HEAD.
- **Lage:** (gemessen 2026-09-28 folge199 via `ci_manage status`) `36454281654` pending; `register-coverage 36454281665` success.
- **Blockade:** keine.
- **Braucht:** `ci_manage status` beim nächsten Pass.

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster periodischer `hinet-cdn`-Lauf.
- **Lage:** (gemessen 2026-09-28 folge198 via `ci_manage log 36344350143`) Job `hinet`: `cont status never read` (8×), exit 1.
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
- **Lage:** (gemessen 2026-09-28 via `sgrep`) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02 / 2027-04-01
- **Trigger:** 2026-10-02 (übrige Routen) / 2026-12-02 (NOIRLab/Gaia-DR4) / 2027-04-01 (BepiColombo).
- **Lage:** (gemessen 2026-09-28) `pithia` backend-tot; `lasair` direct absent/proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

## An mountain

- **Kuprat-Kanäle — Admission (4)** | (gemessen 2026-09-28 folge199) RIXS spin (`rixs_spin.bin`, Zenodo 7286412, `cuprate-cdn.yml --rixs`), RIXS charge (`rixs_charge.bin`, Zenodo 15179114, `--plasmon`), EELS (`eels_acoustic.bin`, `crystal_compiler --eels`), Eindringtiefe (`srd62_suprastrom.bin`, `srd62-cdn.yml`). Alle auf der CDN, **0 Register-Zeilen**. Braucht: `format`/`field`/`ttl`-Zeilen (Mountain) + `tag kuprat`; Anlass = Kuprat-Mechanismus (Rätsel der Hochtemperatur-Supraleitung, `kybernetische-astrophysik.md:396-398`; 5. Teileigenschaft = NSE I(q,t)). Mycelium setzt `url`/`origin`/`compiler` danach.
  Origin: mycelium-folge199
- **DAF-Idword — gemeinsamer Archivar-Reader** | (gemessen 2026-09-28 folge199) `src/archivar/bsp_reader/daf.rs:168` akzeptiert jetzt das historische `NAIF/DAF`-Idword (`s970311a.bsp`, GLL-Cruise) neben `DAF/SPK`; `cargo check` 0/0, Test `naif_daf_idword_is_accepted`. Read — der Reader ist geteilter Archivar-Code.
  Origin: mycelium-folge199
- **Tag-Drift `ssd.jpl.nasa.gov`** | (gemessen 2026-09-28) die vier Kuprat-Probes/Workflows hartkodieren `…/releases/download/ssd.jpl.nasa.gov/{rixs_spin,srd62_suprastrom}.bin` — der JPL-SSD-Tag für Sonden-Kernel, nicht Zenodo/NIST. Vor der Kuprat-Registrierung klären (Register folgt dem Produzenten).
  Origin: mycelium-folge199

## An future

- **NSE-Redistribution + Dank** | (gemessen 2026-09-28 folge199) Reply an SAMPLE_CONTACT (`state/mail/[redacted].md`) um die Lizenz-/Redistributionsfrage erweitern; der Akt ist Operator-Hand. Bis zum Einverständnis kein CDN (Operator-Wort 2026-09-28).
  Origin: mycelium-folge199

## Abschluss

Commit-Wort des Operators (`/commit`) liegt vor. Die fremden Sessions haben committet
(`df0617c4c` mountain 198, `2000145f7` sensory 201, `b241de23f` river 60); mein Gate-Arm
(`unbacked_mirror_violations` + Test `fp_unbacked_mirror_requires_basis` + Vocab-Key) ist in
`df0617c4c` mitgelaufen. Eigene Pfade dieses Atoms: `src/archivar/bsp_reader/daf.rs`,
`.github/workflows/spectral-cdn.yml`, `docs/surveys/survey-2026-09-28-raetsel-zensus.md`,
dieses Handover, die Archiv-Verschiebung folge198.
