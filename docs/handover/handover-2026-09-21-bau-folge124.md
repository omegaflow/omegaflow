<!--
  title: Handover — Bau-Folge 124 (Stand 2026-09-21)
  session: Bau-Folge 124
  class: handover
  date: 2026-09-21
  sha256: da01356056b1106781096a524d75f793010565c81fcbac8a9f90d5913027757a
  status: live
-->
# Handover — Bau-Folge 124 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder offene Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** `096ed904` (entscheid folge82); `origin/main` `308c5bd1` → HEAD 1
  Commit voraus (fremd). Baum trug fremde Hunks in `phi/pipeline/ledger.φ` und
  `phi/sources.φ` (emodnet-hfr-Manifestation, ernte) — nicht angefasst.
- **Postfach** — `post.md` trägt fremde Zeilen (2 an ernte: SuperDARN, DEMETER;
  1 an ernte: CI glm-l2); kein neuer Ledger-Eingang. `mail_ledger.φ` Altbestand.
- **CI** — bau-relevant: `35599198872` glm-l2-cdn **failure** (0 flashes,
  Energy-Gate), `35599194982` tools-build **success**. Kein Poll.
- **Safety-Net** — `git_safety --snapshot` `refs/safety/1789993431`.

## Messung dieses Atoms

- **Punkt 2 — Mantis-Shrimp Bau-Doku + Ox64** (`grind-flash`): neue
  `docs/specs/mantis-shrimp-build.md` (minimaler CORE-BUILT-Aufbau, Pin-Map,
  Firmware-Build/Flash, Safety-Auszug, BOM-Teilsatz, `## Zweitknoten — PINE64
  Ox64` mit `pending`-Punkten). sha256 `d098ee2a…`.
- **BOM-Header re-gestempelt** — `mantis-shrimp-bom.md` Header-hash war stale
  (`56efc939…` ≠ Body `cbf79a9a…`), jetzt synchron.
- **Punkt 3 — `text_review` erreicht `tools-latest`**: `tools-build`
  `35599194982` success; `bin/.tools_ensure text_review` exit 0 (sha gegen
  `tools.manifest`).
- **Punkt 1 — glm_l2 Energy-Gate, Wurzel gemessen** (`grind-max`), am echten
  Granule byte-genau: `flash_energy` int16 LE, `_Unsigned` = **String "true"**
  (nicht int 1), `valid_range {0,-6}` (signed = 0…65530 unsigned), `scale 1e-15`,
  unit J, Rohwerte 80…1615. `attr_unsigned` las nur die int-Form → `_Unsigned`
  verpasst → `valid_range` signiert → jeder Rohwert verworfen → 0 flashes. Fix:
  `attr_unsigned` liest auch `_Unsigned="true"`; +1 Gate-Test.
  `cargo check -p omegaflow-harvest --all-targets` 0/0. Der Fix ging in
  `db708714` (ernte folge133) ein — die ernte-Session committete die
  uncommittete Arbeit mit; nicht zurückgerollt.

## Offen (aufgeschlüsselt)

### 1. glm_l2.bin CDN-Manifestation — Fix gebaut, Re-Dispatch misst
- **Status:** `wartend` | **Bindung:** `termin:CI`
- **Lage:** Energy-Gate-Wurzel gemessen + Fix in HEAD (`db708714`, ernte
  folge133); Lauf `35599198872` failure.
- **Blockade:** Re-Dispatch erst nach Commit+Push (der Lauf checkt main aus).
- **Braucht:** nach `/commit`+Push `gh workflow run glm-l2-cdn`; Log einmal lesen
  (`ci_manage view <id>`) — N flashes + Gate-Zähler je Granule; bei Asset sha256
  in `phi/sources.φ`.

### 2. HDF5 layout-v3 Chunk-Read — gemessener Kandidat (core)
- **Status:** `offen` | **Bindung:** `eigen`
- **Lage:** `grind-max` maß am Granule (chunk_dims=[1], 1 B-Tree-Record), dass
  der chunked Read evtl. nur Element 0 materialisiert — dann bliebe der Harvest
  bei ~1 Flash/Granule trotz Gate-Fix. Widerspricht der gemessenen Chunk-Nutzlast
  (6 int16 im Chunk) — **unbestätigt**.
- **Blockade:** unbestätigt; braucht eigene Messung.
- **Braucht:** Re-Dispatch aus Punkt 1 entscheidet (energy-Skip ~5 bei N=1 wäre
  das Signal); dann `src/archivar/hdf5.rs` chunked-read gegen das Granule messen
  und Fix oder Entwarnung.

### 3. DS18B20 1-Wire-Firmware-Lesepfad (Safety-Lücke)
- **Status:** `offen` | **Bindung:** `eigen`
- **Lage:** `sgrep ds18b20 firmware` leer; der Core bindet GPIO7 nicht.
  Safety-Matrix verlangt Cutoff <80 °C für Heizfolie/Peltier.
- **Blockade:** keine (Bau); 1-Wire-Treiber nötig.
- **Braucht:** 1-Wire-GPIO7-Lesepfad im Core `firmware/radiatorium` plus Cutoff;
  `grind-pro`/`build`.

### 4. Ox64-Zweitknoten — Hardware/Bring-up
- **Status:** `wartend` | **Bindung:** `dritter`
- **Lage:** PINE64 hat den Ox64 zugesagt; Doku (`mantis-shrimp-build.md
  §Zweitknoten`) steht; Gerät noch nicht da.
- **Blockade:** Geräteankunft (Anfrage 2026-09-20 an PINE64).
- **Braucht:** Ankunft abwarten; dann Buildroot/OpenWrt-Bring-up + Kopplung
  messen (die `pending`-Punkte der Doku).

## Benchmark

- **Bau-Folge 124:** Punkt 2 (Doku) `grind-flash` — vollständig, kein pro/max-
  Doubling. Punkt 1 (Parser-Wurzel) `grind-max` — hard atom; der erste Lauf kam
  **leer** zurück, der Resume lieferte die Byte-Messung (kein Doppel-Lauf gegen
  einen anderen Vertreter, sondern Resume). Punkt 3 im `build`-Kontext.

## Geteilter Baum — eigener Pfad-Satz

- **Diese Session:** `docs/specs/mantis-shrimp-build.md` (neu),
  `docs/specs/mantis-shrimp-bom.md` (Header-Re-Stamp), neues
  `docs/handover/handover-2026-09-21-bau-folge124.md`, Move
  `handover-2026-09-21-bau-folge123.md` → `archiv/`.
- **Bereits committet (fremde Nachricht):** `tools/harvest/src/bin/glm_l2_compiler.rs`
  (der Fix aus diesem Atom) ging in `db708714` (ernte folge133) ein — die
  ernte-Session committete die uncommittete Arbeit mit; nicht zurückgerollt.
- **Fremd (nicht angefasst):** `phi/pipeline/ledger.φ`, `phi/sources.φ`
  (emodnet-hfr, ernte), `docs/handover/post.md` (fremd-dirty durch
  konkurrierende Session — nicht editiert, kein fremder Hunk mitcommittet),
  `docs/handover/handover-2026-09-21-ernte-folge133.md`, der staged Rename
  `handover-2026-09-21-ernte-folge132.md` → `archiv/`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
