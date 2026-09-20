<!--
  title: Handover — Forschung-Folge 108 (Stand 2026-09-20)
  session: Forschung-Folge 108
  class: handover
  date: 2026-09-20
  sha256: 995a1468692b64d172b89a809307c062208d9f06405f3d9c6b5a77160b29ac09
  status: live
-->
# Handover — Forschung-Folge 108 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 108)

- **HEAD** `0e037e1e` (== `origin/main`); `git_safety` Snapshot
  `refs/safety/1789915190`. Der Baum trug bei Beginn fremde uncommittete Arbeit
  (ernte: `handover-…-ernte-folge107.md` → `archiv/` + neue folge108,
  `docs/zustand/external-state.md` staged M, `phi/pipeline/ledger.φ` M,
  `src/archivar/bsp_reader/daf.rs` worktree M, `brainvision.rs`) — nicht angefasst.
- **Postfach** — `post.md` leer; letzter Ledger-Eingang `1789906306`; keine neue
  forschung-eigene Post. Zeile steht in `docs/zustand/external-state.md`.
- **CI** (Watchdog-Snapshot 2026-09-20T16:03+02:00): in_progress `harvest`
  `35515194248`, `te-gate` `35513982359`, `ci-check` `35513190719`, `health-check`
  `35505471538`; failure `release-build` `35513611936` @`7d0a1272` + die
  ci-check-Kette. Die **CI-Zeile in `external-state.md` ist fällig** (HEAD
  `ae0fce25` ≠ `0e037e1e`), aber die Datei ist von der ernte-Linie gehalten
  (staged, fremde Arbeit) — nicht angefasst.

## Kernel-Force-Review — Warteschlange (härtester undatierter Punkt)

Rat-Verdikt (2026-09-20): die „Kernel-Vorgabe pro Force" ist ein **Konverter-
Fallback für Zeilen OHNE Kernel-Token** (`sources-v2-spec.md:70-71`), nicht
normativ über deklarierten Zeilen; der Parser ehrt den Token zuerst
(`parse.rs:759/1167`), der Konverter materialisiert den Default nur im Synthese-/
Entwurfspfad (`port.rs:18/1214/1237`). Kein Register↔Code-Riss, kein Glätten,
keine Massen-Korrektur. Der Kontrakt ist geschärft (`SOURCE_PORT.md:233-238`),
die veralteten Referenz-Pfade korrigiert (`SOURCE_PORT.md:195-210`,
`docs/concepts/` → `docs/specs/`). Offen bleibt die per-Klasse-Physikbewertung
der deklarierten Abweichungen — Warteschlange, gemessene Matrix in folge107:

- `inverse-square acoustic` 371 (Default gaussian-inverse-square)
- `inverse-square advective` 336 (Default patch-levy)
- `inverse-square thermal` 220 (Default exponential-decay)
- `gaussian-inverse-square em` 60, `gaussian-inverse-square advective` 41,
  `erfc diffusion` 27, `gaussian-inverse-square thermal` 23, `erfc thermal` 17,
  `gaussian-inverse-square gravity` 9, `erfc em` 4 u. a.
- (Schritt: je Klasse die Physik bewerten, ob Punkt-Sensor inverse-square trägt
  oder die Ausdehnungsform gilt — `grind-pro`; nie per-Zeile-Glättung.)

## Datenbank-Ausbau — offene Kandidaten

`--entrez` in diesem Atom gebaut (NCBI E-utilities esearch, keyless): Modul
`tools/utils/src/bin/archive_search/entrez.rs`, Verdrahtung in
`archive_search.rs`/`net.rs`/`web.rs`/`server.rs`, `docs/concepts/tools-map.md`;
`cargo check -p omegaflow-utils --all-targets` 0/0. JSON-Form live gemessen:
Top-Level `header` + `esearchresult` mit `count` (String), `idlist` (Strings),
`querytranslation`. Bewusst nicht in `--all`/`QUERY_MODES` (key=value-Modus).
Noch offen: `--ena`, `--biomodels`, `--nist`, `--cod` (alle HTTP 200 direct außer
KEGG/Lizenz). (Schritt: nächsten Kandidaten nach `docs/SOURCE_PORT.md` portieren —
`grind-flash`.)

## TE-Gate-Verdikt — `wartend`

`te-gate` `35513982359` (in_progress); der n=1000-FPR-Boden ist ungemessen.
(Schritt: `ci_manage view 35513982359`; bei rot `ci_manage log`.)

## Flyby Path 2 — `termin`

Das versiegelte Blatt (`docs/paper/flyby-path-2-preregistration.md`, Seal
2026-08-22; `-falsification-metric-addendum.md`, Seal 2026-08-28) ist kein
Auftrag. Die präregistrierte Kette (plasma-pressure gradient, IMF-Bz, Kp, Swarm)
muss transit-time-korrigiert am Perigäum-Tubus gefüllt und als Auftrag
registriert werden — nach dem Ereignis (28./29.09.) ist die Vorhersage post-hoc.
Kanäle leben: `phi/sources.φ` RTSW mag/wind (`:158,164`), Kp (`:179`), Swarm
(`:6062-6078`). (Schritt: Auftrag in `docs/auftrag/`, dann die Kette füllen —
`research-max`.)

## API-Keys ablegen — `operator-gebunden`

`--core` (keyless nutzbar), `--semanticscholar`, `--materialsproject` sind gebaut;
ohne Key melden sie `pending` bzw. den 429. (Schritt: `CORE_API_KEY` und
`S2_API_KEY` aus `state/mail/mail_ledger.φ`, `MP_API_KEY` aus
`next-gen.materialsproject.org/api` in `.secrets.local`; dann messen.) Gehört zur
`entscheid`-Linie.

## ernte / termin

- **MAG-Asset** — `blockiert` (ernte): Compiler nach `voyager_odr_compiler.rs`,
  dann `sources.φ` + CI-Manifestation.
- **NSE/Haug** — `wartend`: Trigger Dateieingang.
- **BepiColombo MORE** — `termin` (~April 2027). **Flyby-Path-2-Abruf** —
  `termin` (28.09., s. o.).

## Benchmark (dieses Atom)

- Kein Doppellauf: das Atom war ein Architektur-Urteil (Rat zum Kernel-Riss) +
  ein Routine-Port (`--entrez`, Klasse „Routine-Port" geschlossen 2026-09-16,
  Sieger `grind-flash`) + ein Kontrakt-Edit. Rat (pro/max) für den Kernel-Riss,
  `grind-flash` für den Port — keine offene Benchmark-Klasse.

## Geteilter Baum — eigener Pfad-Satz

- `docs/SOURCE_PORT.md`,
  `tools/utils/src/bin/archive_search/entrez.rs` (neu),
  `tools/utils/src/bin/archive_search.rs`,
  `tools/utils/src/bin/archive_search/net.rs`,
  `tools/utils/src/bin/archive_search/server.rs`,
  `tools/utils/src/bin/archive_search/web.rs`,
  `docs/concepts/tools-map.md`,
  `docs/handover/handover-2026-09-20-forschung-folge107.md` (Korrektur + Move),
  `docs/handover/handover-2026-09-20-forschung-folge108.md` (neu).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
