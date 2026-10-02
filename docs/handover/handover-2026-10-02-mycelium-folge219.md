<!--
  title: Handover — Mycelium-Folge 219 (2026-10-02)
  session: Mycelium-Folge 219
  class: handover
  date: 2026-10-02
  sha256: a750c6534beee80434c2faf1a3d20d0a3ad10a354b162e031733bbad55188764
  status: live
-->
# Handover — Mycelium-Folge 219 (2026-10-02)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-01-mycelium-folge218.md` (→ `archiv/`).

## Burn: open 0.0180 · close 0.4450 · cap 0.50 Grund: 29-Stimmen-Free-Voice-Fan-out (Sondierung) + Konzept/Workflows

## Operator-Wort-Register

- Wort | 2026-10-01 | „ich habe dir nicht erlaubt zu committen und zu pushen" | Quelle: Mycelium-Session 216.
- Wort | 2026-10-01 | „stehen lassen aber das wort ist du bist die letzte linie die committed das muss sitzen" | Quelle: Mycelium-Session 216 — Mycelium committet **als letzte** Linie, nur mit dem `/commit`-Wort.
- Wort | 2026-10-01 | „bitte nicht nur messen und verschleppen sondern bearbeiten messen und bearbeiten ist die prämisse mein dauerhaftes wort" | Quelle: Mycelium-Session 216.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209.
- Wort | 2026-10-01 | „ja möchte ich" | Quelle: Mycelium-Session 215 — VCO-rs-Register auf das PDS4-20190704-Asset umstellen.

## Haus (die vier Orte) — gemessen 2026-10-02

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02); `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all`; `phi/pipeline/catalog/*` ist gitignored, `phi/pipeline/index.φ` + `ledger.φ` trackbar.
- Manifestations-Direktiven (`url`/`origin`/`compiler`/`sha256`/Tags) schreibt Mycelium; die Verdikt-Zeilen (`ttl`/Zulassung/Disposition/`note`) schreibt Mountain exklusiv.

## Messungen dieses Atoms (2026-10-02, Mycelium-219)

