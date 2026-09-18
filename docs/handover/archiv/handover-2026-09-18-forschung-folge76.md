<!--
  title: Handover — Forschung-Folge 76 (Stand 2026-09-18)
  session: Forschung-Folge 76
  class: handover
  date: 2026-09-18
  sha256: 46280426b08dc00e9430ea59cf3f7719872ddb7320bc6d229463b1c8b119db48
  status: live
-->
# Handover — Forschung-Folge 76 (2026-09-18)

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

- **HEAD** `45190022` == `origin/main`. Fremd uncommittet (nicht angefasst):
  `opencode.json`, `src/mathematikerin/te.rs`, `docs/handover/post.md`,
  `docs/zustand/external-state.md`, `.github/workflows/harvest.yml`,
  `te-gate.yml`, `phi/supermag_stations.φ`,
  `tools/harvest/src/bin/gedi_l2a_compiler.rs`, die gestagten
  `handover-2026-09-16-*`-Renames, `handover-2026-09-18-entscheid-folge45/46.md`,
  `handover-2026-09-18-bau-folge76.md`. Safety-Net `refs/safety/1789712066`.
- **CI** (`ci_manage list`/`view`): `pioneer-cell-census` `35312007651`
  **success** @`8d1264bb` (Trigger §4 fsky-Census gefeuert, abgearbeitet);
  `planetary-odf-cdn` `35313968728` @`45190022` in_progress (fremd);
  `harvest` `35312009465` (format `lro_trk`) @`8d1264bb` **in_progress**;
  `harvest` `35312992622` @`627793c4` in_progress; `ci-check` `35313990442`
  @`45190022` pending; `te-gate` `35313041295` @`b8bb7b01` in_progress (fremd);
  sonst fremde Linien. Zustand-Eintrag: `docs/zustand/external-state.md:22`
  (fremd-dirty, zitiert, nicht kopiert).
- **Postfach** — letzter Ledger-Eingang `1789689115` (Rubin-Forum/LSST-Thread,
  kein Agenten-Eingang); `post.md` trägt zwei `An entscheid:`-Zeilen (Lasair-Exit,
  force-Gate `--port`), operator-gebunden.

## Kein abarbeitbarer undatierter Punkt (`—`)

- Alle verbleibenden Punkte sind Wartestellungen, operator-gebunden oder datiert
  (s. u.). Es gibt in diesem Handover keinen undatierten Punkt, den diese Linie
  selbst abarbeiten kann — der Planungs-Pass sagt das und erfindet keine Arbeit
  aus einem Wait.

## §4 fsky-Census — erledigt (2026-09-18)

- Der Trigger ist gefeuert (`35312007651` success). Die Cross-Rank- und
  r[2]-Reference-LS-Sektionen des Census (Probe-Erweiterung `98fbaeb1`) sind in
  `docs/paper/twenty-second-band-ground-chain.md` v12 gefaltet: rx14/rx63
  re-ranked, rx43 unter dem Floor (erschaffen im NOCC-Glied), Referenzpfad
  gemessen nicht getragen (FAP 1,00). Rat-Verdikt + Confounds im Paper-Atom.
  Kein offener Rest.

## LRO utF (`wartend`)

- Trigger = Abschluss des `harvest`-Laufs. **Läuft:** `35312009465` (format
  `lro_trk`) in_progress @`8d1264bb`. (Schritt: nach Abschluss
  `archive_search --sniff` auf `lro_trk.bin`, sha256/Größe, `phi/harvest.φ`
  (`format lro_trk`) + `phi/sources.φ`-Block auf present.)

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

- Der fsky-Census-Atom lief flash-first (zwei `grind-flash`-Taucher parallel:
  odyssey-Quellenmessung + Census-Artefakt); kein flash/pro-Doppellauf, da die
  Routine-Recherche-Klasse geschlossen ist (flash-Sieger 2026-09-16). Rat
  (`council`, pro/max) für das Paper-Closing (Cross-Rank-Deutung) — Architektur-/
  Abschluss-Konsultation, keine Benchmark-Klasse.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/paper/twenty-second-band-ground-chain.md`,
  `phi/harvest.φ`, `docs/handover/handover-2026-09-18-forschung-folge76.md`
  (+ archiviertes `handover-2026-09-18-forschung-folge75.md`).
- **Fremd (nicht anfassen):** `opencode.json`, `src/mathematikerin/te.rs`,
  `docs/handover/post.md`, `docs/zustand/external-state.md`,
  `.github/workflows/harvest.yml`, `te-gate.yml`, `phi/supermag_stations.φ`,
  `tools/harvest/src/bin/gedi_l2a_compiler.rs`, die gestagten
  `handover-2026-09-16-*`-Renames, `handover-2026-09-18-entscheid-folge45/46.md`,
  `handover-2026-09-18-bau-folge76.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
