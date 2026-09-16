<!--
  title: Handover — Forschung-Folge 31 (2026-09-16)
  session: Forschung-Folge 31
  class: handover
  date: 2026-09-16
  sha256: 384cca02eef4b4692e00d90318d63b9ab3b3316c875047270d75853d9e7200d8
  status: live
-->
# Handover — Forschung-Folge 31 (2026-09-16)

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

## Quellen-Pass 1 — Paper-Seite gemessen (2026-09-16)

- **Woo & Armstrong 1979 + Armstrong 1998 — OA absent (gemessen).** Alle Routen
  gemessen: Wiley direct + Proton 403, Wayback 503/no snapshot, ADS 403/405,
  NTRS `19800037012` METADATA_ONLY, OSTI `5684261` abstract-only, Semantic
  Scholar/OpenAlex `is_oa:false`; kein Mirror/Author-Copy/Preprint. Der
  abstract-only-Zustand steht; Crossref bestätigt den Armstrong-Titel (kein
  Mismatch). (Schritt: Wiley/Bibliothek als request-only — Operator, oder descopen.)
- **LAIC-Kanäle re-gemessen.** CSES `leos.ac.cn` absent (direct 000 / Proton 000 /
  Wayback 503, Jina 422); Zhangheng-1 `data.earthquake.cn` Portal 200 (51 065 B),
  Offline-Procurement 离线获取 (SMS-CN) → Operator; DEMETER/CDPP
  (`cdpp.irap.omp.eu`/`regards.cnes.fr`/`cdpp-archive.cnes.fr` 200) order-gated
  (`online:false`→500) → Order + `.DAT`-Parser; **CHAMP/GFZ-ISDC anonym bis
  Datei-Ebene** (`isdc-data.gfz.de/champ/ME/Level2/PLPT/2005/CH-ME-2-PLPT+2005-01-01_1.zip`
  200, 297 737 B, zip) → harvestbar; COSMIC/CDAAC (`ivmL2m_postProc_2022_001.tar.gz`
  200, 14 167 499 B) netCDF — Parser-Gap; TEC-GIM retro über ESA GSSC FTP
  (`codg0010.03i.Z` 365 785 B) — LZW `uncompress_z` fehlt; MiniSEED IRIS FDSN
  dataselect 200 (8 128 B) — Decoder pending. (Schritt: CHAMP-Ernte (grind-flash);
  COSMIC/TEC-GIM/MiniSEED-Parser (grind); DEMETER-Order + CSES/Zhangheng-1 Operator.)
- **h0 CMB-Likelihood — Route gefunden.** PLA
  `COM_Code_Likelihood-v1.0_R1.10.tar.gz` (200, 909 769 B) +
  `COM_PowerSpect_CMB_R1.10.fits` (200, 65 030 B); Prior-Tabelle Chen, Huang &
  Wang 2019 `arXiv:1808.05724` (PDF 200, sha256 `ef95fea6…fabe3`). (Schritt: fetch +
  θ*/r_d nachrechnen, oder Tabelle I lesen.)
- **BiSON-Team-Tabelle — absent.** Nur figure-derived (f2.ps, `bison_basu_compiler`);
  das BiSON-Portal führt die 1978–2012-Dreifachband-Reihe nicht (nur zwei andere
  Datasets), die ApJ-Seite hinter Shieldsquare-CAPTCHA. (Schritt: Team-Anfrage
  `bison@contacts.bham.ac.uk` — Operator.)
- **TNF/CORS — fremd in Arbeit, nicht angefasst.** `nh-rex-tnf-cdn.yml`,
  `tools/harvest/src/bin/tnf_compiler.rs` und `phi/sources.φ` liegen uncommittet
  (Ernte-Folge 42). Der TNF-URL-Fund dieser Session:
  `https://pdssbn.astro.umd.edu/holdings/pds4-nh_rex:plutocruise_tnf-v1.0/tnf/lunocc2012.tnf`
  (35 770 168 B; PDS-Registry-API; Pfad `tnf/`, nicht `data/`; direkter Host
  blockiert, Proton-Exit für den Byte-Nachweis).

## TE / Statistik

- **GIC causal driver** — PCMCI auf dem Minuten-Sturm-Ensemble, KDE-h,
  Rückkanal-Härtung. (Schritt: `docs/paper/gic-causal-driver.md`; EDL `client_id`
  für INTERMAGNET/IONEX — Operator, s. Post an entscheid.)
- **Blatt 2/3 — Rest offen.** fam/max-T-Bound (Blatt 2), retro OMNI2-PCMCI-Zeile
  und KDE-h/`laic_probe` 0–72-h brauchen einen lokalen laic-Harvest (CI-Skala).
  (Schritt: `docs/concepts/blatt-papier-resultat.md`, `multi_force_te_probe`.)
