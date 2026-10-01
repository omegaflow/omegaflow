<!--
  title: Handover — Mountain-Folge 216 (Stand 2026-09-30)
  session: Mountain-Folge 216
  class: handover
  date: 2026-09-30
  sha256: e560153a7f5cfce87bdffb0c9613352e079bfb322e07819756cab524d5fee103
  status: live
-->
# Handover — Mountain-Folge 216 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; Stand der Runde am
`d8c0d8340`, vor den Mountain-Commits `8fdb361b3`/`1a340b191`). Dieses Atom: den
registrierten Anderson-Auftrag bis zur Kante gearbeitet — der Rat hat die Frage entschieden,
die Anderson-Residuen-Tabelle (PRL 100, 091102, Table I) ist gelandet, `flyby_anderson_probe`
trägt jetzt die Rat-Stufen 1+2 (differentielle Chord-Schranke) und ist gelaufen: **alle sechs
Zeilen `rift-excluded`** (max 0.0168 mm/s). Der zwischenzeitliche lokale Riss ist entschieden:
die lokal gemessene „Leck-Rate" liegt auf Chebyshev-Granulat-Grenzen (neu `ephemeris_granule_census`:
32-d-Granulate, rekonstruierte Position unstetig, Median 0.044 km) — **numerisches Artefakt,
nicht physikalisch**; die lokale Schranke ist nicht bindend.
Die adressierten Blöcke (mycelium-folge214, future-folge161) sind gegen den Baum gemessen
und gefaltet. Der Drei-Haus-Tor-Fund vom Voratom ist nachgezogen: `ephemeris_house_gate` nimmt
jetzt `--epoch-hms HH:MM:SS`; ohne Argument bleibt die JUICE-Perigäum-Zeit (11:45:12) als
explizit benannter Default (`cargo build -p omegaflow-measure --bin ephemeris_house_gate`
0 Warnungen, Lauf 1990-12-08 reproduziert folge215: Δ DE↔INPOP 0.1929 km). Dazu die drei roten
`ci-check`-Tests geheilt: `flyby_encounters` chronologisch geordnet, `pds3_binary` +Inf-Byteorder
(big-endian `0x7F800000`), `port`-Replay — die sechs vco_rs-`field`-Zeilen trugen mehrwortige
Beschreibungen, die den Whitespace-Parser verschoben (auf Einzel-Token geheilt).

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

## Offen (aufgeschlüsselt)

### Anderson-Nachrechnung — hält die Flyby-Anomalie gegen Haus-Tor + Familien-Schwelle? (beantwortet)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** der Rat entschied 2026-09-30 (`council`): eine starre Frame-Translation ist eine Eichfreiheit der Range-Observablen, sie kürzt sich in `ρ` heraus — nur ihre Zeitänderung leckt. `data/flyby2/anderson_residuals.tsv` ist gelandet (6 Zeilen, Anderson 2008 PRL 100, 091102, Table I, Quellen im Datei-Kopf; σ-Riss: Galileo II 1.0 vs 0.08, MESSENGER 0.01 vs 0.05). `flyby_anderson_probe` trägt Stufe 1+2 (differentielle Chord-Schranke, Regressionstest `pair_slope_reads_the_change_not_the_offset`) und ist gelaufen (gemessen 2026-09-30 via `cargo run -p omegaflow-measure --bin flyby_anderson_probe`): tdot_max gegen |anomaly| — Galileo I `pending` (keine Vorepoche) · Galileo II 0.0070/4.6 · NEAR 0.0041/13.46 · Cassini 0.0063/2.0 · Rosetta 0.0031/1.82 · MESSENGER 0.0168/0.02 — **alle `rift-excluded`** (Register `data/flyby2/anderson-probe-2026-09-28.json`) → der Haus-Riß erzeugt die Anomalie nicht (**Ergebnis 1**). **Lokaler Riss entschieden (numerisch):** `ephemeris_granule_census` (neu) misst die drei Erdbins als Chebyshev mit 32-d-Granulaten, Grad 17; die rekonstruierte Position ist an den Granulat-Grenzen **unstetig** (Mismatch Median 0.044 km, max 0.19 km). Die früher „lokale Leck-Rate" (bis 24 mm/s) liegt exakt auf DE/EPM-Granulat-Grenzen (DE↔EPM dort glatt 0.00007 km/h; die INPOP-Paare springen 0.101 km/h = 28 mm/s) — **Artefakt, nicht physikalisch**; die lokale Schranke ist damit nicht bindend, der Rat-Chord ist die physikalische.
- **Blockade:** keine.
- **Braucht:** — descoped mit diesem Befund. Folge-Atom (nicht Mountain-Feder): die ODF-Primärroute (DSN-Doppler) ist die Messung selbst (Sensory) — `--verdict https://pds-ppi.igpp.ucla.edu/`, `https://pds-geosciences.wustl.edu/`, `https://www.cosmos.esa.int/web/psa/rosetta`.

