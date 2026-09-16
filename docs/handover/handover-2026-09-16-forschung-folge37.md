<!--
  title: Handover — Forschung-Folge 37 (2026-09-16)
  session: Forschung-Folge 37
  class: handover
  date: 2026-09-16
  sha256: 36d2d1a14b4a296f4ddc3583f0ced06cf0eccacd0ee0a5267761065ac7332cf4
  status: live
-->
# Handover — Forschung-Folge 37 (2026-09-16)

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

**Vor allem anderen: dieses Handover gegen den Baum prüfen.** Jeder offene
Punkt wird gegen den gemessenen Bestand gehalten (`sgrep`/`git log`/`sread`),
bevor irgendetwas gebaut oder gemessen wird — das Register ist die Frage, der
Baum die Messung (A = A).

## Die Weberin — offene Register-Pflichten

Die Proben der Schritte 1–9 stehen (gemessen 2026-09-16); `docs/concepts/die-weberin.md`
§8/§9 ist auf den gemessenen Stand korrigiert. **Schritt 3 — Live-S²-Integration
ist gebaut** (dieser Atom): `S2Osc::from_body`/`from_station` falten Körper- und
Stations-Weltlinien baryzentrisch in denselben `oscs`-Vektor in `sky_tick`
(`src/mathematikerin/{s2.rs,omega.rs}`), Gewicht 1.0 bei Coverage, σ `None`,
4 Tests. Offen:
- **Schritt 7 — Vlies-Asset-Manifestation** — `upload_asset`-Wiring und die
  `*-cdn.yml` stehen; das Vlies-Asset selbst ist nicht manifestiert. (Schritt:
  CDN-Pfad, `vlies_density_compiler` + `vlies-density-cdn.yml`.)
- **Drei neue Workflows, noch nicht gelaufen** — `riss-knoten.yml` (Schritt 8),
  `topocentric-coupling.yml` (Schritt 3), `ephemeris-horizons-check.yml` (L5).
  (Schritt: nach Push `gh workflow run <wf>`, dann `gh run view` je Lauf —
  Artefakte lesen; `riss-knoten`-Station ABK bleibt ungefüttert, die Probe
  meldet ehrlich `absent`.)
- **Rat-offene Messungen zu Schritt 3** — die vier Kernaussagen des Rats
  (sub-resolution: größte Körper-Winkelausdehnung gegen π/64; die
  Earth-Richtungs-Stapelung der Stationen; die Coverage-Lücken der 72+1 Blöcke
  bei `t_presence`; die Shell-Baseline 74+Stationen). Der Cap-Überlauf faltet in
  `HashMap`-Ordnung (Körper), Stationen zuletzt — Stationen ruhen zuerst, das
  Ruhe-Set ist bei Überlauf nicht reproduzierbar; heute unerreichbar (2¹³ gegen
  ~74 Weltlinien), live mit dem Vlies-Fold. (Schritt: eine Probe/Zeile in
  `s2.rs`-Tests oder `band_amplitude_probe`-Muster; die Zahl ins Register.)

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
- **160-Hz-Amplitudenzensus** — Probe gebaut (`band_amplitude_probe.rs`), kein
  CI-Workflow. (Schritt: `pioneer-band-amplitude.yml` anlegen.)
- **NOCC-Reduktionsvorschrift** — Integration in
  `twenty-second-band-ground-chain.md` blockiert (fremde uncommittete Hunk).
  (Schritt: Datei-Eigentum klären.)
- **7 planetare ODF** — `planetary-odf-cdn` run 35097513235 noch
  `in_progress`; odyssey exit 1, mro canceled, mars_express/rosetta laufen.
  (Schritt: `gh run view 35097513235` + `--log-failed` der zwei Fehljobs.)

## Positionslinien / Ephemeriden
- **CDN-Planetenbins** — Workflow `ephemeris-horizons-check.yml` gebaut, Lauf
  offen. (Schritt: dispatch + Artefakt lesen.)

## CI / geteilter Zustand
- **CI-Status** — Eintrag in `docs/zustand/external-state.md` ist beim
  HEAD-Wechsel fällig; nach dem Push neu messen (format-Job, test-Job mit den
  neuen S²-Tests). (Schritt: `gh run list --workflow=ci-check.yml` + Check-Runs
  des neuen SHA.)
- **post.md-Fold** — die `An forschung`-Zeile ist in der Arbeitskopie gelöscht
  (sha256 neu), aber nicht gestaged: `post.md` trägt fremde uncommittete Hunks.
  (Schritt: der nächste `post.md`-Commit trägt die Löschung.)

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
- **S²-Live-Integration (2026-09-16).** `grind-flash` trug den Bau: 3 neue
  Signaturen, `sky_tick`-Fold, 4 Tests, `cargo check` + `cargo check --tests`
  sauber (null Fehler, null Warnungen). pro/max nicht eskaliert — die Klasse
  *Schritt-3-Bau* ist mit flash geschlossen (Regel: flash-first, Eskalation nur
  bei gemessen falscher/unvollständiger Antwort). Burn: `session_burn` —
  Forschung-Linie `$0.0581`; der S²-Bau-Subagent ist einer der 9 `grind-flash`-
  Läufe (`$0.2436` gesamt).
- **Empfehlung an die nächste Session — kein Doppel-Benchmark.** Die Klasse
  *Weberin-Register-/Bestands-Diagnose* bleibt geschlossen (`research-max` fand
  den Doc-Drift); die *S²-Bau*-Klasse ist mit flash geschlossen. Beide werden
  zitiert, nicht erneut gefahren.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
