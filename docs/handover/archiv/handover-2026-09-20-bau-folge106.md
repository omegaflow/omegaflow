<!--
  title: Handover — Bau-Folge 106 (Stand 2026-09-20)
  session: Bau-Folge 106
  class: handover
  date: 2026-09-20
  sha256: 564186667088e30bb5b16b457ee687515a4954989c0683a08246c9d35b96d6fa
  status: live
-->
# Handover — Bau-Folge 106 (2026-09-20)

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
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-20, Session-Beginn)

- **HEAD** `e52b59d0` (== `origin/main`, „port: migrate dist_key/pmra_key/
  pmdec_key/radvel_key in the --port converter"). Arbeitsbaum: fremd-`M`
  `docs/handover/post.md`, `docs/zustand/external-state.md`; fremd-`R`
  entscheid-folge62-Archiv-Move; untracked `handover-2026-09-20-entscheid-folge63.md`
  — nicht angefasst. Snapshot `refs/safety/1789921637`.
- **Postfach** — `post.md` leer (18 Zeilen), keine bau-Zeile; kein neuer Eingang.
  Zustand-Eintrag `docs/zustand/external-state.md:20` (fremd-`M`) zitiert, nicht kopiert.
- **CI** — Watchdog-Snapshot `2026-09-20T18:10:36+02:00`: `ci-check`
  `35520538766` in_progress; die 3 `failed`
  (`35517957987`/`35517136467`/`35516232478`) liegen am Branch `tools-latest`,
  nicht am Bau-HEAD. Kein grüner `ci-check` am HEAD.

## Offen

- **`rv_scale` für die 17 `radvel`-Quellen messen — `pending`** — seit diesem Atom
  ist ein `radvel` ohne gemessene Skala **absent** (vorher Faktor 1.0: km/s wurde
  als m/s gelesen, 1000× zu klein). `phi/sources.φ` trägt 17 `radvel`-Zeilen
  (`:7720, 8736, 8778, 8830, 8929, 8951, 8979, 9009, 9026, 9076, 9094, 9111,
  9130, 9147, 9207, 9238, 9497`) und **0** `rv_scale` — die Radialgeschwindigkeit
  dieser Quellen fällt bis zur Messung auf absent. (Schritt: je Quelle die
  radvel-Einheit aus der Katalog-Doku messen — RAVE `rv`/`HRV` = km/s → `1000.0`
  — und `rv_scale <faktor>` in den Quellenblock setzen; Werkzeug
  `archive_search --verdict <url>`/`sfetch`.) · `pending`
- **`dist_key` ohne `dist_scale`** — 61 Treffer in 4 (gitignored) Korpora; seit
  diesem Atom ist ein `dist` ohne Skala absent (0 honored) statt Faktor 1.0. Die
  registrierten `sources.φ` tragen 26 `dist` / 26 `dist_scale` (kein Verlust).
  Offen bleibt die Einheiten-Messung je Katalog (`sources.φ:6006` `Dist`=kpc vs.
  `:6055` `Dist`=pc). (Schritt: je Quelle `dist_scale` in Meter/Einheit messen
  und setzen, dann Konverter-Re-Lauf.) · `pending`
- **`pos` ohne `body`-Direktiv** — gemessen 3 Blöcke:
  `sources_new_untested_2k.φ:1806`, `sources_potential_pre-cdn_9k_richest.φ:199`,
  `:2344`; kein kanonisches `pos` (`docs/SOURCE_PORT.md:275`). Die Dateien liegen
  nicht im Baum (lokal/ignoriert). (Schritt: den 3 Blöcken ein `body <body>`-Direktiv
  geben bzw. `lat`/`lon` setzen, dann Konverter-Re-Lauf.) · `blockiert`
  (Dateien fehlen lokal)
- **Queue-Korpora Re-Lauf nach Konverter-Fix — `operator-gebunden`** — der
  `--port`-Konverter trägt jetzt ra/dec/plx/z, dist/pmra/pmdec/radvel und rv_scale;
  die 7 `parser-gap`-Korpora (`phi/pipeline/ledger.φ:94-120`) brauchen den
  `--port`-Re-Lauf. Lokaler Funktionslauf ist der Session verweigert, kein
  CI-`--port`-Workflow (Korpora gitignored). (Schritt: Operator-Wort für den
  lokalen Release-Binär-Lauf auf den 7 Korpora, dann `--port` + `--probe`, dann die
  7 Ledger-Notes fortschreiben.) · `operator-gebunden`
- **`ci-check`-Verdikt am Fix-HEAD — `wartend`** — der Push dieses Atoms triggert
  `ci-check` neu (`paths: src/**`) und führt die neuen Tests
  `test_parse_sources_dist_without_scale_is_absent`,
  `test_parse_sources_rv_scale_directive`,
  `test_extract_cmap_dist_without_scale_is_absent`,
  `test_extract_cmap_rv_without_scale_is_absent` und die erweiterte
  `test_port_convert_dist_pm_rv_keys` mit; `cargo check --all-targets` ist 0/0.
  (Schritt: den neuen Run einmalig aus `/tmp/opencode/ci_status.md` lesen; bei Rot
  `ci_manage log <id>`.) · `wartend`

## Benchmark

- **Bau-Folge 106**: Atom „`dist_scale`/`rv_scale` als `Option` — Einheiten-
  Fabrikation entfernt (0 honored)". Urteilsatom, direkt in der Haupt-Session
  (`build`) — kein Doppellauf; die Klasse „Urteil UND Schreiben in einem Kontext"
  ist die des `build`. `cargo check --all-targets` 0/0.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/archivar/types.rs`, `src/archivar/parse.rs`,
  `src/archivar/extract.rs`, `src/archivar/port.rs`, `src/archivar/tests.rs`,
  `src/gate/commit_gate_vocab.json`; neues
  `docs/handover/handover-2026-09-20-bau-folge106.md`; Move
  `handover-2026-09-20-bau-folge105.md` → `archiv/`.
- **Fremd (nicht anfassen):** `docs/handover/post.md` (`M`),
  `docs/zustand/external-state.md` (`M`),
  `docs/handover/handover-2026-09-20-entscheid-folge62.md` → `archiv/` (`R`),
  `docs/handover/handover-2026-09-20-entscheid-folge63.md` (`??`). Nie ein
  nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Push triggert
`ci-check` selbst; die Session pollt nicht. `/consent` ist der session-weite
Consent, nie das Commit-Wort.
