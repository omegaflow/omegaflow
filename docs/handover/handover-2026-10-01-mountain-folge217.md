<!--
  title: Handover — Mountain-Folge 217 (Stand 2026-10-01)
  session: Mountain-Folge 217
  class: handover
  date: 2026-10-01
  sha256: f27d4831493e21cf48a99ccebab98238147608c4f2213f9d6080ebf87e202025
  status: live
-->
# Handover — Mountain-Folge 217 (2026-10-01)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; Stand der
Runde am `0c32b4a87`, HEAD dieses Atoms `6c5490f79` — parallel auf `e40af9679`
fortgeschrieben). Mycelium-216 (`31bf509c4`) hat die zwei `sources.φ`-Blöcke
(`pioneer11_odf`, `pioneer10_telemetry`) angelegt — **ohne `ttl`** (Mountains
exklusives Recht); Mountain hat `ttl 604800` gesetzt und den Archivar-Arm für
`pioneer11_odf` gebaut (`extract.rs` parse_series + `series_component_name`,
`main_flow.rs` Serien-Liste, Test; `cargo check` 0/0). Der Rat hat
`pioneer10_telemetry` **einstimmig als Nicht-Zeuge** entschieden
(Instrument-Eigenzustand, kein Welt-Zeugnis); `PTLM` ist in `witness.rs` als
`FieldIdentity::Pending` benannt (Test), der Witness-Disclaim steht in
`phi/witnesses.φ`. Die drei declined/infra-Dateien (`who_flunet`, `nbp_Lmon`,
`WMMHR.COF`) liegen in `archive-root/declined/`. Die Anderson-ITRF/EOP-Kontrolle
aus future-162 ist beantwortet.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„jeder Punkt trägt eine Empfehlung; wartende Linien erhalten eine bevorzugte Abarbeitungsbitte" | 2026-09-29 | Operator (Mountain 204)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„braucht es wirklich pro?" — pro nur mit benanntem Hart-Atom oder gemessener flash-Fehllage | 2026-09-30 | Operator (Session, Mountain 211)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„arbeite deine Liste bis zur Kante ab" — jeder eigene Punkt bis zur Kante, nichts Machbares liegen lassen | 2026-09-30 | Operator (Session, Mountain 212)
„verschleppen und nicht eigenes ist verboten" — Linienliste nur `eigen`, jeder Punkt im Atom bis zur Kante | 2026-09-30 | Operator (Session, Mountain 213)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-09-30 | Operator (Session, Mountain 216)
„<LOCK-Wort für das private Experiment>" | 2026-10-01 | Operator (Session, Mountain 217) — verbatim im privaten Cut `state/operator-gespraeche/2026-10-01-mountain.md`; LOCK privat, kein CDN/`sources.φ`/`witnesses.φ`, kein getrackter Baum; zur Faltung nach `state/future/handover/handover-2026-10-01-future-folge163.md` (Origin: mountain folge217)
„alles was das experiment betrifft bleibt privat" | 2026-10-01 | Operator (Session, Mountain 217) — stehend: das ganze private Experiment (Daten, Ableitungen, experiment-spezifischer Code) bleibt privat; private Heimat `state/mountain/kuprat-complex-te/`; zur Faltung nach `state/future/handover/handover-2026-10-01-future-folge163.md` (Origin: mountain folge217)

## Offen (aufgeschlüsselt)

### Ephemeriden-Bins — Unstetigkeit an den 32-d-Granulat-Grenzen (Ursache geheilt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner Ephemeriden-Re-Manifest-Lauf (`kernel-flatten` / `ephemeris_compiler`) → dann `ephemeris_granule_census` gegen die neuen Bins.
- **Lage:** `ephemeris_granule_census` (gemessen 2026-09-30): DE441 Erde 346877 Granulate, INPOP 684, EPM 4875 — je 32.0 d Halbbreite, Chebyshev-Grad 17; rekonstruierte Position sprang an den Grenzen um Median 0.044 km (max 0.19 km). Ursache: `src/mathematikerin/least_squares.rs::solve_normal_equations` reduzierte den RHS nicht mit der Matrix → Fit ~40 m neben der Funktion; **geheilt** (Test `normal_equations_reduce_the_rhs_with_the_matrix`, `granule_fits_meet_at_the_shared_boundary`; `--selftest` 0.00003 m = f64-Boden). Der Fix ist committet (`946c7b232`, in HEAD `6c5490f79`).
- **Blockade:** die CDN-Bins sind mit dem fehlerhaften Solver kompiliert — der Fix greift erst nach Re-Manifest; der Re-Manifest-Dispatch ist Mycelium-Feder (der Fix liegt auf `main`).
- **Braucht:** Mycelium dispatcht die Ephemeriden-Re-Manifest-Workflows (`de44-cdn`, `inpop-epm-cdn`, `kernel-flatten` + die Missions-Compiler); danach `ephemeris_granule_census` gegen die neuen Bins.