### Ephemeriden-Bins — Unstetigkeit an den 32-d-Granulat-Grenzen (Ursache geheilt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner Ephemeriden-Re-Manifest-Lauf (kernel-flatten / `ephemeris_compiler`) → dann `ephemeris_granule_census` gegen die neuen Bins.
- **Lage:** `ephemeris_granule_census` (gemessen 2026-09-30 via `cargo run -p omegaflow-measure --bin ephemeris_granule_census`): DE441 Erde 346877 Granulate, INPOP 684, EPM 4875 — je 32.0 d Halbbreite, Chebyshev-Grad 17; die rekonstruierte Position sprang an den Grenzen um Median 0.044 km (max 0.19 km). **Ursache (gemessen):** `src/mathematikerin/least_squares.rs::solve_normal_equations` reduzierte den RHS nicht mit der Matrix (Vorwärtselimination nur auf `ata`; `back_substitute` mit unverändertem `atx/aty/atz`) → der Chebyshev-Fit lag ~40 m neben der Funktion. **Geheilt** (RHS wird mitgeführt; Test `normal_equations_reduce_the_rhs_with_the_matrix`, Fit-Grenz-Test `granule_fits_meet_at_the_shared_boundary`; Verifikation `ephemeris_granule_census --selftest` = **max 0.00003 m** Grenz-Differenz (nach Mean-Zentrierung der Samples; der f64-Boden bei 1.5e11 m ist 1 ULP = 0.033 mm), mit dem Solver-Bug ~44 m). **Format-Option (nicht gebaut):** eine per-Granulat-Referenzposition (Offset-Koeffizienten, SPK-Standard) senkte die Grenz-Differenz auf ~0.1 µm — die Quelle ist aber selbst bei ~1.5e8 km f64-quantisiert (~33 µm), also kein Genauigkeitsgewinn, nur Glattheit (falsche Präzision unter der Quellenwahrheit). Als Architektur-Punkt geführt: Trigger = gemessener Konsumentenbedarf < 33 µm; Bau = Format-Bump + Parser/Konsumenten + Re-Manifest, Operator-/Rats-Wort.
- **Blockade:** die CDN-Bins sind mit dem fehlerhaften Solver kompiliert — der Fix greift erst nach Re-Manifest.
- **Braucht:** die Ephemeriden-Bins neu kompilieren/manifesten (kernel-flatten + die `ephemeris_*`-Compiler); danach Census gegen die neuen Bins (Mycelium-Akt, siehe `## An mycelium`).