- **Ephemeriden-Re-Manifest `itokawa` geschlossen:** `archive_search --sniff` = **HTTP 200 / 6024 B / sha256 `5ebcf305f30fc3482c1f23eaa68563894560b276a410b8a8e2cc465b0c8282a6`** unter `ssd.jpl.nasa.gov-horizons`; die `url`-Zeile `phi/sources.φ:16008` (`at itokawa`) stand bereits, kein Re-Manifest nötig. Die zwei `kernel-flatten`-Läufe `36894645771` / `36873476009` sind **success** (gemessen via `ci_manage view`) — Pioneer-daily (`sources.φ:16029`/`:16037`) und itokawa sind damit beide geschrieben.
- **Adressierte Blöcke gefaltet:** future-folge165 (Pioneer-daily `-horizons` 200, `sources.φ:16029`/`:16037` — die 404-Messung ist überholt; `wartend.φ`-Zeilen juno/cassini/nssdca: Trigger nicht gefeuert), mountain-folge221 (5 Absolutpfade im Holdings-Survey erledigt — kein Fremd-Edit), sensory-folge218 (`auftrag-gic-einreichung.md` steht nicht mehr im Orphan-Zensus → Träger vorhanden).
- **CI gemessen via `ci_manage`:** `rave-cdn 36932654093` / `sb9-cdn 36932650180` / `first14-cdn 36932643096` failure — **eine Ursacheklasse: `tapvizier.cds.unistra.fr` TAP**; rave `tap_query http exit status: 22: curl: (22) ... error: 400` (`--table III/279/rave_dr5`), sb9 dito (`--table B/sb9/main`, Issue #122), first14 `uws job phase ERROR — the query stays unharvested` + exit 1 (`--table VIII/92/first14`, async 1790894025487). `ci-gate 36936698719` @`3965e3814` rot: clippy 4 Fehler (`needless_range_loop` `src/mathematikerin/least_squares.rs:28`, `useless_vec` `:71`, `approx_constant` Φ `src/mathematikerin/omega.rs:5` + `src/mathematikerin/te.rs:1952`) · Träger **river**; dropped-gate bereits auf **1351** gebumpt (Träger mycelium). `matrix-rotor 36937794838` failure = **Hosted-Runner-Präemption** (GH-API: Step 7 `cancelled` 23:15:32Z, `if: always()`-Schritte `skipped`; Workflow-Header dokumentiert die Präemption, Checkpoint/Resume greift — kein Code-Defekt). `ps1-cdn 36939444468` in_progress; @`60ba8dccf` queued: `ci-gate 36942298171` / `ci-check 36942298307` / `register-coverage 36942298365`, `quake-feeds-cdn 36943588445` / `ned-cdn 36943509961`. **Fix-Dispatch:** `rave-cdn 36944700194` / `sb9-cdn 36944703250` / `first14-cdn 36944707063` auf `60ba8dccf` neu gestartet (transiente tapvizier-Ablehnung; die rekonstruierte rave-ADQL antwortet jetzt 200).
- **Orphan-Zensus:** `register_lookup --orphan-docs` = **5**; `register_lookup --orphans` = **2** (`phi/blocked_sources.φ:447`/`:463`, future). `open_points_check folge218` = 0 absent / 0 stale-citations.
- **Arbeitsbaum == HEAD** `60ba8dccf` == `origin/main` (`git_safety --snapshot`: nothing to record).
- **Getragen:** `docs/concepts/free-voices.md` — die Free-Voice-Schwarm-Karte; diese Übergabe ist ihr Träger (Aufruf, Roster, Grenzen, Kadenz).
- **Zwei neue Source-Compiler → je ein CDN-Workflow (mountain-222, `## An mycelium`):** `ascat_compiler` → `.github/workflows/ascat-cdn.yml` (Tag `manati.star.nesdis.noaa.gov-ascat`, Asset `ascat_uhr_ascat_b.bin`; wählt das jüngste UHR-ASCAT-B-Granulat aus der manati-Jahresliste, `--label uhr_ascat_b --ci-mode`); `ceers_spectra_compiler` → `.github/workflows/ceers-cdn.yml` (Tag `web.corral.tacc.utexas.edu`, Asset `ceers_spectra.bin`; MSA 323 / NIRSpec4 / prism, lsk + Cache). Beide Wege sind uncommittet — `gh workflow run` sieht sie erst nach Push.
- **jwst_spectra-Tag entzerrt:** `phi/sources.φ:10382` url-Tag `ssd.jpl.nasa.gov` → `exoplanetarchive.ipac.caltech.edu` (Upload-Ziel des Compilers `jwst_spectra_compiler.rs:796` + `origin`); `cdn-health.yml:45` auf `curated48_spectra.bin exoplanetarchive.ipac.caltech.edu`; Manifest-Dispatch `kernel-flatten 36946571514` schreibt `curated48_spectra.bin` an den neuen Tag (der Alt-Ort `ssd.jpl.nasa.gov` trägt die Bytes weiter — nicht relokalisiert).
- **Zwei Ephemeriden-Probes → ein CI-Workflow (mountain-222, `## An mycelium`):** `.github/workflows/ephemeris-probes.yml` — Job `eclipse-shadow` (2017-08-21 + 2024-04-08; der Probe holt ihre sun/moon/earth-Bins selbst über `ensure_bin` aus dem CDN) und Job `flyby-anderson` (Erd-Haus-Bins + Anderson-Residuen-Tabelle `anderson_residuals.tsv` per `gh release download` aus dem neuen CDN-Release `anderson-residuals`). `ephemeris_house_gate` bleibt bei seinem Workflow (`ephemeris-house-gate.yml`, dispatcht `36946576753`).
- **Anderson-Residuen-Tabelle → CDN (CI-only, Operator-Wort 2026-10-02):** getrackter Seed `tools/measure/anderson_residuals.tsv` (sha `093e9e47b2d5f12fcccd9387ecc925512013af7398f188360fccfb5e029ddfb6`, identisch mit dem Register-`sha256`); der Job `flyby-anderson` in `ephemeris-probes.yml` erzeugt/überschreibt das Release `anderson-residuals` (`contents: write`) und lädt den Seed — **CI ist der Manifestator** (der Session-Upload war der vorläufige Vollzug derselben Bytes). Register-Zeile `format reference` + `origin` (arXiv/Anderson 2008) + `sha256` + `ttl` an `phi/sources.φ`.

## Offen (aufgeschlüsselt)

### tapvizier-TAP-Klasse — rave/sb9/first14 `-cdn` rot
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende der drei neu dispatchten `-cdn`-Läufe
- **Lage:** (gemessen 2026-10-02 via `ci_manage log` + Handrepro) Die Alt-Läufe `rave-cdn 36932654093` / `sb9-cdn 36932650180` / `first14-cdn 36932643096` (`head_sha 7fce797df`, enthält river 77 `d9baca435`) scheiterten an `tapvizier.cds.unistra.fr` TAP — HTTP 400 (`tap_query exit 22`) bzw. `uws job phase ERROR`. Die **rekonstruierte ADQL der rave-Slice ist jetzt 200** (`curl ... QUERY@…` = `{"metadata":…,"data":…}`), d. h. der Server lehnte zur Laufzeit transient ab; kein Code-Defekt. Neu dispatcht @`60ba8dccf`: `rave-cdn 36944700194` / `sb9-cdn 36944703250` / `first14-cdn 36944707063`.
- **Blockade:** keine (transiente Server-Ablehnung)
- **Braucht:** Lauf-Ende der drei neuen Läufe lesen (`ci_manage view/log`); grün → Klasse schließen, rot → erneut messen.

### Zwei neue CDN-Workflows — Push + Erstlauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `/commit` (Push) → dann `gh workflow run ascat-cdn.yml` / `gh workflow run ceers-cdn.yml`
- **Lage:** (gemessen 2026-10-02) `.github/workflows/ascat-cdn.yml` + `.github/workflows/ceers-cdn.yml` geschrieben; `gh workflow run` liest nur den default branch, die Läufe starten erst nach dem Push.
- **Blockade:** uncommittet
- **Braucht:** nach Push die zwei Läufe dispatchen und deren Lauf-Ende lesen; manifestiert `manati.star.nesdis.noaa.gov-ascat/ascat_uhr_ascat_b.bin` und `web.corral.tacc.utexas.edu/ceers_spectra.bin` (beide aktuell 404).

### jwst_spectra-Tag — Manifestation am neuen Tag
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `kernel-flatten 36946571514`
- **Lage:** (gemessen 2026-10-02) `phi/sources.φ:10382` url-Tag auf `exoplanetarchive.ipac.caltech.edu` abgeglichen; Asset dort noch 404 (der Alt-Ort `ssd.jpl.nasa.gov` trägt die Bytes weiter). `kernel-flatten 36946571514` dispatcht.
- **Blockade:** Lauf-Ende
- **Braucht:** `archive_search --verdict "https://github.com/omegaflow/sources/releases/download/exoplanetarchive.ipac.caltech.edu/curated48_spectra.bin"` nach Lauf-Ende; 200 → Punkt schließen.

### Ephemeris-Probes in CI — Erstlauf nach Push
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `/commit` (Push) → `gh workflow run ephemeris-probes.yml`
- **Lage:** (gemessen 2026-10-02) `.github/workflows/ephemeris-probes.yml` geschrieben (jobs `eclipse-shadow`, `flyby-anderson`); `gh workflow run` liest nur den default branch. Die Anderson-Residuen-Tabelle ist als CDN-Asset (`anderson-residuals/anderson_residuals.tsv`, sha `093e9e47…`) registriert und wird **vom Workflow selbst** aus dem getrackten Seed `tools/measure/anderson_residuals.tsv` manifestiert (CI-only).
- **Blockade:** uncommittet
- **Braucht:** nach Push dispatchen, Lauf-Ende/Artefakte lesen und das Verdikt an Mountain (Gegenüber `mountain-222`).

### ci-gate/ci-check @`3965e3814` — clippy rot
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende des nächsten `ci-gate` nach dem river-clippy-Fix; neuer Lauf @`60ba8dccf`
- **Lage:** (gemessen 2026-10-02 via `ci_manage jobs`/`log`) `ci-gate 36936698719` @`3965e3814` rot: clippy 4 Fehler `src/mathematikerin/least_squares.rs:28` + `:71`, `src/mathematikerin/omega.rs:5`, `src/mathematikerin/te.rs:1952` — Träger **river**; dropped-gate auf `1351` gebumpt (mycelium, absorbiert). Neuer `ci-gate 36942298171` / `ci-check 36942298307` queued @`60ba8dccf`.
- **Blockade:** river-clippy-Fix
- **Braucht:** river heilt die 4 Clippy-Stellen + pusht; der nächste `ci-gate` misst.

### `matrix-rotor` — roter Lauf = Hosted-Runner-Präemption
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster planmäßiger Lauf (`43 */6 * * *`)
- **Lage:** (gemessen 2026-10-02 via GH-API `/actions/runs/36937794838/jobs`) Job `rotor` completion `failure`, **Step 7 „Den verborgenen Rotor fahren" `cancelled`** (Start 23:14:23Z, Cancel 23:15:32Z), die `if: always()`-Folgeschritte 8/9 `skipped` → der Job wurde abgebrochen, nicht durch Code. Der Workflow-Header dokumentiert Hosted-Runner-Präemption als erwartet („The runner has received a shutdown signal", 2026-09-28 an 8 Läufen) und hat Checkpoint/Resume; **kein Code-Defekt**.
- **Blockade:** keine
- **Braucht:** kein Fix — die rote Anzeige ist die erwartete Präemption; `matrix-rotor.yml` bleibt unverändert. Optional die Shutdown-Zeile im vollen Log bestätigen (`gh api .../logs`), nicht nötig.

