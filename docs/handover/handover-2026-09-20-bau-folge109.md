<!--
  title: Handover — Bau-Folge 109 (Stand 2026-09-20)
  session: Bau-Folge 109
  class: handover
  date: 2026-09-20
  sha256: 7978110d352acf1b563451c15e09b4d762ab708b29040f614dfaec291f65f816
  status: live
-->
# Handover — Bau-Folge 109 (2026-09-20)

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
und werden nie als Handlungsschritt geführt. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-20, Session-Beginn)

- **HEAD** `8172288e` (ernte folge118) **== `origin/main`**; die Basis bewegte
  sich während der Session (Start `66579fc3` „bau folge108", dann Push ernte
  folge118). Arbeitsbaum: fremd entscheid folge64 — `docs/handover/post.md` `M`,
  `docs/zustand/external-state.md` `M`, `handover-2026-09-20-entscheid-folge64.md`
  `??`, Move `handover-…-entscheid-folge63.md` → `archiv/` (staged); **nicht
  angefasst**. Snapshot `refs/safety/1789924698`.
- **Postfach** — `post.md` trug die eine Zeile `An entscheid: Queue-Korpora
  Re-Lauf` (entscheid faltet sie in seinem uncommitteten Hunk); keine
  `mail_ledger.φ`-Zeile > `1789918147`; kein bau-Eingang.
- **CI** — `ci-check`-Kette unruhig: aktiv `35523145456` @`5c61e16e` (Vorfahr);
  rot (attempt 1) `35520538766` @`fa44f315` (**main**), `35517957987`
  @`v2026-09-20` (Release-Branch); viele `ci-check` `cancelled` (Watchdog). Der
  Rot-Grund ist gemessen: Job `test`,
  `archivar::tests::hapi_draft_names_register_unit_when_server_unit_is_off_registry`
  erwartete `gaussian-inverse-square`, der Registry-Kanon liefert `patch-levy`.
- **Zustand-Ledger** — die `CI-Status`-Zeile in `external-state.md` ist fällig
  (HEAD-Wechsel seit `c29234d1`), liegt aber im fremden entscheid-Hunk und bleibt
  darum unangetastet; der Wert steht als Snapshot oben.

## Offen

- **NIST WebBook `parser-def` (bau)** — `phi/blocked_sources.φ:55`; `cbook.cgi`
  liefert nur `text/html` (kein JSON/XML/TSV), JCAMP/InChI/MOL nur per bekannter
  ID; kein offizielles API. (Schritt: `docs/SOURCE_PORT.md` — Force-Gate/Disposition
  der Query-Route oder HTML-Arm in `src/archivar/extract.rs`.) · `blockiert`
- **Queue-Korpora Re-Lauf** — die 7 `parser-gap`-Korpora (`phi/pipeline/ledger.φ`)
  brauchen den lokalen Release-Binär-Lauf (`--port` + `--probe`) nach dem
  `port.rs`-Fix; die `pos`-ohne-`body`-Blöcke hängen am selben Lauf-Ort. An
  `entscheid` geroutet (Operator-Queue #5). (Schritt: Operator-Wort für den
  Lauf-Ort, dann `--port` + `--probe`, dann die Ledger-Notes fortschreiben.) ·
  `operator-gebunden`
- **`ci-check`-Verdikt am Folge-109-HEAD** — (Schritt: nach Push den Run einmalig
  aus `/tmp/opencode/ci_status.md` lesen; bei Rot `ci_manage log <id>`.) · `wartend`

## Benchmark

- **Bau-Folge 109**: Atom „die zwei Cross-Line-Lücken schließen, die den
  Planungs-Pass und `ci-check` blockieren" — (a) `bin/.tools_ensure`-Gate
  content-addressed, (b) HAPI-Draft-Kernel-Erwartung auf den Registry-Kanon. Die
  Klasse „Routine-Messung/Diagnose" hat ihren flash-Sieger (gemessen 2026-09-16,
  8 Profile, `grind-flash` $0.0008 gegen pro/max $0.0041–0.0090 bei identischem
  Ergebnis); kein Doppellauf. Der `.tools_ensure`-Fix ist Shell-Logik, verifiziert
  mit einem 4-Fall-Harness (match/mismatch/manifest-absent/realer Refresh) plus
  `sh -n`; `cargo check` + `cargo check --features browser_relay` 0 Fehler / 0
  Warnungen.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `bin/.tools_ensure` (content-addressed Gate),
  `src/archivar/tests.rs` (HAPI-Kernel-Erwartung `patch-levy`), neues
  `docs/handover/handover-2026-09-20-bau-folge109.md`, Move
  `handover-2026-09-20-bau-folge108.md` → `archiv/`.
- **Fremd (nicht anfassen):** `docs/handover/post.md`,
  `docs/zustand/external-state.md`, `docs/handover/handover-2026-09-20-entscheid-folge64.md`,
  der `folge63`-Move. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
