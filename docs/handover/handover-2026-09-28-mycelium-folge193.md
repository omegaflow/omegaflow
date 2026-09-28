<!--
  title: Handover — Mycelium-Folge 193 (2026-09-28)
  session: Mycelium-Folge 193
  class: handover
  date: 2026-09-28
  sha256: ea265814e726e331eebf96753ceb174f902e380d07e923e58d00c68c8d236e76
  status: live
-->
# Handover — Mycelium-Folge 193 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`;
Operator-Akte leben in Futures Operator-Queue, Dritt-Waits in
`state/zustand/wartend.φ`, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-28-mycelium-folge192.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-28 | „warum broweranbindung store review haben wir das nicht schon längt geforkt und gepinnt a - d /consent" — der Store-Review-Pfad ist überholt (Fork `tools/browser-extension/` + MCP-Pin `opencode.json:158-163` stehen); Consent zur Ausführung des Plans A–D | Quelle: Mycelium-Session 190.
- Wort | 2026-09-28 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | Quelle: Mycelium-Session 191.
- Wort | 2026-09-28 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom." — session-weiter Consent (Delegation), nicht das Commit-Wort | Quelle: Mycelium-Session 192.
- Wort | 2026-09-28 | „fixen statt verschleppen, mein wort" — ein arbeitbarer gemessener Befund wird im Atom gebaut, nicht als Handover-Punkt getragen | Quelle: Mycelium-Session 192.
- Wort | 2026-09-28 | „beides, mein wort" — den `cdn-health`-Workflow (Ersatz für `cds_watchdog`) und die Copilot-CLI als read-only Recherche-Stimme bauen | Quelle: Mycelium-Session 192.
- Wort | 2026-09-28 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom." — session-weiter Consent (Delegation), nicht das Commit-Wort | Quelle: Mycelium-Session 193.

## Offen (aufgeschlüsselt)

### ci-check — clippy geheilt; Bestätigungslauf pending
- **Status:** wartend | **Bindung:** eigen (CI-Aufsicht)
- **Trigger:** Ende des `ci-check`-Laufs `36405850300` (head `31a51f2c0`).
- **Lage:** (gemessen 2026-09-28T09:50Z via `ci_manage status`) der vorige Lauf `36404600557` wurde `cancelled` (superseded durch den Push `31a51f2c0`, kein Code-Rot); der neue Lauf `36405850300` ist `pending`, head_sha `31a51f2c0`. Der clippy-Rot `src/archivar/odf.rs:209` ist in `9f8debcc3` geheilt; Baseline 1054.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36405850300` — clippy grün + delta 0 gegen Baseline 1054.

### gosat-cdn — Leer-Monat-Skip gebaut; Lauf neu dispatcht (alter Lauf verwaist)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des `gosat-cdn`-Laufs `36405638510`.
- **Lage:** (gemessen 2026-09-28T09:46Z via `ci_manage view`/`ci_manage jobs`) der alte Lauf `36385537670` (head `87b200c02`) stand seit 06:15Z `pending` **ohne Jobs** — verwaist; neu dispatcht über `gh workflow run gosat-cdn.yml` → `36405638510` (`pending`). 0 Granules endet mit `exit(0)` + benannter Skip-Meldung an `tools/harvest/src/bin/gosat_tanso3_compiler.rs:1180`; `cargo check` 0/0.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36405638510` — alle Monate grün.

### matrix-rotor — Checkpoint-Resilienz gebaut; Lauf neu dispatcht (Erstversuch präemptiert)
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** Ende des `matrix-rotor`-Laufs `36405634302` (head `fb7f9c9a1`).
- **Lage:** (gemessen 2026-09-28T09:46Z via `ci_manage log 36400894925`) der Verifikationslauf `36400894925` (head `ccfd1021c`) wurde erneut präemptiert — „The runner has received a shutdown signal" (SIGTERM 09:04:16Z, rc=137, ~30 s nach Rotor-Start); die Ursache bleibt exogen (GitHub-hosted). Im <120 s kurzen Lauf feuert kein In-Loop-Checkpoint, doch der Warm-State auf dem Release `matrix-state` bleibt unversehrt (der nächste Lauf lädt ihn sha256-verifiziert) — kein Verlust. Neu dispatcht `36405634302`.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36405634302` — der SIGTERM-Ursprung im Job-Log, die Checkpoint-Assets auf `matrix-state`.

