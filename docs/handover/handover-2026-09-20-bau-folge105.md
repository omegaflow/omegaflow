<!--
  title: Handover — Bau-Folge 105 (Stand 2026-09-20)
  session: Bau-Folge 105
  class: handover
  date: 2026-09-20
  sha256: 7d2405717f465652d772586eaf28b2d6453a13178db8dd60d1265a6606c9770c
  status: live
-->
# Handover — Bau-Folge 105 (2026-09-20)

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

- **HEAD** `1963be30` (== `origin/main`, „forschung folge114: resolve --nist …").
  Arbeitsbaum: fremde Linien-Arbeit — forschung-Archiv-Move `R`, `M` an
  `docs/concepts/tools-map.md` + `docs/zustand/external-state.md`, untracked
  `docs/auftrag/auftrag-flyby2-kette.md` + `handover-2026-09-20-forschung-folge115.md`
  — nicht angefasst. Snapshot `refs/safety/1789920905`.
- **Postfach** — `state/mail/mail_ledger.φ` letzter Eingang `1789918147`
  (GitHub-OAuth-App „ISH Chat", als `An entscheid:` in `post.md`); kein neuer
  Eingang. Keine bau-Zeile im Postfach. Zustand-Eintrag
  `docs/zustand/external-state.md:20` (fremd-`M`) zitiert, nicht kopiert.
- **CI** — Watchdog-Snapshot `2026-09-20T18:10:36+02:00`: `ci-check`
  `35520538766` in_progress, `35522085406` pending; `failed`
  `35517957987`/`35517136467`/`35516232478` liegen am Branch `tools-latest`,
  nicht am Bau-HEAD. Kein grüner `ci-check` am Bau-HEAD
  (`docs/zustand/external-state.md:22`).

## Offen

- **Queue-Korpora Re-Lauf nach Konverter-Fix — `operator-gebunden`** — der
  `--port`-Konverter trägt seit Folge 104 ra/dec/plx/z und seit Folge 105
  dist/pmra/pmdec/radvel; die 7 `parser-gap`-Korpora
  (`phi/pipeline/ledger.φ:94-120`) brauchen den `--port`-Re-Lauf, um den
  Survivor-Stand zu messen. Lokaler Funktionslauf ist der Session verweigert,
  kein CI-`--port`-Workflow (Korpora gitignored). Post-Zeile steht
  (`post.md:20`, Ernte-Folge 115). (Schritt: Operator-Wort für den lokalen
  Release-Binär-Lauf auf den 7 Korpora, dann `--port` + `--probe`, dann die
  7 Ledger-Notes auf das Ergebnis fortschreiben.) · `operator-gebunden`
- **`ci-check`-Verdikt am Fix-HEAD — `wartend`** — der Push dieses Atoms
  triggert `ci-check` neu (`paths: src/**`) und führt den neuen Test
  `test_port_convert_dist_pm_rv_keys` (`src/archivar/tests.rs`) mit;
  `cargo check --all-targets` ist 0/0. (Schritt: den neuen Run einmalig aus dem Watchdog-Snapshot
  `/tmp/opencode/ci_status.md` lesen; bei Rot `ci_manage log <id>`.) · `wartend`

## Benannter Rest (gemessen, Quellen-Kuration)

- **`dist_key` ohne `dist_scale`** — 61 Treffer in 4 Korpora (Werte: `Dist` 26,
  `dist` 9, `sy_dist` 7, `DIST` 5, `1000/Plx` 3, `DIST_BPG` 3, `d` 2, `Rsun` 2,
  `sy_distan` 2, `D` 1, `Delta` 1). Der Konverter trägt `dist <key>` nur bei
  vorhandener `dist_scale`; die Korpora tragen keine (0 Treffer), und derselbe
  Key hat je Katalog eine andere Einheit (`sources.φ:6006` `Dist`=kpc vs.
  `:6055` `Dist`=pc) — daher keine mechanische Migration, Distanz bleibt absent
  (0 honored). (Schritt: je Quelle die Einheit aus der Katalog-Doku messen und
  `dist_scale` in Meter/Einheit setzen, dann Konverter-Re-Lauf.) · `pending`
- **`pos` ohne `body`-Direktiv** — gemessen 3 Blöcke (nicht 5):
  `sources_new_untested_2k.φ:1806`, `sources_potential_pre-cdn_9k_richest.φ:199`,
  `:2344`; kein kanonisches `pos` (`docs/SOURCE_PORT.md:275`). (Schritt: den 3
  Blöcken ein `body <body>`-Direktiv geben bzw. `lat`/`lon` setzen, dann
  Konverter-Re-Lauf.) · `pending`
- **`dist_scale`-Default `1.0`** — `Extract::CelestialMap` (`src/archivar/extract.rs:1095`)
  und `parse.rs` setzen `dist_scale` auf `1.0`; ein `dist` ohne `dist_scale`
  liest den Rohwert damit als Meter (Faktor 1) — eine latente Einheiten-
  Fabrikation. Kein registrierter Quell nutzt den Default (`sources.φ`:
  26 `dist ` / 26 `dist_scale`). (Schritt: `dist_scale` als `Option` führen
  bzw. `dist` ohne Skala als absent behandeln, mit Test.) · `pending`

## Benchmark

- **Bau-Folge 105**: Atom „`--port`-Konverter dist/pmra/pmdec/radvel +
  `1000/<k>`→`plx`". Delegation `grind-flash` (mechanische Direktiv-Inventur,
  read-only), `cargo check --all-targets` 0/0. Kein Doppellauf — die Klasse
  „routine mechanical port" hat den gemessenen Sieger `grind-flash`
  (2026-09-16, 8 Profile identisch).

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/archivar/port.rs`; `src/archivar/tests.rs`;
  `phi/pipeline/ledger.φ` (7 `parser-gap`-Notes); neues
  `docs/handover/handover-2026-09-20-bau-folge105.md`; Move
  `handover-2026-09-20-bau-folge104.md` → `archiv/`.
- **Fremd (nicht anfassen):** `docs/concepts/tools-map.md` (`M`),
  `docs/zustand/external-state.md` (`M`), `docs/handover/handover-2026-09-20-forschung-folge114.md`
  → `archiv/` (`R`), `docs/auftrag/auftrag-flyby2-kette.md` + 
  `docs/handover/handover-2026-09-20-forschung-folge115.md` (`??`). Nie ein
  nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Push triggert
`ci-check` selbst; die Session pollt nicht. `/consent` ist der session-weite
Consent, nie das Commit-Wort.
