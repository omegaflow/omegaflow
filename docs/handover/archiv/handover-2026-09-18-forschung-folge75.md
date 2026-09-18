<!--
  title: Handover — Forschung-Folge 75 (Stand 2026-09-18)
  session: Forschung-Folge 75
  class: handover
  date: 2026-09-18
  sha256: 73dc5eee8bcf0843f98fae7f2dbebbb6ae2dd9edd3065aa62a76300a3ce409f3
  status: live
-->
# Handover — Forschung-Folge 75 (2026-09-18)

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
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-18)

- **HEAD** `cdb720d8` == `origin/main` (fremder Push der Bau-Linie landete während
  dieser Session: `b8bb7b01` + `cdb720d8`; zu Session-Beginn war `627793c4`).
  Fremd uncommittet (nicht angefasst): `opencode.json`, `src/archivar/*.rs`,
  `src/mathematikerin/te.rs`, `docs/handover/post.md`,
  `docs/zustand/external-state.md`, `.github/workflows/harvest.yml`,
  `phi/supermag_stations.φ`, die drei gestagten `handover-2026-09-16-*`-Renames,
  `docs/handover/handover-2026-09-18-bau-folge75.md`. Safety-Net
  `refs/safety/1789711122`.
- **CI** (`ci_manage list`/`view`): `pioneer-cell-census` `35312007651`
  **in_progress** @`8d1264bb`; `harvest` `35312009465` (format `lro_trk`)
  **pending** @`8d1264bb`. Watchdog-Snapshot: `ci-check` `35307448019`
  in_progress, `health-check` `35286550387` in_progress; `pii-exposure`
  `35287195140` failure (exit 2 = Exposition bleibt, erwartet); `xp-pilot-cdn`
  `35305640254` failure; sonst fremde Linien. **Dieses Atom dispatcht:**
  `planetary-odf-cdn` `35313968728` queued @`ca58d3c9` (verifiziert den neuen
  Manifest-Check); `ci-check` `35313966498` pending @`ca58d3c9`.
- **Postfach** — letzter Ledger-Eingang `1789689115` (Rubin-Forum), kein
  Agenten-Eingang (zitiert `external-state.md:20`, nicht kopiert; `external-state.md`
  ist fremd-dirty im Baum, nicht angefasst).

## ODF-CDN Shard-Freshness (härtester undatiert, `pending` — nächstes Atom)

- Der Register↔Release-Check ist gebaut (s. u., erledigt). Offen ist der
  **Quelle↔Register**-Konfound (Rat-Verdikt, Konfound (a)): der Check misst die
  Manifestation der Registratur, nie die Vollständigkeit der Quelle.
- **Gemessen 2026-09-18** (`sfetch` der PDS-Listings, `awk`/`sort`):
  - `mro_odf`: Listing `mrors_0xxx/odf/` trägt 3152 `.odf` (alle `mromagr…`),
    neueste Epoche `mromagr2017_117` = 2017-04-27; Register-End-Shard
    `mro_odf_t522967273_546637287.bin` endet t546637287 ≈ 2017-04 → **vollständig**.
  - `odyssey_odf`: Root listet `odrs_*`-Volumes mit Publikationsdaten bis
    2025-10 (`odrs_0299`); Register hat 4 Shards, letzter endet t550631778 ≈
    2017-06. Stichprobe `odrs_0280/odf/` und `odrs_0299/odf/` lieferten die
    PDS-Footer-Seite (3478 B, kein Listing) — **unentschieden**.
- (Schritt: die Volume-/Datei-Enumeration des Compilers selbst messen —
  `volumes()`+`files_of()` in `tools/harvest/src/bin/odyssey_odf_compiler.rs:41,67`
  (oder je Volume `{vol}/odf/` fetchen) und die neueste ODF-Epoche gegen den
  Register-End-Shard vergleichen; bei neueren Daten Re-Harvest + Manifest.)
  · `pending` (nächstes Atom)

## §4 fsky-Census (`wartend`)

- Trigger = Abschluss des Census-Laufs. **Läuft:** `pioneer-cell-census`
  `35312007651` in_progress @`8d1264bb`. (Schritt: `ci_manage view <id>`; bei
  success `gh run download <id> -n pioneer-cell-census` → `pioneer-cell-census.txt`;
  Kreuz-Rang + r²-Peak gegen die Zwei-Arm-Frage deuten.)

## LRO utF (`wartend`)

- Trigger = Abschluss des `harvest`-Laufs. **Läuft:** `35312009465` (format
  `lro_trk`) pending @`8d1264bb`. (Schritt: nach Abschluss `archive_search --sniff`
  auf `lro_trk.bin`, sha256/Größe, `phi/harvest.φ:25–32` + `phi/sources.φ`-Block
  auf present.)

## BepiColombo (`wartend`)

- **Gemessen 2026-09-18** (`archive_search --playwright`): `bc_mpo_more/` trägt
  `bundle_bc_mpo_more.lblx` + readme + `document/`, **kein `data/`**. Trigger =
  `data/`-Manifestation (ESA-seitig, extern). (Schritt: bei Manifestation
  PDS4-TNF/ODF-Parser nach dem tatsächlichen Datentyp.)

## NSE/Haug — Rohdaten zugesagt (`wartend`)

- Keller-Antwort (17.09.), sendet „in einigen Tagen". Trigger = Dateieingang.
  (Schritt: bei Eingang `nse_haug_trisp`-Quelle + Compiler + `sources.φ`;
  0-Kanon: kein Asset ohne Datei.)

## Legacy-Konzepte (`operator-gebunden`, hintenangestellt)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als 4.,
  Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt: bei
  Wiederaufnahme den Rat-Erster-Atom bauen —
  `tools/measure/src/bin/silence_map_probe.rs`.)

## Paper / Präregistrierung (`termin`)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- Rat (`council`, pro/max) für die offene Design-Frage des Shard-Checks:
  Verdikt **Design A verfeinert** (Register↔Release-Mengenvergleich, workflow-only;
  B/C verworfen — der Shard-Satz ist erst nach der Ernte berechenbar). Architektur-
  Konsultation, keine Benchmark-Klasse; kein flash/pro-Doppellauf in diesem Atom
  (Routine-Recherche-Klasse geschlossen, flash-Sieger 2026-09-16).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `.github/workflows/planetary-odf-cdn.yml`,
  `docs/handover/handover-2026-09-18-forschung-folge75.md` (+ archiviertes
  `handover-2026-09-18-forschung-folge74.md`).
- **Fremd (nicht anfassen):** `opencode.json`, `src/archivar/*.rs`,
  `src/mathematikerin/te.rs`, `docs/handover/post.md`,
  `docs/zustand/external-state.md`, `.github/workflows/harvest.yml`,
  `phi/supermag_stations.φ`, die drei gestagten `handover-2026-09-16-*`-Renames,
  `docs/handover/handover-2026-09-18-bau-folge75.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
