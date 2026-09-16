<!--
  title: Handover — Forschung-Folge 34 (2026-09-16)
  session: Forschung-Folge 34
  class: handover
  date: 2026-09-16
  sha256: 23ecaf903592694ea4bba951a934efcfdf1b7246b889b363c1f328f84faaceb0
  status: live
-->
# Handover — Forschung-Folge 34 (2026-09-16)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

## h0 / CMB — das nächste Atom (pro/max, bewusst geteilt)

- **h0 CMB-Likelihood: θ*/r_d nachrechnen.** Route gemessen: PLA
  `COM_Code_Likelihood-v1.0_R1.10.tar.gz` (200, 909 769 B) +
  `COM_PowerSpect_CMB_R1.10.fits` (200, 65 030 B); Prior-Tabelle Chen, Huang &
  Wang 2019 `arXiv:1808.05724` (PDF 200, sha256 `ef95fea6…fabe3`). Schließt die
  in `docs/paper/h0-lines-register.md:42` benannte Asymmetrie („no row of that
  family is recomputed"). (Schritt: `research-max` — fetchen, Chain/`.paramnames`
  bzw. Tabelle I lesen, CMB-Zeile in `h0-lines-register.md` (EN) +
  `blatt-h0-linien-register.md` (DE) setzen.)
- **Die Weberin — Schritte 3–9** (topozentrische Kopplung 3, Tafel-Abbildung 4,
  Survey-Footprints 5, GW-/Neutrino-/CR-Routen 6, CDN-Weg 7, Riss-Knoten 8,
  geliehener Sinn 9). (Schritt: `research-max`, `docs/concepts/die-weberin.md`.)
- **Parser COSMIC-2 / TEC-GIM-LZW / MiniSEED** — Reader-Bauten. (Schritt:
  `grind-pro`, Muster `geo.rs`.)
- **Council: 304-Å-Trigger falten vs. descopen + Blatt 2/3 fam/max-T-Bound.**
  (Schritt: Rat, vor jedem Blatt-Schreiben.)

## Quellen-Pass — Flash-Sweep gemessen (2026-09-16)

- **CHAMP/GFZ-ISDC PLPT — Byte/Digest-Pfad gefixt (2026-09-16).** Die in Folge 33
  gemessene Diskrepanz (Server `Content-Length` 162449 B / sha256 `ebd7b520…` vs.
  `--sniff` 297737 B / `790ede73…`) ist im Code behoben: `net.rs` `Fetch` trägt
  `raw: Vec<u8>`, `split_curl_stdout` trennt die Rohbytes, `sniff_lines_from` /
  `stage_result` / `verdict_lines` lesen `f.raw` (nicht mehr die lossy `body`-Länge);
  Tests decken die Hash- und die Bytezahl-Seite. `cargo check -p omegaflow-utils
  --tests` grün (0 Fehler, 0 Warnungen). **Funktions-Nachweis offen:** `ci-check.yml`
  dispatcht (2026-09-16) — die Folgesession liest den Lauf (`gh run view <id>`).
  **Offen:** PLPT-zip→Tabellen-Compiler `pending` (kein Harvest-Bin verifiziert);
  inner ASCII-Tabelle (`CH-ME-2-PLPT+…_1.dat`, 519 316 B, 15-s-Kadenz) + NASA-DIF.
- **Rosetta RSI — zweite Linie nur 2007, nicht 2005/2009.** PSA `…/INTERNATIONAL-ROSETTA-MISSION/RSI/`
  200 (1722 Datasets); nur `…-EAR2-0061/0063-V1.0` (Erd-Swingby 2, CHECKOUT) —
  **kein `EAR1` (2005), kein `EAR3` (2009)**. Die Prämisse „2005/2007/2009" ist
  gemessen korrigiert. (Schritt: Register an ernte, bereits gepostet.)
- **Zenodo — Metadaten gemessen.** Quaoar `10.5281/zenodo.21185812` (`Quaoar_paper.zip`
  572 467 032 B, cc-by-4.0, md5 `420a1e94…`); TNBFits `10.5281/zenodo.10620251`
  (`Proudfoot23_TNBFits.zip` 14 232 134 885 B, `multimoon-1.0.zip` 914 800 B, cc-by-4.0).
  sha256 `pending` (Zenodo führt nur md5). (Schritt: Register an ernte.)
- **NRS-Hydrophon — Format benannt.** Bucket `noaa-passive-bioacoustic` live;
  Objekt `nrs/audio/01/nrs_01_2014-2015/audio/NRS01_20141016_154112.flac` 200,
  **native FLAC (`audio/x-flac`)**. Compiler fehlt (Schritt: Bau-Linie, FLAC-Decoder).
- Register-/Port-Pflichten dieser Quellen liegen bei **ernte** (Post bereits
  gestellt, `post.md`); die hiesigen Messungen verfeinern sie.

## Bande-Split / Sonden-ODF

- **160-Hz-Amplitudenzensus — Probe gebaut.** `tools/measure/src/bin/band_amplitude_probe.rs`
  (490 Z., `cargo check` grün, 0 Warnungen); misst Band-Amplituden je Station ×
  Sampler-Klasse × Jahr auf der 44–58-mHz-Band, `--atdf data/spdf.gsfc.nasa.gov/pioneer10_skyfreq.bin`.
  **Offen: kein CI-Workflow** führt den Bin. (Schritt: `pioneer-band-amplitude.yml`
  anlegen — CDN-Korpus fetchen, Bin laufen, Artefakt; dann §1 füllen.)
