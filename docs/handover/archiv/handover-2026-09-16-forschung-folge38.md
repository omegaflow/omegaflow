<!--
  title: Handover — Forschung-Folge 38 (2026-09-16)
  session: Forschung-Folge 38
  class: handover
  date: 2026-09-16
  sha256: 448ab6821b3bc02ca0ddd1677f7e9718ad168cafed29ff09f6d626062cc4cfa6
  status: live
-->
# Handover — Forschung-Folge 38 (2026-09-16)

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
Der Planungs-Pass nennt **einen schweren und fünf leichte** offene Punkte (der
schwere ist der erste offene Abschnitt, die leichten sind mechanisch
schließbar); die Session arbeitet beide ab.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Die Weberin — offene Register-Pflichten

- **Schritt 3 — die vier Rat-Messungen (gebaut, Lauf offen)** — die Probe
  `tools/measure/src/bin/s2_weberin_probe.rs` und der sortierte Körper-Fold in
  `sky_tick` (`src/mathematikerin/omega.rs`; Körper nach Name, Stationen zuletzt)
  stehen; `cargo check` + `--tests` 0 Fehler/0 Warnungen, Ordnungs-Test
  `sky_tick_folds_bodies_by_name_and_stations_last`. Der Rat hat die
  Nichtreproduzierbarkeit des Ruhe-Sets unter Cap-Überlauf als Fix (nicht
  descoped) bestätigt — die Sortierung ist der Fix. (Schritt: nach Push
  `gh workflow run s2-weberin-probe.yml`, Artefakt lesen, die fünf Zeilen —
  sub-resolution gegen π/64, Erd-Stapelung, Coverage bei `t_presence`,
  Shell-Baseline Σω, Cap-Margin 2¹³−N — in `docs/concepts/die-weberin.md` §8/§9
  + Register.)
- **Schritt 7 — Vlies-Asset-Manifestation** — `upload_asset`-Wiring und die
  `*-cdn.yml` stehen; das Vlies-Asset selbst ist nicht manifestiert. (Schritt:
  CDN-Pfad, `vlies_density_compiler` + `vlies-density-cdn.yml`.)

## Parser / Reader-Bauten
- **Parser COSMIC-2 / TEC-GIM-LZW / MiniSEED** — Reader-Bauten. (Schritt:
  `grind-pro`, Muster `geo.rs`.)

## Council
- **304-Å-Trigger falten vs. descopen + Blatt 2/3 fam/max-T-Bound.** (Schritt:
  Rat, vor jedem Blatt-Schreiben.)

## h0 / CMB — Reste
- **CMB clik/CosmoMC-Eigenevaluation** — die Chain-Statistik ist gewogen, der
  Likelihood-Code-Lauf selbst bleibt offen. (Schritt: Bau eines `clik`/CosmoMC-
  Atoms — oder descoped nach Messung; `docs/paper/h0-lines-register.md`
  §„Named pending points" (1).)

## Quellen-Pass
- **CHAMP/GFZ-ISDC PLPT** — PLPT-zip→Tabellen-Compiler `pending` (kein
  Harvest-Bin verifiziert). (Schritt: `grind-pro`, Harvest-Bin.)
- **Rosetta RSI** — Register an ernte (gepostet); nur EAR2 (2007). (Schritt:
  ernte faltet.)
- **Zenodo** — sha256 `pending` (nur md5 geführt): Quaoar
  `10.5281/zenodo.21185812`, TNBFits `10.5281/zenodo.10620251`. (Schritt:
  Register an ernte.)
- **NRS-Hydrophon** — native FLAC, Compiler fehlt. (Schritt: Bau-Linie,
  FLAC-Decoder.)

## Bande-Split / Sonden-ODF
- **160-Hz-Amplitudenzensus** — Workflow `.github/workflows/pioneer-band-amplitude.yml`
  gebaut (Muster `pioneer-link-correction.yml`). (Schritt: nach Push
  `gh workflow run pioneer-band-amplitude.yml`, Artefakt lesen, Zahl ins Register.)
- **NOCC-Reduktionsvorschrift** — die Datei ist forschung-eigen (geklärt
  2026-09-16); die Integration der Reduktionsvorschrift selbst bleibt offen
  (§176: „the named machine of the NOCC reduction remains open"). (Schritt: die
  Vorschrift in `twenty-second-band-ground-chain.md` integrieren.)
- **7 planetare ODF** — `planetary-odf-cdn` run 35097513235 noch `in_progress`
  (zwei Jobs laufen: mars_express, rosetta); odyssey exit 1, mro canceled —
  Annotationsebene gemessen, `--log-failed` bis Abschluss gated. (Schritt:
  `gh run view 35097513235 --log-failed` nach Abschluss.)

## Positionslinien / Ephemeriden
- **Horizons-Residual** — `ephemeris-horizons-check` 35117188074 success, alle
  Körper `0 uncovered`; der Chebyshev-Residual reicht bis neptune 457925 m. (Schritt:
  prüfen, ob der Residual die erwartete Chebyshev-Approximation ist oder eine
  Schwelle braucht — `ephemeris-horizons-check.yml`.)

## CI / geteilter Zustand
- **CI-Status** — Eintrag in `docs/zustand/external-state.md` ist beim
  HEAD-Wechsel fällig; nach dem Push neu messen (format-Job, test-Job mit dem
  neuen S²-Ordnungs-Test). (Schritt: `gh run list --workflow=ci-check.yml` +
  Check-Runs des neuen SHA.)

## Paper / Präregistrierung
- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE
  28./29.09., Clipper 03.12. (Schritt:
  `docs/paper/flyby-path-2-preregistration.md`.)
- **JWST disequilibrium** — O2/O3-, red-edge-, saisonale Kanäle `pending`, bis
  eine Detektion samt Spektrum ins Register tritt. (Schritt:
  `docs/paper/jwst-disequilibrium-survey.md` §6.)

## Extern gebunden (kein Datum)
- Operator-Zeilen an entscheid (unverändert, `post.md`): CDDIS IONEX,
  GIC/INTERMAGNET, Fink/ANTARES/Rubin, WWLLN, Voyager/JPL-DSN + rohe
  ODF/TRK-2-34, Zhangheng-1, Babamul + GHRC, Woo/Armstrong, BiSON, DEMETER,
  CSES, Flyby-Path-2-Siegel. (Schritt: entscheid faltet.)

## Benchmark
- **Rat → flash (2026-09-16), gemessen (`session_burn`).** Der Rat (council,
  pro/max) trug die Messvorschrift (vier Aussagen + Cap-Verdikt: Fix statt
  descoped) für $0.0391; `grind-flash` baute Probe, sortierten Fold, Test und
  Workflow (`cargo check`/`--tests` sauber) für $0.0708. Der Bau kostete mehr als
  der Rat, weil er das größere Stück trägt — die Klasse
  *Weberin-Schritt-3-Messvorschrift* bleibt Rat+flash, kein Doppel-Benchmark.
- **Workflow-Dispatchs (flash, gemessen):** `riss-knoten` 35117186444 success
  (Station ABK ehrlich `absent`), `topocentric-coupling` 35117188010 success,
  `ephemeris-horizons-check` 35117188074 success.
- **Empfehlung:** die *S²-Messprobe-Bau*-Klasse zitiert flash (wie die
  *S²-Bau*-Klasse in Folge 37); pro/max bleibt für die harten Atome.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
