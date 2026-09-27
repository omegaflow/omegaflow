<!--
  title: Handover — Mycelium-Folge 188 (2026-09-27)
  session: Mycelium-Folge 188
  class: handover
  date: 2026-09-27
  sha256: 6ca0574bcab9f422a3437fe83f1e2a622ceb5b188b80337d8edb9d373e7f423b
  status: live
-->
# Handover — Mycelium-Folge 188 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` |
`termin`; Operator-Akte leben in Futures Operator-Queue, Dritt-Waits in
`state/zustand/wartend.φ`, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge187.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-27 | „Du kannst. Führe den … Plan aus — als `line`-Agent" — session-weiter Consent (Delegation), **nicht** das Commit-Wort | Operator (Mycelium-Session 187).
- Wort | 2026-09-27 | ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | Operator (Future-Session).
- Wort | 2026-09-27 | Entscheidungen nie als Liste vorlegen — jede Entscheidung braucht eine aussagekräftige Erklärung | Operator (Future-Session).
- Wort | 2026-09-27 | D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | Operator (Session, Mountain).
- Wort | 2026-09-27 | „falte alle" — die genuin-offenen Punkte der trägerlosen Docs in die Übergaben ihrer Linien falten (Aufenthalt = Eigentum) | Operator (Mycelium-Session 184).
- Wort | 2026-09-27 | „den rest gebe ich future" — die Tafel trägt nur `eigen`; operator-gebundene Punkte → Future-Operator-Queue, Dritt-Waits → `state/zustand/wartend.φ`; die Queue wird nicht kopiert | Operator (Mycelium-Session 185).

## Offen (aufgeschlüsselt)

### register-dropped — Sweep-Ergebnis
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `36344055152` (`sweep`) beendet.
- **Lage:** (gemessen 2026-09-27 via `ci_manage status`/`jobs`) in_progress auf dem Step `sweep → Dropped sweep — full-history measurement`; der Timeout-Fix (`timeout-minutes: 30 → 180`) steht mit `02f70c517`.
- **Blockade:** Lauf läuft.
- **Braucht:** `ci_manage log 36344055152` **einmal** lesen; die Namen je Owner auftragen.

### dropped-gate — Baseline 989 vs current 1032
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `register-dropped`-Sweep erfolgreich.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36333267310`) `dropped-gate: baseline 989 | current 1032 | delta 43`; `docs/zustand/dropped-baseline.md:16` = 989 @0a0ce96d. Die 43 Namen nennt das Log nicht (unread).
- **Blockade:** keine (Sweep über den Timeout-Fix erreichbar).
- **Braucht:** die 43 je Owner aus `ci_manage log <register-dropped-id>` auftragen; legitime Drops ins annehmende Handover, dann Baseline 989 → 1032 bumpen (nie still).

### CI-check — Zählung + Transfer (Mountain bereits getragen)
- **Status:** wartend | **Bindung:** eigen (CI-Aufsicht)
- **Trigger:** neuer `ci-check`-Lauf am HEAD beendet.
- **Lage:** (gemessen 2026-09-27 via `ci_manage status`) `36341839537` `test`-Job in_progress; `36344048815` pending. Der rote Vorlauf trug 29 Fehler (25 `archivar::*` Mountain, 4 `mathematikerin::*` River); Mountain hat die 25 begrünt und die 4 in `handover-2026-09-27-mountain-folge187.md` `## An River` getragen.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <id>` einmal lesen, sobald der Lauf durch ist.

### modis-cdn — Einmal-Migration
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `36340759898` (`modis-year-split`) beendet.
- **Lage:** (gemessen 2026-09-27 via `ci_manage jobs 36340759898`) Step `split → Lift every family shard into its year tag, then delete the verified-moved slot` in_progress seit 18:28; zuvor scheiterten `compile`/`series-manifest` an `HTTP 422 … file_count limited to 1000 assets per release`. Schema (b) ist gebaut (`dfb405a1b`).
- **Blockade:** Lauf läuft.
- **Braucht:** Ergebnis einmal lesen; erst nach migriertem Familien-Tag `gh workflow run modis-cdn.yml` (sonst 422).

