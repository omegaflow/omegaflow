<!--
  title: Handover — Forschung-Folge 43 (2026-09-16)
  session: Forschung-Folge 43
  class: handover
  date: 2026-09-16
  sha256: 218b1fa16aca0676d881b9159abf5fbb71d86898a318888f7c2952c95578c058
  status: live
-->
# Handover — Forschung-Folge 43 (2026-09-16)

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
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatiert); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Bande-Split / ODF — >2-GiB-Assets (härtester undatiert)

- **odyssey (3 367 454 840 B) und mro (extrapoliert ~12.7 GiB) sprengen GitHubs
  2-GiB-Limit pro Release-Asset** (gemessen: `HTTP 422: size must be less than
  2147483648`; `odyssey_odf_compiler.rs:234-236`). Die Compiler harvesten korrekt
  (parallelisiert, committet `21d94a30`); nur die Manifestation passt nicht in ein
  Asset. **Rat-Verdikt (2026-09-16):** Zeit-Bereichs-Split in **vollständige
  PODF-Shards** — jeder Shard ein eigenes `write_podf_bin`-Ergebnis (eigener
  Header, eigene count, einzeln `parse_podf_bin`-bar), benannt nach dem gemessenen
  Bereich `odyssey_odf_t<t0>_<t1>.bin`, Ziel 2³⁰ B je Shard, harte Grenze 2³¹ B;
  **keine** Kompression (bricht den fixed-stride-Offset `8 + i·72` und löst das
  mro-Limit nicht). Konsument: N `url`-Zeilen mit gleichem `format <name>_odf` in
  `phi/sources.φ`; `main_flow`/`extract.rs` gruppieren nach Format, parsen jeden
  Shard und konkatenieren die Records in Zeitreihenfolge. Präzedenz:
  `mpcobs_compiler.rs:179` (`SHARD_BUDGET = 1 << 30`) + `mpcobs-shard-cdn.yml`.
  (Schritt: Shard-Writer in `src/archivar/odf.rs`, die vier Compiler darauf
  verdrahten, `extract.rs:44`/`main_flow.rs:2114`-Gruppierung, `sources.φ`-Zeilen,
  Workflow-Prüfung je Shard — Schreiber und Konsument im selben Atom.)

## Paper / §4.5

- **76-%-Richtungsstatistik auf dem 613er-Satz nachmessen.** `aia_ladder_probe`
  gibt jetzt je Paar die Positiv-Fraktion am Peak-Lag aus
  (`| positive {:.1}% ({}/{})`, 2026-09-16); die Zahl in
  `docs/paper/corona-heating-ladder.md:316` trägt noch die pending-Marke der
  Teilmenge (281). (Schritt: nach Push `gh workflow run aia-ladder-probe.yml`,
  `gh run download` der 2015-Matrix, die Zahl ersetzen.)
- **Die Monats-`posfrac 0.56–1.00`** (Paper `:316`) hat noch keinen Erzeuger —
  `aia_ladder_probe` bricht nicht nach Monaten auf. (Schritt: Monats-Bin im Probe
  ergänzen oder die Monats-Klausel streichen.)
- **Abstract-Spanne** war nicht re-derivierbar; ersetzt durch die gemessene
  Neun-Werte-Spanne `0.35–0.97 × fam` aus Lauf-Artefakt 35097506781 (Paper
  `:18`, version 11, sha256 aktualisiert) — erledigt, hier nur als Messherkunft.

## CI / geteilter Zustand

- **CI-`format` rot** @625452e5 — fremde unformatierte Dateien (Post-Zeile „An
  alle Linien"): `src/gate/commit_gate.rs:540`,
  `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`,
  `tools/utils/src/bin/archive_search.rs`. Diese Session hat
  `aia_ladder_probe.rs` um die Positiv-Fraktion erweitert (rustfmt-Diff aus
  `ci-check` run 35091175017 noch nicht angewendet). (Schritt: rustfmt-Diff
  anwenden, danach die Post-Zeile löschen.)
- **`docs/zustand/external-state.md`** ist von der Bau-Linie aktiv bearbeitet
  (der TE-Gate-Eintrag trägt dort den committeten Arx-Switch `1337c6c1`). (Schritt:
  @ HEAD fortschreiben, sobald die Datei frei ist — CI-Status-Eintrag ist @
  `77c8be18` stehengeblieben.)
- **`haug-fig1..8.png`** liegen untracked und ownerlos im Wurzelverzeichnis
  (Bezug: der offene „NSE/Haug"-Posten im Postfach-Eintrag). (Schritt: einer
  Linie zuordnen oder löschen — nicht ownerlos liegen lassen.)

## Positionslinien / Ephemeriden

- **Sonne-Anker-Fix verifizieren** — `phi/sources.φ:2149` → `at neptune_c`,
  `:2284` → `at uranus_c` (2026-09-16). (Schritt: nach Push `s2-weberin-probe`
  neu dispatchen; die Sonne muss im sub-resolution-Verdikt erscheinen.)

## Paper / Präregistrierung

- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE
  28./29.09., Clipper 03.12. (Schritt:
  `docs/paper/flyby-path-2-preregistration.md`.)
- **JWST disequilibrium** — O2/O3-, red-edge-, saisonale Kanäle `pending`; die
  Abwesenheit ist gemessen (Sweeps 2026-09-16), bis eine Detektion samt Spektrum
  ins Register tritt. (Schritt: `docs/paper/jwst-disequilibrium-survey.md` §6.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