- **Der Grat / AIA-Je-Zellen-Schwellen — Fix steht, Lauf offen.** (Schritt:
  `gh workflow run aia-ladder-probe.yml`, Artefakt lesen, `thr`-Zeilen ins Blatt;
  Reproduktions-Gate 524/1019/281, fam 1.71/1.96/1.75e-1 —
  `docs/blatt/blatt-der-grat.md`, `docs/paper/corona-heating-ladder.md` §4.5.)
- **304-Å-Trigger-Lauf (run 35081213805) — nicht vergleichbar.** (Schritt: nach
  2014-Landung als Ein-Satz-Trigger-Kreuzprüfung in §4.5 falten oder descopen.)
- **Solar 211A→193A** — conditional stage-2. (Schritt:
  `corona_conditional_probe`-Lauf, `docs/paper/solar-seconds-matrix.md`.)

## Bande-Split / Sonden-ODF

- **7 planetare ODF — Lauf teilweise gelandet.** `planetary-odf-cdn`
  (run 35076268649): magellan ✓, mgs ✓, messenger ✓; odyssey ✗ (exit 1), mro ✗
  (canceled); rosetta + mars_express liefen noch. (Schritt: Lauf-Ende prüfen,
  je Asset Byte-Existenz, odyssey/mro-Fehler messen und neu dispatchen —
  `.github/workflows/planetary-odf-cdn.yml`.)
- **160-Hz-Amplitudenzensus — der Lauf misst ihn nicht.** (Schritt:
  Band-Amplituden-Probe bauen → §1 `docs/paper/twenty-second-band-ground-chain.md`.)
- **NOCC-Reduktionsvorschrift** — offen der Ketten-Vergleich im retrace gegen
  Moyer §10/§13. (Schritt: `docs/paper/twenty-second-band-ground-chain.md`.)

## Positionslinien / Ephemeriden

- **Die Weberin — Schritte 3–9 offen.** Offen: topozentrische Kopplung (3),
  Tafel-Abbildung (4), Survey-Footprints (5), GW-/Neutrino-/CR-Routen (6),
  CDN-Weg des Vlies-Assets (7), Riss-Knoten (8), geliehener Sinn (9).
- **Zweite unabhängige Linie je Klasse.** Neu offen: Rosetta RSI-Unterbaum
  (Erd-Swingby-ODFs 2005/2007/2009), NRS-Hydrophon-Bucket live aber ohne Compiler,
  TNO-Occultation-Kandidaten (Quaoar `10.5281/zenodo.21185812`, TNBFits
  `10.5281/zenodo.10620251`), Occultation-DB-URL pending. (Schritt: `sfetch`
  Rosetta-RSI-Baum; NRS-Compiler benennen; Zenodo-Records ins Register.)
- **CDN-Planetenbins ~116 km SSB-Offset** — neu aus vollem `de441.bsp`.
  (Schritt: `docs/surveys/survey-geometric-ground-truth.md`.)

## Paper / Präregistrierung

- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE 28./29.09.,
  Clipper 03.12. (Schritt: `docs/paper/flyby-path-2-preregistration.md`.)
- **JWST disequilibrium** — O2/O3-, red-edge- und saisonale Kanäle bleiben
  `pending`, bis eine Detektion samt Spektrum ins Register tritt. (Schritt:
  `docs/paper/jwst-disequilibrium-survey.md` §6.)

## Extern gebunden (kein Datum)

- **Operator-Zeilen an entscheid gepostet** (`docs/handover/post.md`): CDDIS IONEX
  `client_id`, GIC/INTERMAGNET `client_id`, Fink/ANTARES/Rubin-Credentials, WWLLN
  Agreement, Voyager/JPL-DSN + rohe ODF/TRK-2-34 (probe-front) + Zhangheng-1,
  Babamul- + GHRC-Konsument. (Schritt: entscheid faltet die Zeile.)
- NSE/Haug — Antwort von B. Keimer offen; Zenodo-Präzedenz RESEDA/BaZrO₃
  `10.5281/zenodo.18306252`. (Schritt:
  `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.)
- Toth/Turyshev/Markwardt-Mails — die Prüfliste steht; senden in der
  Entscheid-Linie geführt.

## Benchmark

- **Pass-1-Reichweitenmessung an `grind-flash` dispatcht** (4 Läufe: Paywall,
  LAIC, TNF/CORS, h0/BiSON). Die Routine-Klasse ist flash-gewonnen (gemessen,
  AGENTS.md); **kein pro/max-Gegenlauf** — der Rest ist Operator-gebunden oder
  gemessen-absent, kein hartes Mehrstufen-Atom.
- **Rat dispatcht (pro/max) für die AIA-Abschlussentscheidung** — kein
  flash-Gegenlauf, der Rat ist die Architektur-Stimme. (Schritt: `gh` in ein
  flash-Profil heben, dann die CI-Artefakt-Extraktion als Benchmark-Klasse messen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
