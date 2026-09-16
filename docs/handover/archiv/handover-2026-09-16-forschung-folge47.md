<!--
  title: Handover — Forschung-Folge 47 (2026-09-16)
  session: Forschung-Folge 47
  class: handover
  date: 2026-09-16
  sha256: 13782cf9c4a1290674c13233846cb957eed8e9c954e1ad35cb060eef8b909219
  status: live
-->
# Handover — Forschung-Folge 47 (2026-09-16)

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
der härteste undatierte); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## ODF-Bande-Split — Konsument (härtester undatiert)

- Der Lauf `planetary-odf-cdn` `35139594201` ist pending (gemessen 2026-09-16,
  headSha 43af521f). Die Shard-Namen sind datenabhängig und erst nach dem Lauf
  messbar. **Anker in dieser Session korrigiert** (gemessen am Baum): von den vier
  planetaren ODF-Blöcken tragen nur zwei einen Shard-Zweig — `mro_odf`
  (`phi/sources.φ:6723–6727`) und `odyssey_odf` (`6729–6733`); `magellan_odf`
  (`6711–6715`) und `mgs_odf` (`6717–6721`) schreiben je eine Einzeldatei und
  bleiben unverändert. (Schritt: `gh run view 35139594201 --log` lesen, den
  gedruckten `phi/sources.φ`-Block nehmen und **jeden ganzen 5-Zeilen-Block**
  `url/format/at earth/ttl/field` + Leerzeile durch die N gemessenen Shard-Blöcke
  ersetzen — nicht nur die `url`-Zeile; `refuse_shard_overlaps` in
  `src/archivar/parse.rs` verweigert jede Überlappung gleichen `format`.)

## Paper / §4.5 — Monats-`posfrac`

- Lauf `35140498561` (`aia-ladder-probe`) ist in_progress (gemessen 2026-09-16,
  headSha 43af521f). Das frühere Artefakt `35134530887` trägt den Monats-Block
  noch nicht. (Schritt: `gh run download 35140498561 --name aia-ladder-2015`;
  den Monats-Block `JJJJ-MM | N events | posfrac F (pos/tot) | mean D` lesen und
  `MIN–MAX` in `docs/paper/corona-heating-ladder.md` setzen — der Monats-Pending-
  Text steht `:317–320`, nicht `:316`; die Zeile bleibt bis dahin pending.)

## Positionslinien / Ephemeriden — `neptune_c` (Zentrum 899)

- Die `de440s`-Linie ist **geschlossen** (gemessen 2026-09-16): Lauf
  `neptune-de440s-cdn` `35143413816` success —   `neptune_ephemeris_compiler
  de440s.bsp --ci-mode` liest target 8 (Neptun), 3425 Granules/Rotationen,
  1 808 616 B, schreibt `data/ssd.jpl.nasa.gov/ephemeris_de440_neptune.bin`;
  die `phi/sources.φ`-Zeile steht (`:2024`, `format ephemeris_de440`,
  `at neptune`, ttl 86400). Offen bleibt die
  **`neptune_c`-Linie (Zentrum 899)**: Zentrum 899 liegt nicht in `de440s` und
  braucht die separate `nep097xl-899.bsp`-Komposition. (Schritt: `nep097xl-899.bsp`
  beschaffen/komponieren, `neptune_center_rift_probe` gegen die drei Häuser
  fahren.)

## CI / geteilter Zustand

- **CI-`format` rot** — fremde unformatierte Dateien (u. a. `src/gate/commit_gate.rs:540`,
  `src/archivar/{dl3,fetch,fits,flac,ifms_agc,tests,units,vtscat}.rs`,
  `src/mathematikerin/s2.rs`, mehrere `tools/…`-Proben/Compiler); die Post-Zeile
  `post.md:18` steht. (Schritt: fremde Linien formatieren ihre Dateien.)

## Paper / Präregistrierung

- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE 28./29.09.,
  Clipper 03.12. (Schritt: `docs/paper/flyby-path-2-preregistration.md`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
