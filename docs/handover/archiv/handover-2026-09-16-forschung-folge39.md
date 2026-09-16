<!--
  title: Handover — Forschung-Folge 39 (2026-09-16)
  session: Forschung-Folge 39
  class: handover
  date: 2026-09-16
  sha256: e9a4ebcf6eea015c2e468c7346295feb6e798ce928c17f6ae5678d6fa607554d
  status: live
-->
# Handover — Forschung-Folge 39 (2026-09-16)

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
- **`ephemeris_juice_cog.bin` 404** — in `phi/sources.φ` als Body-Quelle
  registriert, auf dem CDN `ssd.jpl.nasa.gov` absent (Probe-Lauf 35119482268:
  76 von 77 Bins geladen, `juice_cog` leer/absent). (Schritt: ernte — Asset
  manifestieren oder den Eintrag als `absent`/`pending` messen.)

## Bande-Split / Sonden-ODF

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
- **Sonne ohne `radius_m`/Position** — die Probe 35119482268 meldet
  `sun absent (no radius_m or no position)`; das sub-resolution-Verdikt schließt
  die Sonne darum aus (alle gemessenen Körper sub-resolution). (Schritt:
  Sonnenradius/Position in `phi/sources.φ` messen oder als `absent` benennen.)

## Die Weberin — Rest

- **Vlies-Konsumption** — die Manifestation ist gemessen (Lauf 34064753336,
  `vlies_density.vlde`, sha256 `7bf53447…`); offen bleibt der Archivar-Reader
  für `.vlde` (`format`/Reader fehlen). (Schritt: Reader + `format vlde`,
  eigenes Atom.)

## CI / geteilter Zustand

- **CI-Status** — Eintrag in `docs/zustand/external-state.md` ist beim
  HEAD-Wechsel fällig; nach dem Push neu messen (format-Job, test-Job mit dem
  S²-Ordnungs-Test). (Schritt: `gh run list --workflow=ci-check.yml` +
  Check-Runs des gepushten SHA.)

## Paper / Präregistrierung

- **20-s-Bande-Papier — Tag/Branch** — gewandert an `entscheid` (Post
  `docs/handover/post.md`): die Namenskonvention fehlt am Baum
  (§7 Übersetzungsregeln/One-Source-Regel absent), das Namens-Wort ist
  operator-gebunden. (Schritt: entscheid faltet; danach mechanisch
  `git tag`/`git branch`/`git push`.)
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

- **Workflow-Dispatchs (flash, gemessen):** `s2-weberin-probe` 35119482268
  success (5 Rat-Messungen), `pioneer-band-amplitude` 35119481936 success
  (160-Hz-Zensus). Beide Läufe auf sha `70e2030b` (origin/main zum
  Dispatch-Zeitpunkt), nicht auf dem Session-HEAD `75993600`.
- **Rat (pro/max) → flash (gemessen):** der Rat trug das Tag/Branch-Verdikt
  (Option B: operator-gebundenes Namens-Wort, Post an entscheid) — die
  Namenskonvention existiert nirgends am Baum, ein Session-erfundenes Schema
  wäre die Fabrikations-Klasse. `grind-flash` verifizierte den Vlies-Stand
  (Doc-Drift statt Manifestation, sha256-gemessen). Die Klasse
  *Konvention-fehlt-am-Baum* bleibt Rat, kein flash-Benchmark.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