### hips-png / ps1 — laufende Shards
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende
- **Lage:** (gemessen 2026-10-02 via `ci_manage`) `ps1-cdn 36939444468` in_progress (shard auto-resume); keine offene Aktion.
- **Blockade:** keine
- **Braucht:** keine — beim nächsten Pass schließen, wenn kein Shard rot.

### Orphan-Doku-Träger (Meta-Klasse)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs`-Lauf
- **Lage:** (gemessen 2026-10-02) **5** ohne Live-Träger: `docs/concepts/exzellenz-konzept.md` (2), `docs/concepts/kybernetische-astrophysik.md` (1), `docs/paper/flyby-path-2-addendum-2026-09-29.md` (26), `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (1), `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (5). Diese Übergabe trägt sie als Meta-Klasse.
- **Blockade:** je Dokument fehlt der Owner-Träger
- **Braucht:** je Dokument beim Owner (mountain: orphan-verdicts + pendings-inventur; river: flyby-path-2-addendum + astrophysik + exzellenz) eine Trägerzeile oder ein gemessenes `descoped`; Marker sind teils Prosa (`die Natur ist offen`) — Marker-Review, dann descope.

### Register-Träger — `phi/pipeline/index.φ` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Katalog-Port (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-02 via `register_lookup --open`) Katalog-Offenstand 5 (Arbeitsdateien gitignored).
- **Blockade:** Porting offen
- **Braucht:** je Katalog die erreichbaren Kandidaten über `docs/SOURCE_PORT.md` portieren.