### gosat-cdn — Ghost-Lauf (2026-Shard)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Median-Basis vorhanden (≥2 erfolgreiche `gosat-cdn`-Läufe) ODER Watchdog-Reife.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`/`jobs`) `36322845122` Step `compile (2026)` in_progress seit 13:34; Zwilling `36322848137` pending 0 Jobs. Im lesbaren 100-Lauf-Fenster (13:33–19:26Z) **0 erfolgreiche `gosat-cdn`-Läufe** → keine Median-Basis → **nicht gecancelt** (`gosat-cdn.yml` wurde erst 2026-09-27 gebaut).
- **Blockade:** Ghost runner-seitig; Median-Basis fehlt.
- **Braucht:** beim nächsten Pass (mit ≥2 erfolgreichen Läufen) Median×2 prüfen → `ci_manage cancel 36322845122` + `gh workflow run gosat-cdn.yml`.

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ergebnis des Re-Dispatchs `36344350143`.
- **Lage:** (gemessen 2026-09-27) re-dispatched `36344350143` (queued); Vorlauf `36323256126` failure: `cont status never read Available — the request stays unfetched`, 8× `attempt 0..7 stayed unready`; Auth 200 (6659 B).
- **Blockade:** quellenseitige Readiness.
- **Braucht:** `ci_manage log 36344350143` einmal lesen; bleibt es, wartend auf Hinet, beim nächsten periodischen Trigger erneut dispatchen.

### ESA-CCI-SST — Compiler gebaut; Reader-Arm + CDN-Dispatch offen
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Push dieses Atoms; Artefakt `.github/workflows/esacci-sst-cdn.yml`.
- **Lage:** (gemessen 2026-09-27 via `cargo check`/`build`/Lauf) `tools/harvest/src/bin/esacci_sst_compiler.rs` gebaut (HDF5/NetCDF-4 via `omegaflow::hdf5`, int16 scale/offset/fill); `cargo check` 0 Warnungen; Lauf `--date 2000-01-01 --stride 128` → 1093 SST-Zellen. Quelle: `dap.ceda.ac.uk/neodc/eocis/data/global_and_regional/sea_surface_temperature/CDR_v3/Analysis/L4/v3.0.1` (CDR3.0 ≤2021 / ICDR3.0 ≥2022, beide gemessen). `phi/sources.φ:879-885` (`format esacci_sst_l4_cdr3`, thermal/K, τ=86400); `.github/workflows/esacci-sst-cdn.yml` neu.
- **Blockade:** keine (Compiler-Teil fertig).
- **Braucht:** nach dem Push `gh workflow run esacci-sst-cdn.yml`; Reader-Arm in `src/` als Mountain-Punkt (siehe unten).

### Workflow-Klassen-Zensus (Step 5 des Orphan-Verdicts-Survey)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Konsolidierungs-Plan je Netloc; Artefakt `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (Step 5).
- **Lage:** (gemessen 2026-09-27 via Klassen-Zensus, Tabelle `/tmp/opencode/workflow-klassen-zensus.md`) 325 `.github/workflows/*.yml`: manifest 246, probe 58, build 12, register 6, manifest-watch 3. **13 Netlocs von ≥2 Klassen erzeugt** (Ziel 0): vizier.cds.unistra.fr (5), pds-ppi.igpp.ucla.edu (5), zenodo.org (3), spdf.gsfc.nasa.gov (3), pds-rings.seti.org (3), naif.jpl.nasa.gov (3), tapvizier.cds.unistra.fr (2 explizit; als Compiler-Konstante aus 23 Klassen), ssd.jpl.nasa.gov (2), minorplanetcenter.net (2), irsa.ipac.caltech.edu (2), ftp.imcce.fr (2), data.pmel.noaa.gov (2), modis_lst_cmg (2). Keine der 5 manifest-Stichproben liest ihre Release-Menge aus `phi/sources.φ` (eigener Tag-Satz, `Datei:Zeile`-Belege im Zensus).
- **Blockade:** Konsolidierung ist destruktiv (Release-Vereinheitlichung, eigener CDN-Bereich) — braucht einen gemessenen Plan je Netloc, bevor ein Release verschwindet.
- **Braucht:** je der 13 Netlocs messen, welche Klasse kanonisch ist und welche Releases betroffen sind; dann Plan. CDN = eigene Domäne (Session-Konsens), aber kein Release-Löschen ohne Plan.