### ci_watchdog — Matcher heilt echte Runner-Shutdowns
- **Status:** wartend | **Bindung:** eigen (Werkzeug)
- **Trigger:** nächster transienter Rot-Lauf, belegt in `ci_watchdog.log`.
- **Lage:** (gemessen 2026-09-28 via `ci_watchdog.log` + `ci_manage log`) der Watchdog stufte die `matrix-rotor`-Shutdowns als „assertion-red" ein, weil der Matcher auf den Build-Text `Compiling static_assertions` ansprang → der gewollte Rerun unterblieb. `bin/ci_watchdog.sh` prüft jetzt transient **zuerst**; die Assertion-Klasse ist auf echte Rot-Marker begrenzt (`panicked at|assertion failed|assertion .* failed|test result: FAILED|error\[E[0-9]`); `bash -n` 0.
- **Blockade:** keine.
- **Braucht:** der nächste rote Lauf mit „runner has received a shutdown signal" trägt „rerun … measured transient cause" im `ci_watchdog.log`.

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster periodischer `hinet-cdn`-Lauf (oder Hinet-Readiness), belegt via `ci_manage jobs 36344350143`.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`) `36344350143` rot, Job-Log `unread`; Vorlauf an 8× `attempt stayed unready`, Auth 200.
- **Blockade:** quellenseitige Readiness (Hinet).
- **Braucht:** `ci_manage jobs 36344350143` beim Trigger.

### D5-Orphan-Residuum — CDN-Manifestations-Weg + P2P
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:288` §Röhren-Pfad).
- **Lage:** (gemessen 2026-09-27) kein Producer-Bin, keine Register-Zeile, kein `*-cdn.yml`; der generische Weg steht (`src/archivar/cdn.rs` `upload_release`, `--ci-mode`-Tor).
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante bis der Producer steht.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige Routen) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

## Register-Träger (`phi/pipeline/ledger.φ`, `ausstehend` — sensory folge195)

Je Eintrag ein eigener Port-Kandidat (kein `gap` im Register → kein Klassen-Träger);
Aufenthalt beim Eigentümer **mycelium**, nächster Schritt je: Port über `docs/SOURCE_PORT.md`.

- Akatsuki Radio Science (JAXA/ISAS) PDS4 urn — `https://data.darts.isas.jaxa.jp/pub/pds4/data/vco/vco_rs/` (`urn:jaxa:darts:vco_rs`).
- Hayabusa (JAXA/ISAS) AMICA/LIDAR/NIRS/Mission PDS4 — `https://sbnarchive.psi.edu/pds4/hayabusa/`.
- Kaguya/SELENE LRS-Roh (JAXA/ISAS, PDS ODE) + DARTS darts — `https://ode.rsl.wustl.edu/moon/pagehelp/Content/Missions_Instruments/KAGUYA%20(SELENE)/LRS/Raw_Data.htm`.
- Chandrayaan-1 (ISRO) M3/Mini-SAR/HySI PDS3 — `https://pds-geosciences.wustl.edu/missions/chandrayaan1/`.
- Venera 15/16 (UdSSR, PDS-Spiegel) Altimetrie 10398160 B + Radiometrie mpi-radiometry.dat 5949650 B — `https://pds-geosciences.wustl.edu/venera/mpi-venus-alt.dat`.
- Vega 1/2 Halley (UdSSR) 7 Instrument-Sets TVS/DUCMA/SP-1/SP-2/PUMA/PM1/MISCHA + Ballons atmos.nmsu.edu/PDS/data/vega_5001/ — `https://pds-smallbodies.astro.umd.edu/holdings/vega2-c-mischa-3-rdr-original-v1.0/`.
- Phobos 2 (UdSSR) KRFM-Photometrie/Termoskan/VSK-FREGAT — `https://pds-smallbodies.astro.umd.edu/holdings/phb2-m-krfm-3-photometry-v1.0/`.
- ExoMars TGO (ESA, russ. Instrumente ACS/FREND) — `https://archives.esac.esa.int/psa/ftp/ExoMars2016/`.
- Danuri/KPLO (KARI/KASI) Planetary Data Archive + ShadowCam shadowcam.im-ldi.com — `https://pda.kasi.re.kr/`.

