<!--
  title: Handover — Bau-Folge 115 (Stand 2026-09-21)
  session: Bau-Folge 115
  class: handover
  date: 2026-09-21
  sha256: 8935e42d95c39f60bb79c9bf393a84b1354b42535ef603f03c49b7460a58574d
  status: live
-->
# Handover — Bau-Folge 115 (2026-09-21)

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
Der Planungs-Pass nennt die offenen Punkte als Tafel (Punkt | Status | Bindung |
Schritt); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** Session-Beginn `0fd1c5a9` == `origin/main`; Arbeitsbaum sauber
  (`git status` leer); `git_safety --snapshot`: Baum == HEAD, nichts zu sichern.
- **Postfach** — `An bau`-Zeile (force-gate-B-Flush-Lücke, ernte folge125) in
  diesem Atom gefaltet und aus `post.md` gelöscht; `An entscheid` (Ksg off-path)
  steht (fremd). `external-state.md`-Postfachzeile unverändert (letzter Eingang
  `1789930255`, entscheid folge69).
- **CI** — `ci_manage list`/`view`/`log`: `ci-check` `35566258372` **in_progress**
  @`0fd1c5a9` (der für den fmt-Punkt fällige Lauf); `ci-check` `35537130867`
  **failure** @`5894b345` (clippy `te.rs:1574` + 2 `te.rs`-Tests — forschung);
  `rpw-cdn` `35536016815` **failure** @`ff433f9d` (Compiler sauber, Upload HTTP 422
  = 1000-Asset-Cap); `hyperscanning-te` `35537110267` failure; `health-check`
  `35543181033` in_progress.

## Offen

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| fmt-Rot eigener Hunk `src/archivar/tests.rs` | `wartend` | `termin` (ci-check `35566258372`) | `ci_manage view 35566258372` / `ci_manage log 35566258372`; nennt er `tests.rs`, nur den eigenen Hunk formatieren (kein lokaler fmt-Lauf — CI-only), dann `gh workflow run ci-check.yml`. |
| „strukturierte Feld-Grammatik" (Kanon-Akt) | `operator-gebunden` | `operator` (via entscheid) | `An entscheid:`-Zeile steht in `post.md`; entscheid legt die Frage beim Operator-Rückkehr vor. |
| 3 `ausstehend` Queue-Korpora (`ledger.φ:58–66`) | `blockiert` | `linie:ernte` | force-gate-B-Fix steht (`port.rs` `field_or_review`); Re-Lauf auf dem nächsten frischen Binär, `# pending`-Review-Zeilen zählen, dann Disposition SOURCE_PORT §5.4. |
| 7 `verifiziert` Korpora / 368 Survivor (`ledger.φ:70–96`) | `blockiert` | `linie:ernte` | Survivor-Review/Disposition nach SOURCE_PORT §5.4. |

## Messung dieses Atoms (kein Punkt)

- **`port_mode`-Flush-Lücke geschlossen** (Post von ernte folge125): `flush_port_block`
  (`src/archivar/port.rs`) reicht einen Block, dessen Konversion nur der
  `# pending … review`-Marker ist, jetzt durch (`pending`-Zähler; `srcs`-Bindung
  statt `parse_sources(&conv).is_empty()`); `port_mode` berichtet `… pending review`.
  Gate-Tests in `src/archivar/tests.rs` (`test_flush_port_block_carries_pending_review_marker`,
  `test_flush_port_block_drops_a_block_with_no_recognized_content`); Gate-Fixture
  `parse_sources(&conv).is_empty()` in `src/gate/commit_gate_vocab.json` (der
  stille Verlust ist das Fixture). `cargo check --all-targets` 0/0.
- **CDN-1000-Asset-Cap gemessen + registriert:** `rpw-cdn` `35536016815` Upload
  HTTP 422; Release `ssd.jpl.nasa.gov` (id 367063539) am Cap (`Link rel=last
  page=1000`) — blockiert **jeden** `upload_asset` (`src/archivar/cdn.rs:40`).
  Rat (2026-09-21): Route **B** (family-tag), Rotation als gemessener Punkt an
  `ernte` gepostet; `external-state.md`-Zeile `GitHub-Release-Asset-Cap` steht.
- **Postfach:** `An bau` gefaltet/entfernt; `An ernte` (Rotation) geschrieben.

## Benchmark

- **Bau-Folge 115**: 1 `grind-flash` (422-Messung: Release-Tag, Asset-Zahl,
  Change-Points — vollständig) + 1 `council` (CDN-Rotation-Entscheidung,
  Architektur-Atom → pro/max, kein flash-Vergleich nötig). Der Flush-Fix lief
  direkt im `build`-Kontext (klein).

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/archivar/port.rs`, `src/archivar/tests.rs`,
  `src/gate/commit_gate_vocab.json`, `docs/handover/post.md`,
  `docs/zustand/external-state.md`, neues
  `docs/handover/handover-2026-09-21-bau-folge115.md`, Move
  `handover-2026-09-20-bau-folge114.md` → `archiv/`.
- **Fremd (nicht anfassen):** nichts uncommitted im Baum gemessen; `post.md`-Zeile
  `An entscheid` (Ksg) bleibt. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