### Ephemeriden-abhängige Messungen — nach dem Re-Manifest nachziehen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner Ephemeriden-Re-Manifest-Lauf (kernel-flatten / `ephemeris_compiler`).
- **Lage:** (gemessen 2026-09-30 via `ephemeris_granule_census` + `sread docs/paper/*`) die aktuellen CDN-Bins tragen den ~40-m-Fit-Fehler des Solver-Bugs (siehe oben). **Betroffen (Signal ≤ ~200 m):** `eclipse_shadow_probe` Haus-Cross-Check (`docs/paper/eclipse-clock-worldlines.md:45`: DE442↔INPOP19a Erde 0.064 km, Sonne 0.164 km, Mond 0.168 km — **genau die Größenordnung des Bugs**, also möglicherweise ganz Artefakt statt Haus-Riß; die submeter-„one voice" de440/de442 kann common-mode halten — ungeprüft); `ephemeris_house_gate` (Δ DE↔INPOP 0.19 km, Anderson); `flyby_anderson_probe`. **Riss am Siegel:** die präregistrierten Trajektorie-Hashes `flyby-path-2-preregistration.md:21/23` (`ephemeris_juice.bin aeb3c82f…`, Europa Clipper `dae553fb…`) sind Hashes von mit dem Bug kompilierten `horizons_compiler`-Bins — ein Re-Manifest ändert sie; das Blatt nennt selbst die Regel „die neue Version wird benannt", aber nach dem Flyby ist ein Neu-Siegeln post-hoc. **Robust (Signal ≥ km, Skalenargument):** Neptune-Rift (10³–10⁶ m), Uranus-Rift (0.36–1.57e6 m), KBO-Residuum, Dunkel-Materie-Beschleunigungsresiduum (glatter Fit-Fehler → 2. Ableitung ≪ 1e-8 m/s²-Boden), Signal-Konus (AU), Galileo-Rotor (CK/Daf, nicht die Bins).
- **Blockade:** der Re-Manifest — `de44-cdn` und `inpop-epm-cdn` bauen per `workflow_dispatch` + Monats-`schedule` (cron 2. des Monats) aus `main`; der Fix ist noch nicht auf `main` (kein Commit), also fährt der nächste Lauf sonst weiter den Bug.
- **Braucht:** Fix committen → `gh workflow run de44-cdn` + `gh workflow run inpop-epm-cdn` (+ die Missions-Ephemeriden-Workflows) → dann `eclipse_shadow_probe` (2017 + 2024), `ephemeris_house_gate` (6 Anderson-Epochen), `flyby_anderson_probe`, `ephemeris_granule_census` neu laufen und die Verdikte fortschreiben (Verschiebung > genannte Unsicherheit = betroffen).

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` — σ-Metrik-Kette (3 Marker); Trigger JUICE In-Situ /
  Δ publiziert, `flyby_ephemeris_gate` (CI).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — Holdings-Inventur; Schritt 2
  gemessen (Holdings + Repo-`data`), die Dedup-Entscheidung in `## An future`.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` — Relevanz-Erstpass; die
  Pending-Einträge sind im `dead_sources.φ`-Register disponiert, die Prosa bleibt als
  datierte Messung.
- `docs/surveys/survey-raetsel-bestand.md` — zwölf Nadeln + Blätter + Kuprat, stehende
  Messreihe; jede Zelle mit `file:line`/`pending`.

## Register-Träger (eigene)

- `phi/harvest.φ` `format hapi_csv` (DAS2 Iowa, Asset `das2_iowa_…`) — `asset fehlt`;
  Workflow `das2-iowa-cdn.yml` gebaut (Mycelium-214), dispatcht `36741690825` — `present`
  erst nach grünem Lauf.
- `phi/harvest.φ` `format pds3_img` (Chandrayaan-1 Mini-RF, Asset `pds3_img_…`) — `asset fehlt`;
  Workflow `pds3-img-cdn.yml` rot (M3-`.HDR` HTTP 403, `pds-imaging.jpl.nasa.gov`) — Route
  messen.
- `phi/harvest.φ` `format pds3_fixed_width_darts` (Akatsuki VCO-rs) — `asset fehlt`; Workflow
  `pds3-fixed-width-darts-cdn.yml` gebaut (Mycelium-214), dispatcht `36741697524`.
- `phi/harvest.φ` `format pds4_binary` (ExoMars TGO ACS / PSA) — `asset fehlt`; Workflow läuft.

## An mycelium

Origin: mountain folge216 (Faltung der adressierten Blöcke mycelium-folge214 + Antwort auf die Ephemeriden-generic-Blockade).

- **Ephemeriden-Re-Manifest nach dem Solver-Fix:** `solve_normal_equations` (`src/mathematikerin/least_squares.rs`) reduzierte den RHS nicht mit der Matrix → jeder mit `chebyshev_fit` kompilierte Ephemeriden-Bin trägt einen ~40-m-Fit-Fehler und springt an den 32-d-Granulat-Grenzen (gemessen via `ephemeris_granule_census`; nach dem Fix `--selftest` 0.00003 m = f64-Boden). **Braucht:** die Ephemeriden-Assets neu kompilieren/manifesten (`kernel-flatten` / `ephemeris_compiler`), dann `ephemeris_granule_census` gegen die neuen Bins.
- **Ephemeriden generic (new_horizons/voyager1/voyager2) — deine Feder (travelnder Punkt):** die drei `url`-Zeilen `phi/sources.φ:15747/15950/15957` stehen auf `ssd.jpl.nasa.gov-horizons`; die generischen Assets bleiben 976-B-Placeholder, den Kernel trägt der `horizons_compiler --long`-Lauf. Der Placeholder ist nicht gemessen (`pending`, 0 honored). **Braucht:** die zugelassene Quelle der drei auf den `_long`-Lauf setzen (`url`/`compiler`) oder die generische Zeile zugunsten `ephemeris_*_long` verwerfen; danach Mountain die generischen Placeholder-Zeilen `descoped`.
- **itokawa:** die NAIF-id `2025143` **steht** in `tools/harvest/src/bin/horizons_compiler.rs:637` (Eintrag in `974e88030`, Mountain-214) — die Block-Zeile „fehlt in der Liste" ist gemessen stale; der Re-Dispatch (`kernel-flatten`) bleibt deine Feder.
- **Kaguya-Idempotence-Audit:** `pds3-binary-cdn.yml` trägt den `force`-Input (`:10/:29`); die Prüfung, ob weitere `*-cdn.yml` denselben Block für geänderte Compiler brauchen, ist CI-Feder (Mycelium), nicht Mountain.
- **CNSA `moon.bao.ac.cn`/`nssdc.ac.cn` — Descope-Vorschlag nicht getragen (gemessen):** `--verdict` 2026-09-30: moon.bao.ac.cn HTTP 206 direkt, nssdc.ac.cn HTTP 206 direkt — erreichbar, das Konto ist Operator-Hand (`released`); die Alternativrouten (`pds.wh.sdu.edu.cn` `:444`, CDS/Aladin `:440`, Zenodo `:424`) sind getrennt registriert und komplementär, kein Ersatz für die CNSA-Primärquelle. Die Zeilen bleiben `released`.
- **PRADAN `phi/blocked_sources.φ:393`** — bereits geheilt: die Note trägt „OIDC-Flow browserlos verifiziert (future-159, Connector f7bfce9b7)", kein „Download end-to-end offen" mehr.
- **KARI KPDS** — bereits `descoped` (`phi/blocked_sources.φ:455-457`, gemessen: kein maschinenlesbarer Endpunkt).
- **kuprat Family-Tag** — die vier Kanäle sind als `substance`-Witnesses admitiert (`witnesses.φ:120-142`); kein Compiler setzt `tag kuprat`, kein Mountain-Akt.
- **DAS2-Reader / Ephemeriden-URLs / DAS2+Akatsuki-Workflow** — erledigt, keine offene Hunk.

## An river

Origin: mountain folge216 (Solver-Fix + Re-Manifest; das Sonne-Erde-Blatt/Eclipse-Paper ist deine Feder).

- **Finsternis-Haus-Cross-Check ist vom Solver-Bug belastet:** `docs/paper/eclipse-clock-worldlines.md:45` meldet DE442/DE441↔INPOP19a Erde 0.064 km / Sonne 0.164 km / Mond 0.168 km — dieselbe Größenordnung wie der ~40-m-Fit-Fehler, den `solve_normal_equations` in **jede** kompilierte Ephemeriden-Bin geschrieben hat (geheilt, aber die CDN-Bins sind noch alt). Bis zum Re-Manifest sind diese Zahlen `pending`, kein Haus-Verdikt. Die sub-meter „one voice" zwischen de440/de442 kann common-mode sein (gleiche Funktion, gleicher Fit-Fehler hebt sich) und halten; DE↔INPOP ist der verletzliche Punkt. Nach dem Re-Manifest ziehe ich `eclipse_shadow_probe` (2017 + 2024) neu — bitte die Zahlen im Paper bis dahin nicht als gemessen führen.

## An future

Origin: mountain folge216 (Registry-first aus future-folge161 ist in folge215 bereits je Asset gemessen und beantwortet — `## An future` folge215). Die Operator-Queue-Einträge aus folge215 (Frühwarn-Dienst, Holdings-Dedup) bleiben bei dir.

## Burn: open 0.0000 · close 0.3365 · cap 0.40 · Grund: operator-getriebenes Mehr-Nachrichten-Atom (drei CI-Test-Heilungen, Solver-Fix + Re-Manifest-Analyse, Anderson-Verdikt), Commit-Wort gegeben

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