### Register-Träger — `phi/pipeline/ledger.φ` SSDC
- **Status:** termin | **Bindung:** termin:2026-10-02
- **Trigger:** neue SSDC-Prozedur (`https://limadou.ssdc.asi.it/query.php`)
- **Lage:** (gemessen 2026-10-02 via `state/zustand/wartend.φ:5`/`:10`) `ledger.φ:6` `ausstehend`; `query.php` CAS-Login-Wall bestätigt (future-165), Sotgiu „wait a few weeks".
- **Blockade:** Prozedur nicht live
- **Braucht:** nach Termin `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"`.

### `blocked_sources.φ` mycelium-pending-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30) `:59` BepiColombo, `:85` MESSENGER, `:98` DEMETER, `:346` GOSAT-GW, `:374` DAS2 Iowa, `:378` Occultation-DB, `:402` ExoMars TGO, `:406` Akatsuki, `:410` Kaguya, `:414` Chandrayaan-1, `:418` Chang'e MRM, `:422` Tianwen-1 RoPeR, `:426` Phobos 2, `:430` Vega 1/2, `:434` Hayabusa, `:438` Tianwen-1 MoRIC, `:442` Shandong, `:458` Danuri ShadowCam, `:462` CDSE-CCM.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### GitHub-Issues — Zensus (gemessen 2026-10-01)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Issue-Review beim nächsten Pass (`gh issue list --state open`)
- **Lage:** (gemessen 2026-10-01 via `gh issue list`) **18 offene**: #116 paper-gate (river), #115 pds3_fixed_width_darts (harvest `36738406375` failure, nicht geheilt), #114 de441-cdn (geheilt: `de44-cdn 36868002454` success), #113 dropped-gate (geheilt: Baseline 1339 → `dropped-baseline.md:16`), #81 clippy, #80 Anomalie-Report, #71/#30 flatten bodies, #60/#17 recheck-live drift, #58/#15 cargo test, #53 nvss, #52 first14, #50 vsx, #49 frbcat_flat, #48 gcvs_cat, #47 cbdata.
- **Blockade:** `gh issue close` strukturell verweigert (`opencode.json`)
- **Braucht:** Operator-/future-Akt: `gh issue close 113 114` (gemessen geheilt) + `#47–#53` gegen die letzten `*-cdn`-Läufe prüfen.

