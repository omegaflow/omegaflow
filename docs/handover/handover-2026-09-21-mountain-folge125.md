<!--
  title: Handover — Mountain-Folge 125 (Stand 2026-09-21)
  session: Mountain-Folge 125
  class: handover
  date: 2026-09-21
  sha256: 1b3da01d837c93f17463a4be45ed99d3fb017c3136fce75c8382aebad6949d4e
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

### 1. `archive_search`-Änderungen erreichen `tools-latest` nicht
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Quelle geändert (`archive_search.rs`: `--brave`-Help-Zeile +
  `--all`-Wortlaut; `net.rs`: 402-Meldung nennt `--mwmbl`, `--brave` aus
  `QUERY_MODES` entfernt = `--all` ruft Brave nicht mehr; Test angepasst).
  `cargo check -p omegaflow-utils --all-targets` 0/0. Das PATH-Binary
  (`archive_search 67dbd0f3`) ist der Stand **vor** diesem Atom.
- **Blockade:** Commit/Push fehlen; die Manifestation ist ein CI-Job.
- **Braucht:** `/commit` → Push → `gh workflow run tools-build.yml` →
  `bin/.tools_ensure archive_search` (sha-Abgleich) → `archive_search --help`
  misst `--brave`-Zeile + `--all`-Zahl (34).

### 2. HDF5 layout-v3 Chunk-Read — unbestätigter Kandidat
- **Status:** offen | **Bindung:** eigen
- **Lage:** `grind-max` maß am Granule (chunk_dims=[1], 1 B-Tree-Record), der
  chunked Read materialisiere evtl. nur Element 0; der erfolgreiche
  glm-l2-cdn-Lauf (Asset 126728 B) spricht dagegen — **unbestätigt**.
- **Blockade:** unbestätigt; braucht eine Messung der Element-Zahl.
- **Braucht:** `src/archivar/hdf5.rs` chunked-read gegen das Granule messen
  (Element-Zahl nach Read) — Fix oder Entwarnung.

### 3. DS18B20 1-Wire-Firmware-Lesepfad (Safety-Lücke)
- **Status:** offen | **Bindung:** eigen
- **Lage:** `sgrep ds18b20 firmware` leer; der Core bindet GPIO7 nicht.
  Safety-Matrix verlangt Cutoff <80 °C für Heizfolie/Peltier.
- **Blockade:** keine (Bau); 1-Wire-Treiber nötig.
- **Braucht:** 1-Wire-GPIO7-Lesepfad im Core `firmware/radiatorium` plus Cutoff;
  `grind-pro`/`build`.

### 4. Ox64-Zweitknoten — Hardware/Bring-up
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat den Ox64 zugesagt; Doku
  (`docs/specs/mantis-shrimp-build.md §Zweitknoten`) steht; Gerät nicht da.
- **Blockade:** Geräteankunft (Anfrage 2026-09-20 an PINE64).
- **Braucht:** Ankunft abwarten; dann Buildroot/OpenWrt-Bring-up + Kopplung
  messen (die `pending`-Punkte der Doku).

### 5. Service-Toolset (`smail` & Co.) — keine Frische-Kette
- **Status:** offen | **Bindung:** eigen
- **Lage:** `~/.local/bin/smail` (und `smail_recv`, `mail_watchdog`,
  `mail_digest`, `cds_watchdog`, `job_dashboard`, `job_monitor`,
  `notes_notify`) sind **bare Binär-Kopien** (kein Wrapper, kein Symlink, kein
  Manifest); `target/release/smail` 09-17 16:02 < Quelle `smail.rs` 09-21 08:45
  → stale. `service-build.yml` ist `workflow_dispatch`-only und lädt nur
  Artifacts hoch, kein Release.
- **Blockade:** kein `service-latest`-Release; `.tools_ensure` kennt die
  Service-Bins nicht.
- **Braucht:** Entscheid + Bau — `smail`/`smail_recv` (und ggf. die übrigen
  Service-Bins) in `tools-build.yml` aufnehmen (Manifest + Upload) **oder** ein
  `service-latest`-Release mit eigenem Wrapper-Muster; danach ein `smail`-
  Wrapper + ein `~/.local/bin/smail`-Symlink statt Kopie.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
