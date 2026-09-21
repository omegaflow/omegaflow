<!--
  title: Handover — River-Folge 3 (Stand 2026-09-21)
  session: River-Folge 3
  class: handover
  date: 2026-09-21
  sha256: 54714a4c7f005870a250551a05c43b5ed8efabc1f67583147db648fa4cbde366
  status: live
-->
# Handover — River-Folge 3 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, River-Folge 3)

- **HEAD** Start `690dd2e3`; während der Session zog `origin/main` zweimal per
  Fast-Forward: `e7ff26b4` (future folge85) → `0b1a7d03` (forschung folge140) —
  HEAD == `origin/main` beim Abschluss.
- **Postfach** — `state/mail/mail_ledger.φ` unverändert, letzte Zeile
  `1789978555` (Brave-Limit); keine eingehende Zeile an River in `post.md`;
  Zustand-Eintrag (Postfach extern) nicht fällig.
- **CI** — `ci-check` `35608204623` @`8d6553fe`: Jobs `test` (7 rot), `clippy`
  (rot), `format` (rot), `dropped-gate`/`build` success. River-Anteile
  abgearbeitet (s.u.), fremde Anteile registriert. Kein Poll.
- **`register_lookup --open`** — kein `owner=river`.
- **`git_safety --snapshot`** — `refs/safety/1790008869`.
- **`open_points_check`** — folge2: 12 Pfad-Refs, 0 absent.

## Erledigt in diesem Atom (git trägt)

Vier rote River-lib-Tests aus `ci-check` `35608204623` @`8d6553fe` gefixt, dazu
drei River-eigene Clippy-Lints (Rust 1.98, `-D warnings`):

- `src/archivar/quaoar_occlt.rs` — `find_eocd` nimmt die EOCD-Signatur nur bei
  vollständigem 22-B-Record an (abgeschnittenes ZIP wird abgewiesen);
  Test-Erwartung `date_midnight_unix("20110211")` von 15015 auf **15016**
  korrigiert (gemessen: 2011-02-11 = Unix 1297382400 = 15016 Tage,
  `days_from_civil` = 15016; `ymd_to_days` liefert korrekt 15016).
- `src/archivar/fai_kz.rs` — `parse_obscore_csv` gibt bei vorhandenen Datenzeilen
  mit ausschließlich void-Position `Some((vec![], counts))` (0-Kanon:
  `position_void` bleibt sichtbar), `None` nur bei `counts.rows == 0`; `valid`
  auf `is_some_and` (collapsible_if).
- `src/archivar/hfrnet_rtv.rs` — `lon`-Assertion auf Toleranz (f64 1-ULP:
  `242.75774 - 360.0 ≠ -117.24226` exakt).
- `src/archivar/ia2_tap.rs` — `\0` → `\x00` in Test-Literalen (octal_escapes);
  f32-Literale lossless gekürzt (excessive_precision).
- `src/archivar/babamul.rs` — f32-Literale lossless gekürzt.

## Offen (aufgeschlüsselt)

### 1. Tree-weiter rustfmt-Drift (`format`-Job)
- **Status:** operator-gebunden | **Bindung:** operator (Rat/CI-Config)
- **Lage:** `ci-check` `35608204623` Job `format` rot über ~40 Dateien **aller**
  Linien (`src/archivar/*`, `tools/harvest/src/bin/*`, `tools/measure/src/bin/*`,
  `tools/register/src/bin/*`); CI fährt `actions-rust-lang/setup-rust-toolchain@v1`
  = Rust **1.98** (clippy-Help-URL), kein `rust-toolchain.toml` pinnt eine
  Version; die committete Formatierung stammt von einer älteren rustfmt —
  gemessen an den Diffs (`quaoar_occlt.rs:92` wird gejoint, `:106` gebrochen).
- **Blockade:** tree-weiter Eingriff berührt fremde Linien; kein River-Punkt.
- **Braucht:** Operator-/Rat-Wort — (a) Toolchain pinnen (`rust-toolchain.toml`)
  ODER (b) ein tree-weiter `cargo fmt`-Commit; Post `An future` steht.

### 2. clippy `needless_range_loop` `src/mathematikerin/te.rs:1574`
- **Status:** blockiert | **Bindung:** linie:future
- **Lage:** `ci-check` `35608204623` Job `clippy` (Rust 1.98, `-D warnings`)
  meldet `te.rs:1574`; River-fremd.
- **Blockade:** fremde Datei.
- **Braucht:** future behebt am Baum; Post `An future` steht.

### 3. vC-Permeabilität — Vollzug (hidden run)
- **Status:** termin | **Bindung:** operator (Maschine)
- **Lage:** `future-folge83:182-186`; versteckter sensor-getriebener Lauf wartet.
- **Blockade:** Operator-Maschine.
- **Braucht:** Wort für `OMEGAFLOW_HIDDEN=1`.

### 4. Browser-Brücke — Chrome DevTools MCP pinnen
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** `survey-2026-09-20-browser-anbindung.md:141-144`; Drei-Pfade-Tabelle
  steht (`tools-map.md:259-268`, `80294dcf9`).
- **Blockade:** Debugger-Rechte am live Chrome.
- **Braucht:** Operator-Wort; dann npm `1.9.0` + Telemetrie-Flags.

### 5. Verhaltenshälfte „halten-vor-reichen" der Linien-Umbenennung
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** `future-folge83:252-256`; Namenshälfte gebaut (`8d6553fe`), Regel in
  keiner Datei (`sgrep` leer).
- **Blockade:** Architektur-Wort.
- **Braucht:** Wort → Rat/Atom.

## Benchmark

- Zwei `grind-flash`-Delegationen (Test-Fixes, Clippy-Fixes). Die Klasse
  „Routine-Parser/Fix" hat einen registrierten Sieger (`grind-flash`); kein
  Doppel-Lauf, kein neuer Sieger.

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/quaoar_occlt.rs`, `src/archivar/fai_kz.rs`,
  `src/archivar/hfrnet_rtv.rs`, `src/archivar/ia2_tap.rs`,
  `src/archivar/babamul.rs`
- `docs/handover/handover-2026-09-21-river-folge3.md` (neu)
- `docs/handover/archiv/handover-2026-09-21-river-folge2.md` (verschoben)
- `docs/handover/post.md` (eigene `An river`-Zeile gelöscht, zwei `An future`-
  Zeilen ergänzt)

Fremd, **nicht angetastet:** die Riss-4-CPU-Verdrahtung
(`src/mathematikerin/omega.rs`, `src/mathematikerin/tests.rs`) und die
CI-Status-Zeile in `docs/zustand/external-state.md` lagen während der Session
uncommittet im Baum und wurden von der Forschung-Linie in `0b1a7d03` (forschung
folge140) committet — River hat sie nie berührt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
