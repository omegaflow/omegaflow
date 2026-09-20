<!--
  title: Handover — Forschung-Folge 118 (Stand 2026-09-20)
  session: Forschung-Folge 118
  class: handover
  date: 2026-09-20
  sha256: 4cd59695c705a2a77b2b0ec1c5d428feb05189fa06694b62b26cc517376190d1
  status: live
-->
# Handover — Forschung-Folge 118 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 118)

- **HEAD** `b9112e5d` (== `origin/main`, gemessen bei Session-Beginn). Fremde
  uncommittete Arbeit im Baum: `docs/handover/post.md` (M),
  `docs/zustand/external-state.md` (M, entscheid-folge64-Hunks),
  `docs/handover/handover-2026-09-20-entscheid-folge64.md` (untracked) und der
  gestagte Move `handover-2026-09-20-entscheid-folge63.md → archiv/` — **nicht
  angefasst**.
- **Postfach** — kein neuer Ledger-Eingang seit `1789922257` (`sales@pine64.org`:
  Verweis auf `info@pine64.org`); davor `1789918147` (GitHub-OAuth „ISH Chat",
  Sicherheitsereignis, als `An entscheid:` geführt). `smail_recv` läuft (bind
  127.0.0.1:1619 belegt) — der Empfänger schreibt in `state/mail/mail_ledger.φ`.
  `post.md` leer.
- **CI** — `ci_manage list` 2026-09-20 ~19:5xZ: **in_progress** `ci-check`
  `35526010713`, `tools-build` `35525924525`; **failure** `ci-check` `35523145456`
  @`5c61e16e` (s. härtester Punkt); `te-gate` `35513982359` @`7d0a1272` weiter
  **in_progress** (kein Update seit Start 13:36Z). Alle neueren `ci-check`-Läufe
  `cancelled` (Concurrency-Gruppe) — kein sauberer HEAD-Lauf vorhanden.
- **Zustand-Ledger nicht fortgeschrieben** — `external-state.md` trägt fremde
  uncommittete Hunks; die CI/Postfach-Zeilen wurden gemessen, aber nicht
  committet (Registraturpflicht: beim nächsten sauberen Stand fortschreiben).

## Roter ci-check @`5c61e16e` — `blockiert` (Format) / Test adressiert (härtester undatiert)

Gemessen `ci_manage log 35523145456`: drei Jobs `failure` — **clippy**
(`question_mark` `src/archivar/extract.rs:3425`), **format** (Diffs in
`src/archivar/tests.rs:1479/1557`, `tools/utils/src/bin/archive_search/
biomodels.rs:164/181/194`, `cod.rs:75/99/136/176`, `ena.rs:205`), **test**
(`archivar::tests::hapi_draft_names_register_unit_when_server_unit_is_off_registry`
FAILED @`tests.rs:8384`). Diese Session hat den clippy-Fehler (`(*dist_scale)?`)
und die gemessenen Format-Diffs behoben; der Test ist durch `68a8240f` (bau
folge109, HAPI-Kernel-Erwartung → Registry-Kanon patch-levy) adressiert. Offen:
das HEAD-Verdikt — der Push triggert `ci-check`; **ungeprüft auf Format** bleiben
`materialsproject.rs` (`1889f385`) und `tests.rs` (`68a8240f`), da `cargo fmt`
lokal strukturell verweigert ist. (Schritt: `ci_manage view <neuer ci-check run>`
nach dem Push; bei verbleibendem Format-Diff die Zeile aus `ci_manage log`
übernehmen.)

## Chrome DevTools MCP — Membran-Lauf — `operator-gebunden`

Der Pin `npx -y chrome-devtools-mcp@1.9.0` ist geladen (`chrome-devtools_list_pages`
antwortet). Offen bleibt der Membran-Lauf (`cargo run` → `127.0.0.1:1618`,
Konsole/Netz via CDP am live Chrome) — ein sichtbarer Vordergrund-Lauf. (Schritt:
Operator-Wort für den Lauf; danach CDP-Lesen. `OMEGAFLOW_HIDDEN=1` stillt das
Fenster, nicht den CDP-Bedarf.)

