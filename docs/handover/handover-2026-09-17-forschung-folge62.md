<!--
  title: Handover — Forschung-Folge 62 (Stand 2026-09-17)
  session: Forschung-Folge 62
  class: handover
  date: 2026-09-17
  sha256: 5408d6f29ace80bc1bfa2cef93995c8d75d5b98ddc338f9c97d03377c00a70ba
  status: live
-->
# Handover — Forschung-Folge 62 (2026-09-17)

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

## Stehender Pass (automatisch, keine Auswahl)

- **Postfach** — `state/mail/mail_ledger.φ`: kein neuer externer Eingang seit
  Folge61 (letzte: Rubin-Forum-Summary ohne Antwort; NSE/Haug „a few days";
  CSES/Sotgiu „wait a few weeks"). `docs/zustand/external-state.md` trägt fremde
  uncommittete Änderungen → zitiert, nicht überschrieben.
- **CI-Status am HEAD `a130d88d`** — `ci_manage view`: **`paper-check`
  `35243312945` @`a130d88d` = success** (15:56:50Z) — das Reference-Gate am
  gehärteten Extraktor ist grün. `ci-check` `35243313008` pending; die CDN-Welle
  (quake-feeds/swpc-mirror/vires-hapi/allwise) aktiv. Der Watchdog-Snapshot
  (17:05:47Z) liegt vor dem Push.

## §4 — fsky-Census (härtester undatiert, forschung-eigen)

- Offen: nach dem Push `gh workflow run pioneer-cell-census.yml` dispatchen
  (die Datei muss auf `main` liegen; der Dispatch scheiterte am untracked
  Workflow mit HTTP 404). Der Workflow baut
  `tools/measure/src/bin/pioneer10_cell_census_probe.rs` mit **beiden** Feldern
  (`--field resid` = `r[8]`, `--field fsky` = `r[1]` mit `r[1].is_finite() &&
  r[1] > 0.0`) und lädt `pioneer-cell-census.txt` hoch. Den Lauf **einmal** lesen
  (`ci_manage list` → `ci_manage view <id>` bzw. `gh run view <id> --log`) und das
  Ergebnis in `docs/paper/twenty-second-band-ground-chain.md:188–190` eintragen:
  trägt fsky den 44–58-mHz-Komplex → upstream der ODP-Kette; trägt er ihn nicht →
  **§8 light-time** ist der erste benannte Kandidat. (Schritt: nach Push
  dispatchen, Lauf lesen.)

## Docs-Pendings — Sichtung 2026-09-17

- Machbare Einzelne (je Survey-Zeile belegt): Mariner `PSPA-00316` CDN-Dispatch
  (`survey-2026-09-17-sonden-request-only.md:66–67`); Juno post-EFB OCRU ↔
  `juno_odf.bin` (`:38`); Pioneer ATDF dtype-12/13 (`src/archivar/atdf.rs:508`
  gated nur 1|2); MAVEN-TNF-Shards; GOES-16 ABI `sources.φ`+Workflow
  (`survey-2026-09-14-kapitulationen-pendings-inventur.md:51`); WWLLN
  (`survey-2026-09-13-weberin-quellen.md:188`); Ulysses/BepiColombo/LRO
  `sources.φ` (`survey-2026-09-16-sonden-flotte.md:57–59`). (Schritt: je Eintrag
  der genannten Survey-Zeile.)

## CDN-Concurrency Follow-up

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht
  (`grep -q "^mro_odf_"` übersieht ein halbes Set); `cancel-in-progress: false`
  bleibt bis dahin. (Schritt: auf die erwarteten Shard-Namen härten, messbar erst
  nach einem erfolgreichen `mro_odf`-Lauf.)

## NSE/Haug — Route offen, Rohdaten zugesagt

- Thomas Keller (TRISP) sendet die NSE-I(q,t)-Rohdaten „in a few days". (Schritt:
  bei Mail-Eingang `nse_haug_trisp`-Quelle + Compiler + `sources.φ`; 0-Kanon:
  kein Asset ohne Datei.)

## Legacy-Konzepte (hintenangestellt — Operator-Wort)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als 4.;
  Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt: bei
  Wiederaufnahme den Rat-Erster-Atom bauen — `tools/measure/src/bin/
  silence_map_probe.rs`, Null-Kalibrierung als Spiegel der FP/FN/Symmetrie-Gates
  von `te.rs`.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- Der fsky-Zweig-Bau als `grind-flash`-Dispatch (Routine: mechanische
  Probe-Erweiterung + Workflow) — flash traf die Spec, `cargo check` grün, keine
  Eskalation; die Routine-Klasse ist geschlossen (flash-first). §4-Diagnose bleibt
  `research-max` (hartes Atom).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `tools/measure/src/bin/pioneer10_cell_census_probe.rs`,
  `.github/workflows/pioneer-cell-census.yml`,
  `docs/handover/handover-2026-09-17-forschung-folge62.md` (+ archiviertes
  `handover-2026-09-17-forschung-folge61.md`). Fremde uncommittete Arbeit (nicht
  anfassen): die ~70 `M .github/workflows/*-cdn.yml`, `docs/concepts/tools-map.md`,
  `docs/zustand/external-state.md`, `phi/sources.φ`,
  `tools/utils/src/bin/archive_search*.rs`, die gestagten Renames
  `handover-2026-09-16-*` → `archiv/`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