### Ephemeriden-abhängige Messungen — nach dem Re-Manifest nachziehen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner Ephemeriden-Re-Manifest-Lauf (`kernel-flatten` / `ephemeris_compiler`).
- **Lage:** (gemessen 2026-09-30 via `ephemeris_granule_census` + `sread docs/paper/*`) die aktuellen CDN-Bins tragen den ~40-m-Fit-Fehler. **Betroffen (Signal ≤ ~200 m):** `eclipse_shadow_probe` Haus-Cross-Check (`docs/paper/eclipse-clock-worldlines.md:45`: DE442↔INPOP19a Erde 0.064 km, Sonne 0.164 km, Mond 0.168 km — Größenordnung des Bugs, möglicherweise Artefakt statt Haus-Riß); `ephemeris_house_gate` (Δ DE↔INPOP 0.19 km); `flyby_anderson_probe`. **Riss am Siegel:** die präregistrierten Trajektorie-Hashes `flyby-path-2-preregistration.md:21/23` sind Hashes von mit dem Bug kompilierten `horizons_compiler`-Bins — ein Re-Manifest ändert sie (Neu-Siegeln post-hoc unzulässig). **Robust (Signal ≥ km):** Neptune-/Uranus-Rift, KBO-Residuum, Dunkel-Materie-Residuum, Signal-Konus, Galileo-Rotor.
- **Blockade:** der Re-Manifest (Mycelium-Feder).
- **Braucht:** nach dem Re-Manifest `eclipse_shadow_probe` (2017 + 2024), `ephemeris_house_gate` (6 Anderson-Epochen), `flyby_anderson_probe`, `ephemeris_granule_census` neu laufen und die Verdikte fortschreiben (Verschiebung > genannte Unsicherheit = betroffen).

### Kuprat-Treiber-Lauf über öffentliche Kanäle — nicht wohlgestellt (descoped)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-01 via `grind-flash`) der Kuprat-„Treiber-Lauf" (wer treibt die Kohärenz) ist über die öffentlichen Zeugen-Kanäle **nicht wohlgestellt**: RIXS-Spin (Energieachse), RIXS-Charge/EELS (PSD ohne Phase), SRD-62 (λ(T)) tragen keine gemeinsame geordnete Achse; TE braucht eine gepaarte Reihe (n≥8), und die Dotierungs-Achse ist n=3 — `rixs_cuprate_probe` trägt deshalb zu Recht `no statement`. Kein Code gebaut. Der einzige wohlgestellte Pfad ist das private Labor-Holding (LOCK, `## An future`).
- **Blockade:** keine (der öffentliche Pfad ist gemessen leer).
- **Braucht:** — descoped mit diesem Befund; kein öffentlicher Bau.

### Anderson-ITRF/EOP-Kontrolle je Haus — beantwortet (descoped)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** die future-Stimme (`state/stimmen/2026-09-30_anderson-frage_nemotron.txt`) forderte vor jedem survives/dies die ITRF/EOP-Realisation je Haus. (gemessen 2026-10-01 via `general`, DOI `10.3847/1538-3881/abd414` → HTTP 200): **JPL DE440/441 deklarieren keine terrestrische ITRF/EOP-Realisation** — der Himmelsrahmen ist ICRF3, das Erdorientierungsmodell der Integration ist bewusst vereinfacht (Vondrák et al. 2011 Langzeit-Präzession + modifizierte IAU-1980-Nutation, nur 18.6-a-Term; Park et al. 2021, AJ 161, 105, §2.5); **INPOP19a** und **EPM2021** nennen in ihren Release-Notizen keine Realisation. Die konstante Frame-Translation kürzt sich in Differential-Doppler ohnehin heraus; nur die Drift leckt (`ephemeris_house_gate`: Chord-Slope −115 m/yr → ~3.65 µm/s, ~300× unter den ~1 mm/s der FA). → das Anderson-Verdikt (`rift-excluded`, sechs Zeilen) hält gegen die Kontrolle; kein neuer Bau.
- **Blockade:** keine.
- **Braucht:** — descoped mit diesem Befund (die Zitate gilt es bei Bedarf am DOI nachzulesen; die Quelle selbst ist nicht im Baum).

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` — σ-Metrik-Kette (3 Marker); Trigger JUICE In-Situ / Δ publiziert, `flyby_ephemeris_gate` (CI).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — Holdings-Inventur; Schritt 2 gemessen (Holdings + Repo-`data`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` — Relevanz-Erstpass; die Pending-Einträge sind im `dead_sources.φ` disponiert, die Prosa bleibt datierte Messung.
- `docs/surveys/survey-raetsel-bestand.md` — zwölf Nadeln + Blätter + Kuprat, stehende Messreihe; jede Zelle mit `file:line`/`pending`.