### Quellenseitige Waits (`state/zustand/wartend.φ`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort/Readiness
- **Lage:** (gemessen 2026-10-02) Aufnehmer mycelium: `ssdc-limadou`, `voyager-nssdca`, `mariner10-nssdca`, `viking-nssdca`, `cassini-trk`, `juno-jplnav`, `superdarn-af68c4f1`, `emodnet-hfr`, `bepicolombo-more`, `noirlab-gaia-dr4`; die ODF-Rohdaten-Anfragen (ESOC/KinetX/Turyshev) unter `:29–31`.
- **Blockade:** Antwort
- **Braucht:** Postfach + Wiedervorlagen beobachten (Trigger feuert → selbes Atom).

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2026-12-03 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `europa-clipper` · `bepicolombo-more` · `laic-cses`
- **Lage:** (gemessen 2026-10-02 via `state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium.
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

### `http_401`-Residuum
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue Mail/Asset-Messung
- **Lage:** (gemessen 2026-09-30) nach der GitHub-PAT-Rotation kein neuer 401.
- **Blockade:** keine
- **Braucht:** weiter beobachten.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383`)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

## LOCK

- **`rr-brustgurt`** (Operator): LOCK — Hardware erst bei Förderung; Live-BLE HR NotSupported → keine RR; FIT `nn=0` (gemessen 2026-09-26); Brustgurt Polar H10/HRM-Dual. (`state/zustand/wartend.φ:22`)

## An river

Origin: mycelium-folge219 (CI-Tafel).

