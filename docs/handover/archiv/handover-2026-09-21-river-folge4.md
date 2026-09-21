<!--
  title: Handover — River-Folge 4 (Stand 2026-09-21)
  session: River-Folge 4
  class: handover
  date: 2026-09-21
  sha256: 84358291521d5278958345c53ac711a405c5b01281be0ef7e8aa0caebd3c59d6
  status: live
-->
# Handover — River-Folge 4 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, River-Folge 4)

- **HEAD** Start `3e319087` == `origin/main` == `merge-base`, Arbeitsbaum leer;
  während der Session zog `origin/main` per Fast-Forward auf `b3ea90a4` (sensory
  folge141, fremde Pfade) — HEAD == `origin/main` beim Abschluss.
- **Postfach** — `state/mail/mail_ledger.φ` gelesen, keine eingehende Zeile an
  River; `post.md` trug zwei `An future` (rustfmt-Drift, `te.rs:1574`) + ein
  `An mountain` (Riss 4, fremd).
- **`git_safety --snapshot`** — Arbeitsbaum == HEAD, nichts zu sichern.
- **`register_lookup --open`** — kein `owner=river`.
- **`open_points_check`** — folge3: 22 Pfad-Refs, 0 absent.
- **CI** — `ci_manage list`: 42 `ci-check`-Läufe im sichtbaren Fenster (letzte
  100 Runs), **0 success**, 35 cancelled, 5 failure; Watchdog-Snapshot nennt
  `35625268875` aktiv. Der `format`-Job wurde am Fehllog `35625268875` gemessen:
  CI fährt `stable` = **rustc 1.98.1** (identisch zur lokalen Toolchain), 70
  rustfmt-Diffs über ~40 Dateien. Kein Poll.
- **Browser-Brücke** — `chrome-devtools_list_pages` antwortet (`about:blank`) →
  der MCP-Pin lebt; der MCP führt einen **eigenen** Browser, keinen CDP zum
  Operator-Chrome (`--remote-debugging-port` nicht gesetzt).

## Descoped mit Befund

- **„Live-Chrome-Anbindung" descoped:** der MCP bedient einen eigenen Browser
  (`about:blank`, gemessen); Konsolen-/Netz-/Performance-Debug via CDP steht dort.
  Die Anbindung an den Operator-Profil-Chrome bräuchte zusätzlich Chrome mit
  `--remote-debugging-port=9222` plus `--browserUrl` am MCP — eine benannte
  Option, kein Erfordernis der Membran.

## Offen (aufgeschlüsselt)

### 1. clippy `needless_range_loop` `src/mathematikerin/te.rs:1574`
- **Status:** blockiert | **Bindung:** `linie:future`
- **Lage:** `ci-check` `35608204623` Job `clippy` (Rust 1.98, `-D warnings`)
  meldet `te.rs:1574`; River-fremd.
- **Blockade:** fremde Datei.
- **Braucht:** future behebt am Baum; Post `An future` steht.

### 2. Gate-Fixture „halten-vor-reichen" (Verhaltenshälfte)
- **Status:** eigen | **Bindung:** eigen
- **Lage:** die Regel steht in `AGENTS.md:184`; das gate-bare Residuum — ein
  Routing-Tag (`An <line>:` / `linie:<x>`) gepaart mit einem Schritt, dessen
  Owner-Vokabular (parser-def → mountain, account/key → future …) die
  Sender-Natur ist — braucht die Owner-Vokabular-Karte und die Sender-Ableitung
  aus dem Pfad; nicht soundly als reine Regex (der Rat selbst: „nicht alles ist
  eine Regex").
- **Blockade:** keine.
- **Braucht:** Bau-Atom — `check_line_routing`-Muster erweitern (`commit_gate_vocab.json`
  + Test), sobald die Owner-Karte gate-lesbar ist.

### 3. Linien-Umbenennung — Namenshälfte
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** Rat-Blatt 2026-09-21 empfiehlt reine Stimmen-Namen; Namenshälfte
  gebaut (`8d6553fe`), Verhaltenshälfte nun in `AGENTS.md:184`; das lebende
  Routing-Vokabular ist noch funktional benannt.
- **Blockade:** Architektur-Wort.
- **Braucht:** Wort „umbenennen" → Atom (Slugs, `post.md`-Adressen, `linie:`-Tags,
  Palette, Tafel-Köpfe; Gate-Fixture im selben Commit).

### 4. vC-Permeabilität — Vollzug (hidden run)
- **Status:** blockiert | **Bindung:** operator (Maschine)
- **Lage:** `future-folge83:182-186`; versteckter sensor-getriebener Lauf wartet;
  **keine Sensoren angeschlossen** (Operator-Wort 2026-09-21).
- **Blockade:** Sensoren fehlen.
- **Braucht:** Sensoren angeschlossen (Trigger), dann `OMEGAFLOW_HIDDEN=1`.

### 5. format-Drift — Recurrence-Wurzel
- **Status:** operator-gebunden | **Bindung:** operator (Architektur)
- **Lage:** lokales `cargo fmt` ist Regel-verboten, jede Session schreibt Code
  ohne Format-Lauf → Drift sammelt sich bis zum `fmt:`-Heil-Commit (jetzt: 70
  Diffs, `rustc 1.98.1`). Ein Toolchain-Pin behebt das nicht (kein Versionsdrift).
- **Blockade:** Architektur-Wort.
- **Braucht:** Wort — CI-Auto-Format-Job (`cargo fmt` + Commit) **oder** lokales
  `cargo fmt` als erlaubter Syntax-Schritt.

## Geteilter Baum — eigener Pfad-Satz

- `AGENTS.md` (eigene Regel-Zeile in `:184`)
- `docs/handover/handover-2026-09-21-river-folge4.md` (neu)
- `docs/handover/archiv/handover-2026-09-21-river-folge3.md` (verschoben)
- `docs/handover/post.md` (eigene `An future`-rustfmt-Zeile gelöscht)
- fmt-Heil (eigener Dienst-Commit, Baum war beim Start leer):
  `src/archivar/{babamul,hdf5,hfrnet_rtv,ia2_tap,port,quaoar_occlt,tests}.rs`,
  `src/mathematikerin/te.rs`,
  `tools/harvest/src/bin/{babamul,fai_kz,glm_l2,gong_series,ia2_tap,quaoar_occlt,radnet,rpw,tess,tycho2}_compiler.rs`,
  `tools/measure/src/bin/{free_model_bench,hyperscanning_group_te,text_review,topocentric_coupling_probe,weberin_body_verdict,weberin_verdicts_compiler}.rs`,
  `tools/register/src/bin/{doc_audit,open_points_check,register_lookup}.rs`

Fremd, **nicht angetastet:** die `An mountain`-Zeile in `post.md` (fremde Riss-4-
Zeile) und alle nicht genannten Pfade.

## Benchmark

- Drei `grind-flash`-Delegationen (fmt-Heil, 70 Diffs). Klasse
  „Routine-Fix/Handanwendung" hat einen registrierten Sieger (`grind-flash`);
  kein Doppel-Lauf. Zusätzlich eine `council`-Sitzung (Architektur, pro/max) —
  ihr Pin-Verdikt wurde durch die Messung (CI = lokal = 1.98.1, unformatierter
  Code) widerlegt; die Arbeitsschicht lieferte den Konfund, den der Rat nicht sah.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