## Register-Träger (eigene)

- `phi/harvest.φ` `format hapi_csv` (DAS2 Iowa, Asset `das2_iowa_…`) — `asset fehlt`; Workflow `das2-iowa-cdn.yml` dispatcht `36741690825`.
- `phi/harvest.φ` `format pds3_img` (Chandrayaan-1 Mini-RF) — `asset fehlt`; Workflow rot (M3-`.HDR` HTTP 403, `pds-imaging.jpl.nasa.gov`).
- `phi/harvest.φ` `format pds4_binary` (ExoMars TGO ACS / PSA) — `asset fehlt`; Workflow läuft.

## An mycelium

Origin: mountain folge217 (Registry-first aus future-162 — Aufnahme-Akte und Korrektur der Vorlage).

- **Pioneer-Blöcke geschlossen:** `31bf509c4` hat `pioneer11_odf`/`pioneer10_telemetry` angelegt; Mountain hat beiden `ttl 604800` gesetzt (dein Block ließ es aus — Mountains exklusives Recht) und `PTLM` in `witness.rs` als `FieldIdentity::Pending` benannt, der Witness-Disclaim steht in `phi/witnesses.φ`. Kein offener Hunk.
- **Ephemeriden-Re-Manifest:** der Solver-Fix (`946c7b232`) ist auf `main`; dispatch die Re-Manifest-Workflows (`de44-cdn`, `inpop-epm-cdn`, `kernel-flatten` + Missions-Compiler), dann zieht Mountain die Bins-abhängigen Messungen nach.
- **Ephemeriden generic (new_horizons/voyager1/voyager2):** die drei `url`-Zeilen `phi/sources.φ:15747/15950/15957` bleiben 976-B-Placeholder; setze sie auf den `_long`-Lauf oder verwirf sie zugunsten `ephemeris_*_long`; danach Mountain die Placeholder-Zeilen `descoped`.
- **Kaguya-Idempotence-Audit:** prüfe, ob weitere `*-cdn.yml` den `force`-Input für geänderte Compiler brauchen (CI-Feder, Mycelium).
- **vco-rs:** PDS4-20190704 kanonisch (`0c32b4a87` bestätigt); `pds4_fixed_width_akatsuki` steht — kein offener Hunk.

## An river

Origin: mountain folge216 (Solver-Fix + Re-Manifest; das Sonne-Erde-Blatt/Eclipse-Paper ist deine Feder).

- **Finsternis-Haus-Cross-Check ist vom Solver-Bug belastet:** `docs/paper/eclipse-clock-worldlines.md:45` meldet DE442/DE441↔INPOP19a Erde 0.064 km / Sonne 0.164 km / Mond 0.168 km — dieselbe Größenordnung wie der ~40-m-Fit-Fehler (geheilt, aber die CDN-Bins sind noch alt). Bis zum Re-Manifest sind diese Zahlen `pending`, kein Haus-Verdikt.

## An future

Origin: mountain folge217 (Wort-Faltung, Operator 2026-10-01).

- **Das private Experiment — LOCK privat (Operator-Wort 2026-10-01):** „alles was das experiment betrifft bleibt privat" (verschärft das vorige LOCK). Kein CDN, keine `sources.φ`, keine `witnesses.φ`-Adresse, **kein getrackter Baum**; Daten, Ableitungen (TE-Läufe) und experiment-spezifischer Code bleiben in `state/mountain/kuprat-complex-te/`. Die Zeugen-Klasse steht allein im Code (`LABR`/Substance, Rat 2026-09-29, `mountain-folge204:80-86`). Falte das Wort in deinen LOCK-Abschnitt (`state/future/handover/handover-2026-10-01-future-folge163.md:64-73`).
- **History-Akt angefordert (Operator-Wort 2026-10-01):** der private Reader/Compiler und die Experiment-Referenzen in Handover/Rätzel-Survey sind **bereits getrackt und damit öffentlich**; sie sollen per History-Akt zurückgezogen werden. Eigentümer des Verfahrens: `docs/auftrag/auftrag-pii-history-rewrite.md` (Future/Operator). In dieser Session **strukturell blockiert** (`git push --force` denied, `git filter-repo` fehlt/Python verboten, geteilter Baum) — Prozedur privat in `state/mountain/kuprat-complex-te/`.

## Burn: open 0.0000 · close 0.3636 · cap 0.50 · Grund: Registry-first-Messung (Arm-Lücke), `pioneer11_odf`-Arm, `archive-root`-Disposition, Anderson-ITRF/EOP-Gegenlesung (`general`/flash $0.0589), Wort-Registrierung Experiment-LOCK + History-Akt-Anforderung, privater Rewrite-Vorlauf. Kein pro/max-Dispatch.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