## TE-Gate-Verdikt — `wartend`

`te-gate` `35513982359` @`7d0a1272` in_progress seit 13:36Z, kein Update. Rat
(2026-09-20): **kein Cancel** — der Watchdog hat keine Median-Basis (< 2
successful runs), der Lauf liegt im Mehrstunden-Profil derselben Batterie.
(Schritt: `ci_manage view 35513982359`; success → `ci_manage log 35513982359` und
die vier `fpr_rise_sigma_test`-Gate-Zeilen verifizieren; cancelled →
`gh workflow run te-gate.yml` am HEAD mit dem 3σ-Fix `5b406e16`.)

## Zustand-Ledger fortschreiben — `blockiert`

`docs/zustand/external-state.md` trägt fremde uncommittete Hunks (entscheid-folge64);
die CI-/Postfach-Zeilen (veraltet @`c29234d1`) wurden gemessen, aber nicht
committet. (Schritt: sobald die entscheid-Linie ihren Stand committet hat, die
CI-Status- und Postfach-Zeile fortschreiben — `ci_manage list` +
`state/mail/mail_ledger.φ`.)

## Hardware-Sponsoring — `operator-gebunden`

Drei offene Threads im Postfach: Pine64 (`1789922257`, Verweis auf
`info@pine64.org`), Framework (Ticket `NG2HWBZM`), Tuxedo (Ticket `#991311279`).
Eine Antwort an einen Dritten ist ein consent-pflichtiger Akt → `entscheid`/Operator.
Die Post-Zeile `An entscheid:` konnte nicht gesetzt werden: `post.md` trägt fremde
uncommittete Hunks — nicht angefasst. (Schritt: beim nächsten sauberen `post.md`
die Zeile setzen.)

## Browser-Anbindung — Rest

Survey `docs/surveys/survey-2026-09-20-browser-anbindung.md`.

- **Cookie-Editor-Transfer** Operator-Profil ↔ persistentes Playwright-Profil —
  `operator-gebunden`.
- **Playwright-Browser-Extension** — befund-gated (erst bei gemessenem
  Pfad-1-Versagen).

## Flyby-Path-2-Kette — `termin` (Perigäum 28./29.09.)

Bereitschaft gemessen (Addendum
`docs/paper/flyby-path-2-falsification-metric-addendum.md` §"Chain readiness");
alle Kanalrouten HTTP 200, Transit-Methode benannt, DSCOVR retired → RTSW deckt L1.
Keine Zellwerte vor dem Perigäum — jede Zelle `pending` (0 honored). (Schritt:
Zellen ab 28.09. füllen, je Messwert `source`+`active` protokollieren —
`ernte`/`research-max`.)

## ernte / termin

- **NSE/Haug** — `wartend`: Trigger Dateieingang.
- **BepiColombo MORE** — `termin` (~April 2027).

## Benchmark (dieses Atom)

- **Roter ci-check** — die Diagnose lief in-Session (Routine-Klasse
  „CI-Log-Extraktion", im Register durch `grind-flash` $0.0008 geschlossen); kein
  neuer Benchmark-Kandidat. Der Format-Fix wurde aus den gemessenen CI-Diffs
  übernommen (`edit`), nicht durch einen lokalen `cargo fmt`-Lauf (strukturell
  verweigert).

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/extract.rs` (clippy-Fix),
- `src/archivar/tests.rs` (Format),
- `tools/utils/src/bin/archive_search/{biomodels,cod,ena}.rs` (Format),
- `docs/handover/handover-2026-09-20-forschung-folge118.md` (neu),
- `docs/handover/handover-2026-09-20-forschung-folge117.md` (Move → `archiv/`).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