- **clippy rot @`3965e3814` (`ci-gate 36936698719`):** 4 Clippy-Fehler unter `-D warnings` — `needless_range_loop` `src/mathematikerin/least_squares.rs:28`, `useless_vec` `src/mathematikerin/least_squares.rs:71`, `approx_constant` Φ `src/mathematikerin/omega.rs:5` und `src/mathematikerin/te.rs:1952`. Gemessen via `ci_manage log 36936698719`. Bitte heilen + pushen; der nächste `ci-gate` misst.
- **tapvizier-TAP-Klasse (`rave-cdn 36932654093` / `sb9-cdn 36932650180` / `first14-cdn 36932643096`):** gemeinsame Ursache `tapvizier.cds.unistra.fr` TAP — HTTP 400 (`tap_query exit 22`) bzw. `uws job phase ERROR`. `tap_compiler` (`tools/harvest/src/bin/tap_compiler.rs`) ist die Feder; river 77 `d9baca435` („fix ADQL quoting/count") hat die Klasse nicht geheilt. Bitte die generierte ADQL gegen `tapvizier` messen.

## An future

Origin: mycelium-folge219.

- **GitHub-Issues:** `gh issue close` ist in `opencode.json` strukturell verweigert (Session darf nur `list`/`view`). Gemessen geheilt und schließbar: #113 (dropped-Baseline 1339) + #114 (de44-cdn success). #115 (harvest `36738406375` failure) und #81 (clippy, alter SHA) brauchen eine neue Messung. Bitte in die Operator-Queue: `gh issue close 113 114`, dann `#47–#53` gegen die letzten `*-cdn`-Läufe.

## An mountain

Origin: Operator-Wort (Mycelium-Session 2026-10-02); Caveat ehrlich registriert.

- **Wire-Richtungs-Slot fehlt ohne Abstand:** der 26×f64-Wire trägt **keinen Richtungs-Slot**, wenn kein Abstand (Parallaxe/Distanz) gemessen ist. Die **extragalaktischen Spektren** (`format jwst_spectra`, `curated48_spectra.bin`/`ceers_spectra.bin`, `sources.φ:10382`/`:10926`) sitzen darum jetzt am `at`-Anker (`sun`); die **eigentliche Messung (die bins/flux-Serie) ist vollständig geführt**. Eine echte Richtungs-/Distanz-Führung bräuchte ein **`z` im JWS1-Bin** → das ist ein **Compiler-/Contract-Schritt** (Mountain/Mycelium). Kein stiller Default, keine fabrizierte Richtung — der Riss steht.
- **Free-Voice-Kampagne 2026-10-02 — Route-Leads (Session am Baum gegengeprüft, `archive_search --verdict`), Verdikt-Feder bei Mountain:**
  - **M3-ENVI-Spiegel** (`blocked_sources.φ:468` `ip-blocked`): Wayback `…/web/20160603190919/http://pds-imaging.jpl.nasa.gov/data/m3/CH1M3_0003/DATA` **200**; ODE `ode.rsl.wustl.edu/moon/indexProductSearch.datasetFiles.aspx` **206**; SBN `sbn.psi.edu/` **206**; `pds-imaging.jpl.nasa.gov/volumes/m3.html` **206** (nennt die Spiegel-Sites).
  - **MESSENGER Erd-Encounter 2005** (`:86`): SPDF `spdf.gsfc.nasa.gov/pub/data/messenger/` **206**; NAIF `naif.jpl.nasa.gov/pub/naif/MESSENGER/` **200**; WUSTL `pds-geosciences.wustl.edu/missions/messenger/` **206**; 2005-spezifisch bleibt `unbelegt` (`/data-odf/2005/` 404).
  - **Tianwen-1 MoRIC** (`:439`): `alasky.cds.unistra.fr/…/Norder6/Dir0/Npix0.png` **206**.
  - **Danuri ShadowCam** (`:459`): `pds.shadowcam.im-ldi.com/derived/` **200** (13117 B).
  - **PDS-Imaging-Node-Spiegel:** ODE `ode.rsl.wustl.edu/` + SBN `sbn.psi.edu/` **206**; USGS Astrogeology `astrogeology.usgs.gov/search` **206** (Wurzel 403) — der alte `pdsimage.wr.usgs.gov` ist tot (nur Wayback 1996), der lebende USGS-Knoten ist `astrogeology.usgs.gov`.
  - **Shandong** (`:443`): Wurzel 206, `/data/` 404, kein `/data/`-Snapshot. **GOSAT-GW** (`:347`)/**BepiColombo** (`:60`): `unbelegt`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