### D5-Orphan-Residuum — CDN-Manifestations-Weg + P2P
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:288` §Röhren-Pfad; `sgrep zeugnis/roehre` in `tools/` leer).
- **Lage:** (gemessen 2026-09-27 via `explore`) kein Producer-Bin, keine `phi/`-Zeile, kein `*-cdn.yml`; generischer Weg steht (`src/archivar/cdn.rs:41` `upload_release`, `--ci-mode`-Tor). P2P: Nostr-P2P in `ce1e231`/`576bcbb` entfernt; Zukunftsform `future-concepts.md:33-38` §4.
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante bis der Producer steht.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige Routen) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>`; bei Erholung `*-cdn.yml` dispatchen.

### Orphan-Docs-Träger (7, operator-Wort „falte alle")
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Träger-Prüfung `register_lookup --orphan-docs` beim nächsten Pass.
- **Lage:** (gemessen 2026-09-27 via `register_lookup --orphan-docs`) 7 trägerlose Prosadokumente mit offenen Markern: `docs/concepts/arxiv-api.md` (2), `blatt-papier-beweis.md` (3), `exzellenz-konzept.md` (3), `kybernetische-astrophysik.md` (10), `pfeiler-der-architektur.md` (2), `positive-maske.md` (2), `docs/surveys/survey-2026-09-17-sonden-request-only.md` (11). Kein lebendes Handover nennt sie.
- **Blockade:** keine.
- **Braucht:** je Dokument die genuin-offenen Punkte lesen und in die Übergabe der besitzenden Linie tragen (Operator-Wort „falte alle").

## Träger (Prosadokumente, eigene)
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | wartend Mail-Eingang (kein Nachfassen) | nächster Schritt: Trigger Mail.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | Step 5 Zensus **gemessen** (13 Netlocs ≥2 Klassen) | nächster Schritt: Konsolidierungs-Plan je Netloc (Punkt oben).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Layout-Wort in Future-Queue | nächster Schritt: Migration nach Wort.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen §7 Roh-Korpora-Disposition; Akt in Future-Queue | nächster Schritt: Disposition nach Wort.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | Wiedervorlage 2026-12-02 | nächster Schritt: 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending tot | nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/concepts/tools-map.md` | offene Marker unverändert | nächster Schritt: `register_lookup --orphan-docs` beim nächsten Pass.

## An Mountain (gemessen, fremde Feder)

- **ESA-CCI-SST Reader-Arm fehlt** (read-only-Messung im ESACCI-Atom): `format esacci_sst_l4_cdr3` wird von der Archivar-Leseseite nicht gelesen — es fehlen `geo.rs::magic_of` + `geo.rs::comp_max` + der `matches!`-Dispatch in `main_flow.rs` + die comp→Feldnamen-Abbildung in `extract.rs` sowie die gemeinsame `COMP_ESACCI_SST`-Konstante (derzeit lokal im Compiler). Ohne diese Arme liest die Pipeline `esacci_sst.bin` nicht → `pending`. Der Manifestations-Teil (`url`/`origin`/`compiler`/Workflow) ist Myceliums Feder und steht.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
