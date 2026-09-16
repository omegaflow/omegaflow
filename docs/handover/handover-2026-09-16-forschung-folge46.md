<!--
  title: Handover — Forschung-Folge 46 (2026-09-16)
  session: Forschung-Folge 46
  class: handover
  date: 2026-09-16
  sha256: ff93ff3e13d9a788affcc9bc21dc852ae67e049d1fc03c07844c85a296f3b26f
  status: live
-->
# Handover — Forschung-Folge 46 (2026-09-16)

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

- Der Lauf `planetary-odf-cdn` `35139594201` ist pending (gemessen 2026-09-16). Die
  Shard-Namen sind datenabhängig und erst nach dem Lauf messbar. (Schritt: `gh run
  view 35139594201 --log` lesen; den gedruckten `phi/sources.φ`-Block nehmen und die
  Einzel-`url`-Zeilen bei `phi/sources.φ:6685/6691/6697/6703` durch die N gemessenen
  Shard-Blöcke ersetzen.) Die Invariante „halboffene, strikt disjunkte TDB-Bereiche"
  ist jetzt als Gate gebaut: `refuse_shard_overlaps` in `src/archivar/parse.rs`
  verweigert jede Überlappung gleichen Namens (`shard_range_of` liest den halboffenen
  Bereich aus dem Shard-Namen; `parse_podf_shard_name` in `src/archivar/odf.rs`) — der
  manuelle Ersatz ist damit bewacht, nicht mehr nur angenommen.

## Paper / §4.5 — Monats-`posfrac`

- Lauf `35140498561` (`aia-ladder-probe`) ist in_progress (gemessen 2026-09-16). Das
  frühere Artefakt `35134530887` trägt den Monats-Block noch **nicht** (gemessen: 0
  `posfrac`-Treffer). (Schritt: `gh run download 35140498561 --name aia-ladder-2015`;
  den Monats-Block `JJJJ-MM | N events | posfrac F (pos/tot) | mean D` lesen und
  `MIN–MAX` in `docs/paper/corona-heating-ladder.md:316` setzen — die Zeile bleibt bis
  dahin pending.)

## Positionslinien / Ephemeriden

- **h0-lines TAP** — der ESA-Host `gea.esac.esa.int/tap-server/tap/sync` antwortet
  HTTP 200 (gemessen, `--verdict` stage 1 direct; der Registerstand „000" vom
  2026-09-13 ist überholt). Es gab keinen CI-Weg für `cepheid_parallax_weigh`; der
  Workflow `.github/workflows/cepheid-parallax-weigh.yml` ist angelegt. (Schritt: nach
  Push `gh workflow run cepheid-parallax-weigh.yml`, Artefakt lesen, gegen N = 1606 /
  0.2619 ± 0.0004 mas abgleichen.)
- **Neptun `de440s.bsp`** — Compiler `neptune_ephemeris_compiler`, Workflow
  `neptune-de440s-cdn.yml` und die zwei `phi/sources.φ`-Blöcke sind gebaut (gemessen:
  32 726 016 B, sha256 c1c7feea…; das Vorhandover nannte 7.55 MB — durch Messung
  widerlegt). (Schritt: nach Push `gh workflow run neptune-de440s-cdn.yml`; `neptune_c`
  (Zentrum 899) liegt nicht in `de440s` und braucht die separate `nep097xl-899.bsp`-
  Komposition.)

## Paper / Recherche — gemessene offene Routen

- **JWST disequilibrium** — ETH-Deposit `10.3929/ethz-c-000797709` via DataCite
  gemessen (Titel „Observing spatial and temporal variations…", 429 weg; Jahr pending);
  die `doi.org`-Reachability und OpenAlex bleiben HTTP 429 (direct + Proton). (Schritt:
  Retry nach Cooldown; für die `doi.org`-Route Operator-Consent für einen Proton-Exit.)
- **Sturzflut-Tibet** — GloFAS als `derived-model` in `phi/declined_sources.φ`
  eingetragen (gemessen). Der Flut-Peak bleibt pending auf der gemessenen Abwesenheit
  (IRIS/FDSN-Query am Kollabpunkt 28.271/85.515 leer = 0 honored), nicht auf der
  Modell-Route.

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