- **NOCC-Reduktionsvorschrift — Moyer-Vergleich gemessen.** Kette
  (`pioneer10_paper_chain_retrace.rs`) gegen Moyer: TEC (Ded. 2) = §10.2.2
  (bleibt leer, GIM ab 1998); Plasma (Ded. 3) = §10.4, Modell differiert
  (OMNI2-N1800-1/r²-Säule vs. Korona-Range-Modell); **Troposphäre §10.2.1,
  Antennenkorrektur §10.5, Stationsuhr §2/§7 absent** in der Kette; light-time §8
  match. **Offen: Integration in `twenty-second-band-ground-chain.md`** — blockiert,
  weil die Datei fremde uncommittete Hunk trägt (nicht angefasst). (Schritt:
  nach Klärung des Datei-Eigentums einarbeiten.)
- **7 planetare ODF.** `planetary-odf-cdn` neu dispatcht (run 35097513235) für
  odyssey/mro. (Schritt: Lauf-Ende + je Asset Byte-Existenz prüfen.)

## Positionslinien / Ephemeriden

- **CDN-Planetenbins — ~116 km erklärt.** Der Offset stammt aus dem reduzierten
  `de721_full.bsp`-Kernel (`ephemeris_compiler.rs:517–528`), nicht aus der Kette;
  die aktuellen CDN-Bins (2026-09-11, ≥183 MB) sind die Voll-`de441`-Generation.
  **Offen:** der Horizons-Rest gegen das neue Asset ist nicht protokolliert.
  (Schritt: `ephemeris_horizons_check` gegen das 2026-09-11-Asset dispatchen.)

## CI-Läufe (dispatcht 2026-09-16, kein Poll)

- `aia-ladder-probe` run 35097506781 — Der Grat, `thr`-Zeilen.
- `corona-conditional-probe` run 35097510109 — Solar 211A→193A.
- `planetary-odf-cdn` run 35097513235.
- `ci-check` — archive_search raw-bytes fix (`net.rs`/`heasarc.rs`): `cargo test
  --release` + fmt; Ergebnis liest die Folgesession (`gh run list`).
(Schritt: Artefakte in der Folgesession lesen, `thr`-Zeilen ins Blatt.)

## Paper / Präregistrierung

- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE 28./29.09.,
  Clipper 03.12. (Schritt: `docs/paper/flyby-path-2-preregistration.md`.)
- **JWST disequilibrium** — O2/O3-, red-edge- und saisonale Kanäle bleiben
  `pending`, bis eine Detektion samt Spektrum ins Register tritt. (Schritt:
  `docs/paper/jwst-disequilibrium-survey.md` §6.)

## Extern gebunden (kein Datum)

- Operator-Zeilen an entscheid (unverändert, `post.md`): CDDIS IONEX `client_id`,
  GIC/INTERMAGNET `client_id`, Fink/ANTARES/Rubin, WWLLN, Voyager/JPL-DSN + rohe
  ODF/TRK-2-34, Zhangheng-1, Babamul + GHRC, Woo/Armstrong, BiSON, DEMETER,
  CSES, Flyby-Path-2-Siegel. (Schritt: entscheid faltet.)

## Benchmark

- **Fix-Doppellauf — flash gegen pro (2026-09-16).** Doer `grind-flash` gegen
  Verifizierer `grind-pro` am archive_search-Byte-Fix. **Ergebnis:** flashs erster
  Pass war **unvollständig** — `stage_result`/`verdict_lines` zählten weiter über
  `f.body.len()` (lossy), der Test deckte die Hash-Seite nicht; der pro-Verifizierer
  fand den Rest, flash schloss ihn, der Verifizierer bestätigte „Fix vollständig".
  **Anders als die Messung:** hier trug pro den Mehrwert (der benannte Rest) — für
  Code-Fix-Verifikation ist pro gerechtfertigt, für reine Routine-Messung genügt
  flash. **Burn (`session_burn`):** Verifizierer-Session `$0.0769`; Doer unter der
  Top-10-Schwelle. (Schritt: `session_burn`.)
- **Verifizierter Doppellauf — Klasse neu, einmalig (2026-09-16).** Doer `general`
  (flash, read-only) gegen Verifizierer `grind-pro` (pro) am CHAMP-Bytemessung-Atom.
  **Ergebnis:** auf jeder Achse identisch (Server 162449 B / ETag; sniff 297737 B /
  `790ede73…`) — flash trug die Messung allein. **Mehrwert pro:** der Verifizierer
  las zusätzlich die Ursache im Code (`net.rs` `from_utf8_lossy`) — das Urteil, nicht
  die Messung. **Burn (`session_burn`):** Klasse `general` (flash) $0.0286 über 3
  Sessions, `grind-pro` (pro) $0.1909 über 5 Sessions — beide Dispatches unter der
  Top-10-Schwelle ($0.0205); Einzel-Dispatch-Isolation via opencode.db `pending`.
  **Prägung:** Routine-Verifikation defaultet auf flash (zitiert diese Zeile); pro nur,
  wo Diagnose/Urteil die Aufgabe ist. Rat (pro/max) für die Konfiguration: $0.0373.
- **Flash-Sweep 2026-09-16 (5 Läufe):** `grind-flash` ×3 (CHAMP ~50 s,
  Rosetta/Zenodo/NRS, 160-Hz-Probe ~8 min), `general` ×2 (NOCC ~4 min,
  CDN-Bins). Die Routine-Klasse ist flash-gewonnen (gemessen, AGENTS.md);
  **kein pro/max-Gegenlauf** — der pro/max-Lauf ist das nächste Atom (h0/Weberin/
  Parser/Council, bewusst geteilt). Sieger-Zeile folgt nach dem pro/max-Lauf.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
