<!--
  title: Handover — Forschung-Folge 45 (2026-09-16)
  session: Forschung-Folge 45
  class: handover
  date: 2026-09-16
  sha256: 124b339be98c89022fb3cad8f0ec4ffca0bddb70fb00f16f36ca3f11aa53020f
  status: live
-->
# Handover — Forschung-Folge 45 (2026-09-16)

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

- Der Shard-Schreiber ist gebaut (Rat 2026-09-16, Design C: kein Konsumenten-Code,
  jeder Shard ist seine eigene `SourceConfig`, das Feld verschmilzt nach
  (name, epoch) in den Matrix-Ringen; Invariante: halboffene, strikt disjunkte
  TDB-Bereiche). **Offen: die Shard-Namen sind datenabhängig und erst nach dem
  CI-Lauf messbar.** Der beauftragte Lauf `35138707030` wurde **cancelled / 0
  Jobs** (10 s, kein Compiler erreicht); Nachfolger `35138720790` (pending) und
  `35115390613` (in_progress). (Schritt: `gh run view 35138720790 --log` bzw.
  `35115390613` lesen; die Compiler drucken den `phi/sources.φ`-Block, die
  Einzel-`url`-Zeilen bei `phi/sources.φ:6685/6691/6697/6703` durch die N
  gemessenen Shard-Blöcke ersetzen — die im Vorhandover genannten 6687–6697
  trugen `at earth`/`ttl`/`field`.)

## Paper / §4.5 — Monats-`posfrac`

- **76-%-Richtungsstatistik — erledigt.** Die volle 613er-Matrix trägt **74.4 %
  (456/613)** (Artefakt `aia-ladder-2015`, Lauf `35134530887`); in
  `docs/paper/corona-heating-ladder.md:316` gesetzt.
- **Monats-`posfrac`** — der Erzeuger steht: `aia_ladder_probe` druckt seit
  2026-09-16 einen Monats-Block (`JJJJ-MM | N events | posfrac F (pos/tot) |
  mean D`), `cargo check` 0/0. (Schritt: Probe auf dem 613er-2015-Satz in CI
  laufen lassen, den Monats-Block aus dem Artefakt lesen und `MIN–MAX` in
  `corona-heating-ladder.md:316` setzen — die Zeile bleibt bis dahin pending.)

## CI / geteilter Zustand

- **CI-`format` rot @ `2f2f3437`** — die forschung-eigene `aia_ladder_probe.rs`
  ist jetzt rustfmt-clean (fn_call_width-Bruch an `:486`); fremd bleiben 23
  Dateien: `src/gate/commit_gate.rs:540`,
  `src/archivar/{dl3,fetch,fits,flac,ifms_agc,tests,units,vtscat}.rs`,
  `src/mathematikerin/s2.rs`,
  `tools/measure/.../{band_amplitude,corona_event,pcmci_class_benchmark,s2_weberin,trishuli_gauge}_probe.rs`,
  `tools/utils/.../archive_search.rs`,
  `tools/harvest/.../{champ_plpt,ephemeris,mars_express_odf,mpcobs,rosetta_odf,tap,voyager_odr}_compiler.rs`.
  Die Post-Zeile `post.md:18` bleibt. (Schritt: fremde Linien formatieren ihre
  Dateien.)
- **`docs/zustand/external-state.md`** von der Bau-Linie bearbeitet; der
  CI-Status-Eintrag ist @ `77c8be18` stehengeblieben. (Schritt: @ HEAD
  fortschreiben, sobald die Datei frei ist.)

## Positionslinien / Ephemeriden

- **Sonne-Anker-Fix verifiziert** — Lauf `35138709981` (s2-weberin-probe)
  completed: die Sonne erscheint namentlich als einzige Ausnahme vom
  sub-resolution-Verdikt (α_sun = 2.110839 rad > π/64, SSB-Abstand 7.995568e8 m).
  Die Anker stehen gemessen bei `phi/sources.φ:2223` (`at neptune_c`) und
  `:2358` (`at uranus_c`) — nicht 2149/2284.

## Paper / Recherche — gemessene offene Routen

- **Sturzflut-Tibet**: Flut-Peak bleibt pending (kein offener co-lokaler Pegel;
  IRIS/FDSN-Query am Kollabpunkt 28.271/85.515, r 0.5°/1.0°, 2026-08-20…30 =
  leer). Der Abfluss-Hauptweg bleibt bezahlt (DHM gemessen); neue offene Route
  ist die GloFAS-Reanalyse im EWDS
  (`https://ewds.climate.copernicus.eu/datasets/cems-glofas-historical`, HTTP
  200, **Modell**-Daten, CDS-Key). (Schritt: GloFAS als Modell benennen oder
  streichen.)
- **h0-lines-register**: der TAP-Leg-Zustand hat sich geändert — der ESA-Host
  `gea.esac.esa.int/tap-server/tap/sync` antwortet jetzt HTTP 200 (Register:
  „000" am 2026-09-13). (Schritt: `cepheid_parallax_weigh` gegen den ESA-Host
  re-laufen lassen.)
- **Neptun-Bau-Linie**: Datenroute offen gemessen — NAIF `de440s.bsp` HTTP 200,
  7.55 MB. (Schritt: Compiler + `sources.φ`-Registrierung + kernel-flatten.)
- **JWST disequilibrium**: §6 korrigiert — HD 80606b (arXiv 2407.12456,
  NIRSpec/G395H) als gemessenes saisonales JWST-Gegenstück (nicht-biosignaturhaft)
  eingetragen; die O2/O3- und Red-Edge-Detektion bleiben pending (Sweep
  2026-09-16 ohne Detektion). ETH-Deposit `10.3929/ethz-c-000797709` und OpenAlex
  liefen in HTTP 429. (Schritt: Retry nach Cooldown.)

## Paper / Präregistrierung

- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE
  28./29.09., Clipper 03.12. (Schritt:
  `docs/paper/flyby-path-2-preregistration.md`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