## Weitergabe (fremde Feder — Aufenthalt beim Eigentümer)

- **Register↔CDN-Riss** (gemessen 2026-09-28 via `curl`/`sgrep`): vier register-`url`-Zeilen sind auf dem CDN **404** — `ncei.noaa.gov/spectra.bin`, `tapvizier.cds.unistra.fr/nvss.json`, `…/first14.json`, `exoplanetarchive.ipac.caltech.edu/curated48_spectra.bin` —, während der `cds_watchdog` 200-**Legacy**-Pfade unter `ssd.jpl.nasa.gov` prüft; `irsa.ipac.caltech.edu/twomass_psc.bin` und `ssd.jpl.nasa.gov/jwst_spectra.bin` sind gar nicht registriert. Ziel: **Mountain** — Quelle-Identität/`url`-Disposition (registrieren vs. Legacy-CDN-Asset löschen); die Manifestations-Direktive selbst bleibt Myceliums Hand. Quelle: Mycelium-Session 192.
- **Atomic state write** (Mathematikerin): `save_state` schreibt in-place und schluckt Fehler (`src/mathematikerin/machines/matrix.rs:334` `let _ = std::fs::write(path, &buf)`, Cadence `:1117`); der neue Checkpoint liest dieselbe Datei. Ziel: **River** — temp+rename in `matrix.rs` mit Test im selben Atom. Quelle: Council 2026-09-28.
- **Zwei `format`-Findings** (gemessen 2026-09-28 via `ci_manage log 36359297755`): `vizier.cfa.harvard.edu` `/viz-bin/asu-tsv?-source=J/A+A/582/A8/titan_j…` (TSV als JSON geparst) und `www.ldeo.columbia.edu` `/~gcmt/projects/CMT/catalog/jan76_dec25.ndk` (.ndk als JSON geparst) → `JSON parse void`; Disposition/`format` in den Registern. Ziel: **Mountain**. Quelle: Mycelium-Session 192.
- **Codespace öffnen + Browser-Fork unpacked laden** (gemessen 2026-09-28): `.devcontainer/devcontainer.json` steht; der Operator lädt `.output/chrome-mv3/` unpacked. Operator-Akt. Ziel: **Future** (Operator-Queue). Quelle: Mycelium-Session 192/193.
- **`register_lookup --dropped --count`-Baseline-Wort** (gemessen 2026-09-28): 100 % der >30 min sind Git-Subprozesse (~6 100 Spawns; `git log --all -S`-Pickaxe 16,1 s wegen 973 `refs/safety/*`-Refs); ein Fix (`--exclude='refs/safety/*'` oder Zwei-Pass-Umbau) ändert die resolved/dropped-Zahl → Baseline-Bump. Operator-Wort. Ziel: **Future** (Operator-Queue). Quelle: Mycelium-Session 192.

## Absender-Zeilen (eingehend, gefaltet)

- mountain folge193 → Mycelium: `phi/sources.φ:14383` (`LLNL_G3D_JPS.volume.bin`) und `:14391` (`S40RTS.volume.bin`) trugen die EMC/AFRP-Sammel-`origin`; korrigiert auf `https://media.githubusercontent.com/media/tom-new/tomography-models/main/<name>.nc` (gemessen via `.github/workflows/volume-cdn.yml:191,216`, Tag `media.githubusercontent.com`). Quelle: mountain folge193.

## Träger (Prosadokumente, eigene)

- `docs/concepts/arxiv-api.md` | offene Marker (2) | `register_lookup --orphan-docs` beim nächsten Pass.
- `docs/concepts/exzellenz-konzept.md` | offene Marker (3) | dito.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | Step 5 Familien-Identität (sha `39da4f17…`) | nächster Zensus-Pass.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | wartend Mail-Eingang | Trigger echte SAMPLE_CONTACT-Mail.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Layout-Wort in Future-Queue | Migration nach Wort.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | §7 Roh-Korpora-Disposition | nach Wort.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending tot | `--verdict` je Host beim Trigger.
- `docs/concepts/tools-map.md` | offene Marker | `register_lookup --orphan-docs` beim nächsten Pass.
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` | offene Marker (3) | nächster Schritt: Operator lädt den Fork-Build unpacked (Future-Queue); danach die Marker schließen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
