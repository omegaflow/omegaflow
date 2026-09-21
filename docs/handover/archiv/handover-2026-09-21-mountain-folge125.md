<!--
  title: Handover — Mountain-Folge 125 (Stand 2026-09-21)
  session: Mountain-Folge 125
  class: handover
  date: 2026-09-21
  sha256: d0488e644b09249b4db1c03ee6ad764254388580288e6b7ec0f7049fc8ef365e
  status: live
-->
# Handover — Mountain-Folge 125 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder offene Punkt wird
**aufgeschlüsselt** geführt — **Lage** (der Zustand, gemessen) / **Blockade**
(woran es hängt, oder „keine") / **Braucht** (was es löst: Werkzeug, Datei, URL,
Anfrage, Operator-Wort). Jeder Punkt trägt seinen Status-Tag (`wartend` |
`operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** `e7f1b041` == `origin/main` beim Start; zwischen den Messungen
  landeten fremde Commits `769b4f46` und `51d06337` (Line-Palette) → HEAD
  `51d06337` == `origin/main`. Der Baum trägt die eigene uncommittete Arbeit.
- **Postfach** — neuester Ledger-Eingang `1789978555` (Brave Search API
  „usage limit reached", informativ); kein handlungsbedürftiger Fall.
- **CI** (Watchdog-Snapshot 15:31Z): aktiv `ci-check 35603922594`,
  `health-check 35596716341`, `hyperscanning-te 35596009980`, `te-gate
  35595896140`; failed `glm-l2-cdn 35599198872/35599180155`, `ci-check
  35599318266/35595635539`, `wds/vsx/rave-cdn`. Kein Poll.
- **`register_lookup --open`** — 601 offen, 0 Post, 14 `zustand` due.
- **`git_safety --snapshot`** — `refs/safety/1789998819`.

## Offen (aufgeschlüsselt)

### 1. HDF5 layout-v3 Chunk-Read — unbestätigter Kandidat
- **Status:** offen | **Bindung:** eigen
- **Lage:** `grind-max` maß am Granule (chunk_dims=[1], 1 B-Tree-Record), der
  chunked Read materialisiere evtl. nur Element 0; der erfolgreiche
  glm-l2-cdn-Lauf (Asset 126728 B) spricht dagegen — **unbestätigt**.
- **Blockade:** unbestätigt; braucht eine Messung der Element-Zahl.
- **Braucht:** `src/archivar/hdf5.rs` chunked-read gegen das Granule messen
  (Element-Zahl nach Read) — Fix oder Entwarnung.

### 2. DS18B20 1-Wire-Firmware-Lesepfad (Safety-Lücke)
- **Status:** offen | **Bindung:** eigen
- **Lage:** `sgrep ds18b20 firmware` leer; der Core bindet GPIO7 nicht.
  Safety-Matrix verlangt Cutoff <80 °C für Heizfolie/Peltier.
- **Blockade:** keine (Bau); 1-Wire-Treiber nötig.
- **Braucht:** 1-Wire-GPIO7-Lesepfad im Core `firmware/radiatorium` plus Cutoff;
  `grind-pro`/`build`.

### 3. Ox64-Zweitknoten — Hardware/Bring-up
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat den Ox64 zugesagt; Doku
  (`docs/specs/mantis-shrimp-build.md §Zweitknoten`) steht; Gerät nicht da.
- **Blockade:** Geräteankunft (Anfrage 2026-09-20 an PINE64).
- **Braucht:** Ankunft abwarten; dann Buildroot/OpenWrt-Bring-up + Kopplung
  messen (die `pending`-Punkte der Doku).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
