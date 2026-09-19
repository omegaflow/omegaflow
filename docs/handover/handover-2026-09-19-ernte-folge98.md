<!--
  title: Handover — Ernte-Folge 98 (Stand 2026-09-19)
  session: Ernte-Folge 98
  class: handover
  date: 2026-09-19
  sha256: 40ea85387bf2dcffa19c576ed7a8d75b099c4c667bb0421230df6ad07025ab2b
  status: live
-->
# Handover — Ernte-Folge 98 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Folge 98)

- **HEAD** `60d2c2fb` (Session-Start, „ernte folge97: cassini compilers crawl
  parallel+pruned") → `d1750fe0` (== origin/main beim Abschluss; eine fremde Linie
  committete+pushte während der Session). Safety-Net `refs/safety/1789849281` (Start).
- **Postfach** — `post.md` leer (keine ernte-Zeile); `external-state.md:20`: jüngster
  Ledger-Eingang `1789795811` (Rubin-Forum, informativ); keine Nachricht an ernte.
- **CI** — Watchdog-Snapshot 21:25Z + `ci_manage list`; der `external-state.md:22`-Eintrag
  (Bau-Folge 93, HEAD `60d2c2fb`) trägt den Cassini-success bereits — zitiert, nicht
  kopiert: **success** `cassini-odf-cdn` `35466653128`/`35466677690` (20:12→20:20Z),
  `cassini-rsr-cdn` `35466678889`/`35466680133`, `harvest-dispatch` `35466644143`,
  `auto-dispatch` `35466644173`, `fmt-apply` `35464780631`; **pending** `ci-check`
  `35466644178`, `hyperscanning-te` `35466436802`, `te-gate` `35462518676`,
  `health-check` `35466025266`; **in_progress** `hyperscanning-te` `35465589119`,
  `ci-check` `35465331299`, `allwise-cdn` `35464786486`.

## Source-Port — offene Arme

- **OpenNeuro ds007822** `phi/pipeline/ledger.φ:122-124` — `--dataset`-Argument im
  Compiler gebaut (`openneuro_compiler.rs`, Default `ds005034`), `openneuro-cdn.yml`
  mit Workflow-Input `dataset` + Filter-Trennung (ds005034: `--subject sub-02 --task
  rest`; Probe: keine Filter); `cargo check -p omegaflow-harvest --all-targets` 0/0.
  Probe **dispatcht** `35468307481` @`cbd8573a` (in_progress, `gh workflow run
  openneuro-cdn.yml -f dataset=ds007822`), `wartend` (Trigger: Run-Abschluss);
  Ergebnis aus Watchdog / `ci_manage view 35468307481`, grün → `sources.φ`-Registrierung.
  ds004103 (BOLD-fMRI) per Messung `declined` (`declined_sources.φ`).
  ds007471 (BrainVision-`.vhdr/.eeg/.vmrk`-Arm) + ds008192 (SNIRF/HDF5-Arm) offen →
  bau. `wartend`.
- **archive_search `--sniff` Riss** — `--sniff` meldet einen sha256 über einen
  **Teil-Download**, nicht über die volle Datei (gemessen 2026-09-19: `cassini_odf.bin`
  1 496 960 792 B → sniff 45 494 455 B/`fc48b662…`, dann 54 007 121 B/`f97f217d…`;
  `cassini_rsr.bin` 1 308 000 008 B → sniff 62 661 056 B/`578e2109…`). Wahrheit:
  GitHub-Release-API `digest` + `content-length`. **Schritt:** den sniff-Fetchpfad
  (`tools/utils/src/bin/archive_search.rs` `Mode::Net("sniff")`) auf ganze Datei
  umstellen oder den Hash als partiell markieren; danach `--sniff` für Voll-Assets
  unbenutzbar als sha256-Zeuge. `bau`.
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20` —
  `blockiert` (extern). Re-Messung 2026-09-19: Root je 200, sync-QUERY beider
  VOTable-`ERROR` `connection to server at "localhost" port 5432 failed: Connection
  refused`. **Schritt:** Re-Check (`archive_search --verdict` + sync-QUERY), Trigger
  Backend-Erholung. `blockiert`.
- **gedi_l2a** `phi/harvest.φ:57-65` — `blockiert` (Code, hdf5.rs fremd) → an bau.
- **icesat2_atl03** `phi/harvest.φ:75-83` — `blockiert` (Budget); ein fremder Arm
  (`tools/harvest/src/bin/icesat2_atl03_compiler.rs`) ist im Baum in Arbeit → an bau.

## Wartend (kein Auswahlpunkt)

- **allwise-cdn** `35464786486` (scheduled, stündlich) — `allwise_coverage.fp01`
  noch 404 (`archive_search --sniff`); bei Abschluss CDN-Manifestations-Pflicht
  (`sources.φ`-Registrierung fehlt). **Schritt:** Watchdog-Snapshot / `ci_manage view
  35464786486`. `wartend`.
- **Lasair-LSST** `external-state.md:23` — direct+Proton 502, Wayback 200 ohne Snapshot.
- **Voyager/Mariner/Viking/Juno/Cassini-Antworten** `blocked_sources.φ` — Anfragen offen.
- **BepiColombo bc_mpo_more** `blocked_sources.φ:30-33` — Freigabe ~April (PSA/Iess).
- **Limadou PI-Freigabe** `ledger.φ:26-28` — per-act consent. `operator-gebunden`.
- **Queue-Korpora** `ledger.φ:82-120` — `--port` braucht das Operator-Wort.
  `operator-gebunden`.

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — maxTime unverändert
  `2026-07-30T23:30:00Z`. Nächste Re-Messung **2026-10-19**. `termin`.

## Benchmark

- **Cassini-Registrierung** (grind-flash): mechanisch; der Agent fand den Riss
  selbst (die Vorgabe-sha256 waren Teil-Downloads des `--sniff`) und registrierte
  die Release-API-Wahrheit — kein pro/max nötig.
- **OpenNeuro-Parametrisierung** (grind-pro): Compiler `--dataset` + Workflow-Input
  + Filter-Trennung; Dispatch HTTP 422 (remote ohne Input) korrekt als Messung
  gemeldet, keine erfundene Run-ID. Burn: s. `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/harvest.φ`, `phi/sources.φ`, `phi/blocked_sources.φ`,
  `phi/declined_sources.φ`, `phi/pipeline/ledger.φ`,
  `tools/harvest/src/bin/openneuro_compiler.rs`,
  `.github/workflows/openneuro-cdn.yml`, `docs/handover/post.md` (eine bau-Zeile +
  Header-sha), die neue `handover-2026-09-19-ernte-folge98.md`, der Move
  `docs/handover/archiv/handover-2026-09-19-ernte-folge97.md`.
- **Fremd (nicht anfassen):** `.github/workflows/hyperscanning-te.yml`,
  `src/archivar/hdf5.rs`, `src/mathematikerin/te.rs`,
  `tools/measure/src/bin/hyperscanning_group_te.rs`, die drei
  `handover-2026-09-16-*`-Moves, `docs/handover/handover-2026-09-19-forschung-folge97.md`.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
