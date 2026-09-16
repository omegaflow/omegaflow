<!--
  title: Handover — Forschung-Folge 40 (2026-09-16)
  session: Forschung-Folge 40
  class: handover
  date: 2026-09-16
  sha256: dfb3bffa7d74bed820b16c9899cb2f21abd76c185508e5a4c10e0c42e372e668
  status: live
-->
# Handover — Forschung-Folge 40 (2026-09-16)

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

## Paper / Zahlen — §4.5-Korrektur

- **corona-heating-ladder §4.5 — 2014/2015 nicht reproduziert.** Der korrigierte
  GOES-Lauf 35097506781 (drei Jobs, 2013/14/15) trifft 2013 exakt (524
  Ereignisse, fam 1.7131e-1), aber 2014/2015 nicht (1350 vs. 1019; 613 vs. 281
  Ereignisse; fam 1.9058e-1 vs. 1.96e-1, 1.7789e-1 vs. 1.75e-1; der 211→335-Peak
  liegt bei 144 s statt ~96 s). Die der-grat-Zelle schließt mit dem gemessenen
  Mismatch (`docs/blatt/blatt-der-grat.md:150-158`). (Schritt: entscheiden, ob
  §4.5 eine Korrektur trägt oder die Trigger-Definition differiert —
  `docs/paper/corona-heating-ladder.md:302-304`.)

## h0 / CMB

- **CMB clik/CosmoMC-Eigenevaluation** — die Chain-Statistik ist gewogen, der
  Likelihood-Code-Lauf selbst offen. (Schritt: Bau eines `clik`/CosmoMC-Atoms
  oder descopen nach Messung; `docs/paper/h0-lines-register.md` §„Named pending
  points" (1).)

## Quellen-Pass

- **CHAMP/GFZ-ISDC PLPT** — gemessen: kein Harvest-Bin (`archive_search plpt
  --root tools` findet nur `laic_probe.rs:47`; `sgrep -i plpt phi/sources.φ`
  absent). (Schritt: `grind-pro`, Harvest-Bin.)
- **`ephemeris_juice_cog.bin`** — gemessen: `kernel-flatten` run 35123314530 =
  failure, Job `bodies` exit 1, der JUICE-Step wurde übersprungen; Asset HTTP
  404 absent. (Schritt: den `bodies`-Fehlergrund messen und fixen, dann
  re-dispatch.)

## Bande-Split / Sonden-ODF

- **7 planetare ODF** — gemessen (`planetary-odf-cdn` 35097513235): odyssey exit
  1 (Volume-URLs `odrs_0297/0298/0299` HTTP 404), mro canceled (runner
  shutdown), mars_express + rosetta 4-h-Timeout; juno/magellan/mgs/messenger
  success. (Schritt: die odyssey-Volume-URL korrigieren, Timeout/Retry für
  mro/mars_express/rosetta.)

## Positionslinien / Ephemeriden

- **Horizons-Residual** — gemessen: keine Schwelle in Workflow oder Probe; der
  Residual wird roh gedruckt (neptune max 457925.194 m). (Schritt: entscheiden,
  ob eine Schwelle nötig ist oder der rohe Chebyshev-Residual der Befund bleibt —
  `ephemeris-horizons-check.yml`.)
- **Sonne-Anker-Fix verifizieren** — drei `ephemeris_binary`-Quellen trugen
  `at sun`; `phi/sources.φ:2149` → `at neptune_c`, `:2284` → `at uranus_c`
  (2026-09-16). (Schritt: `s2-weberin-probe` nach dem Push neu dispatchen; die
  Sonne muss im sub-resolution-Verdikt erscheinen.)

## Die Weberin

- **Vlies-Konsumption** — gemessen: `.vlde` fehlt in `src/archivar` und
  `phi/sources.φ`; die Manifestation `vlies_density.vlde` (sha256 `7bf53447…`)
  steht. (Schritt: Reader + `format vlde`.)

## Paper / Präregistrierung

- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE
  28./29.09., Clipper 03.12. (Schritt:
  `docs/paper/flyby-path-2-preregistration.md`.)
- **JWST disequilibrium** — O2/O3-, red-edge-, saisonale Kanäle `pending`; die
  Abwesenheit ist gemessen (Sweeps 2026-09-16), bis eine Detektion samt Spektrum
  ins Register tritt. (Schritt: `docs/paper/jwst-disequilibrium-survey.md` §6.)

## CI / geteilter Zustand

- **CI-Status** — gemessen: für HEAD `e5cb31cf` liegt kein `ci-check`-Lauf vor;
  @ `07a91a99` sind clippy + format rot (`flac.rs:498`, `ifms_agc.rs:101`,
  `galileo_odr.rs:87`; fmt-Diffs `ifms_agc`/`flac`/`fits`/`dl3`). (Schritt: den
  `docs/zustand/external-state.md`-Eintrag @ HEAD fortschreiben; die roten
  Stellen gehören fremden Linien.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
