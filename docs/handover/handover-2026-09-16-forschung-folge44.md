<!--
  title: Handover — Forschung-Folge 44 (2026-09-16)
  session: Forschung-Folge 44
  class: handover
  date: 2026-09-16
  sha256: c79b534a4b8f5b34afe180f635215925d4fa625c1cd217ead5590c7622d5e585
  status: live
-->
# Handover — Forschung-Folge 44 (2026-09-16)

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

## ODF-Bande-Split — Konsument (härtester undatiert)

- **ODF-Shard-Konsument** — der Shard-Schreiber ist gebaut (Rat 2026-09-16,
  Design C: kein Konsumenten-Code, jeder Shard ist seine eigene `SourceConfig`,
  das Feld verschmilzt nach (name, epoch) in den Matrix-Ringen; Invariante:
  halboffene, strikt disjunkte TDB-Bereiche). **Offen: die Shard-Namen sind
  datenabhängig und erst nach dem CI-Lauf messbar.** (Schritt: `gh workflow run
  planetary-odf-cdn.yml`; die Compiler drucken den `phi/sources.φ`-Block — `gh
  run view <id> --log` lesen, die Einzel-`url`-Zeilen `odyssey_odf`/`mro_odf`
  in `phi/sources.φ:6687-6697` durch die N gemessenen Shard-Blöcke ersetzen.)

## Paper / §4.5

- **76-%-Richtungsstatistik auf dem 613er-Satz.** Lauf `35134530887` (SHA
  `0e2fc1a2`, trägt die Posfrac-Ausgabe) stand auf pending; die 613er-Matrix mit
  Positiv-Fraktion liegt in seinem Artefakt `aia-ladder-2015`. (Schritt: `gh run
  download 35134530887 --name aia-ladder-2015`, die Zahl in
  `docs/paper/corona-heating-ladder.md:316` ersetzen.)
- **Monats-`posfrac 0.56–1.00`** (Paper `:316`) hat keinen Erzeuger —
  `aia_ladder_probe` bricht nicht nach Monaten auf. (Schritt: Monats-Bin im Probe
  ergänzen oder die Monats-Klausel streichen.)

## CI / geteilter Zustand

- **CI-`format` rot** — fremde unformatierte Dateien `src/gate/commit_gate.rs:540`,
  `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`,
  `tools/utils/src/bin/archive_search.rs` (Post-Zeile „An alle Linien"). Die
  forschung-eigene `aia_ladder_probe.rs` ist jetzt rustfmt-clean (Join aus
  `ci-check` run 35091175017 angewandt). (Schritt: die fremden Dateien ihren
  Linien überlassen, danach die Post-Zeile löschen.)
- **`docs/zustand/external-state.md`** ist von der Bau-Linie aktiv bearbeitet.
  (Schritt: @ HEAD fortschreiben, sobald die Datei frei ist — der CI-Status-Eintrag
  ist @ `77c8be18` stehengeblieben.)

## Positionslinien / Ephemeriden

- **Sonne-Anker-Fix verifizieren** — `phi/sources.φ:2149` → `at neptune_c`,
  `:2284` → `at uranus_c` (2026-09-16). (Schritt: nach Push `s2-weberin-probe`
  dispatchen; die Sonne muss im sub-resolution-Verdikt erscheinen.)

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
